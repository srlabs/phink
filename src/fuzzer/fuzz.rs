use crate::{
    cli::{
        config::{
            PFiles::{
                CorpusPath,
                CoverageTracePath,
                DictPath,
            },
            PhinkFiles,
        },
        ziggy::ZiggyConfig,
    },
    contract::{
        payload::PayloadCrafter,
        remote::{
            ContractSetup,
            FullContractResponse,
        },
        selector::Selector,
    },
    cover::coverage::InputCoverage,
    fuzzer::{
        bug::BugManager,
        engine::{
            pretty_print,
            timestamp,
        },
        fuzz::FuzzingMode::{
            ExecuteOneInput,
            Fuzz,
        },
        parser::{
            parse_input,
            Message,
            OneInput,
        },
    },
};
use anyhow::Context;
use contract_transcode::ContractMessageTranscoder;
use frame_support::__private::BasicExternalities;
use sp_core::hexdisplay::AsBytesRef;
use std::{
    env,
    fs,
    io::{
        self,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
    sync::Mutex,
};

pub const MAX_MESSAGES_PER_EXEC: usize = 1; // One execution contains maximum 4 messages.

pub enum FuzzingMode {
    ExecuteOneInput(PathBuf),
    Fuzz,
}

#[derive(Clone)]
pub struct Fuzzer {
    pub ziggy_config: ZiggyConfig,
    setup: ContractSetup,
}

impl Fuzzer {
    pub fn new(ziggy_config: ZiggyConfig) -> anyhow::Result<Self> {
        Ok(Self {
            ziggy_config: ziggy_config.to_owned(),
            setup: ContractSetup::initialize_wasm(ziggy_config)?,
        })
    }

    pub fn execute_harness(self, mode: FuzzingMode) -> anyhow::Result<()> {
        let config = &self.ziggy_config;

        match mode {
            Fuzz => {
                self.fuzz()?;
            }
            ExecuteOneInput(seed_path) => {
                if config.config.verbose {
                    let args: Vec<String> = env::args().collect();
                    let full_command = args.join(" ");
                    println!("Full command: {}", full_command);
                }

                let covpath =
                    PhinkFiles::new(config.config.fuzz_output.clone().unwrap_or_default())
                        .path(CoverageTracePath);
                // We also reset the cov map, doesn't matter if it fails
                let _ = fs::remove_file(covpath);
                self.exec_seed(seed_path)?;
            }
        }

        Ok(())
    }

    fn build_corpus_and_dict(self, selectors: &[Selector]) -> io::Result<()> {
        let phink_file = PhinkFiles::new(self.ziggy_config.config.fuzz_output.unwrap_or_default());

        fs::create_dir_all(phink_file.path(CorpusPath))?;
        let mut dict_file = fs::File::create(phink_file.path(DictPath))?;

        write_dict_header(&mut dict_file)?;

        for (i, selector) in selectors.iter().enumerate() {
            write_corpus_file(i, selector, phink_file.path(CorpusPath))?;
            write_dict_entry(&mut dict_file, selector).unwrap();
        }

        Ok(())
    }

    fn should_stop_now(bug_manager: &BugManager, messages: Vec<Message>) -> bool {
        messages.is_empty()
            || messages.iter().any(|payload| {
                payload
                    .payload
                    .get(..4)
                    .and_then(|slice| Selector::try_from(slice).ok())
                    .map_or(false, |slice: Selector| {
                        bug_manager.contains_selector(slice)
                    })
            })
    }

    fn fuzz(self) -> anyhow::Result<()> {
        let (mut transcoder_loader, invariant_manager) = self.clone().init_fuzzer()?;

        ziggy::fuzz!(|data: &[u8]| {
            Self::harness(
                self.clone(),
                &mut transcoder_loader,
                &mut invariant_manager.clone(),
                data,
            );
        });
        Ok(())
    }

    fn init_fuzzer(self) -> anyhow::Result<(Mutex<ContractMessageTranscoder>, BugManager)> {
        let contract_bridge = self.setup.clone();

        let transcoder_loader = Mutex::new(ContractMessageTranscoder::load(Path::new(
            &contract_bridge.path_to_specs,
        ))?);

        let invariants = PayloadCrafter::extract_invariants(&contract_bridge.json_specs)
            .expect("🙅 No invariants found, check your contract");

        let selectors_without_invariants: Vec<Selector> =
            PayloadCrafter::extract_all(self.ziggy_config.contract_path.to_owned())?
                .into_iter()
                .filter(|s| !invariants.contains(s))
                .collect();

        let invariant_manager = BugManager::new(
            invariants,
            contract_bridge.clone(),
            self.ziggy_config.config.to_owned(),
        );

        self.build_corpus_and_dict(&selectors_without_invariants)
            .expect("🙅 Failed to create initial corpus");

        println!(
            "\n🚀  Now fuzzing `{}` ({})!\n",
            &contract_bridge.path_to_specs.as_os_str().to_str().unwrap(),
            &contract_bridge.contract_address
        );

        Ok((transcoder_loader, invariant_manager))
    }

    fn execute_messages(
        self,
        decoded_msgs: &OneInput,
        chain: &mut BasicExternalities,
        coverage: &mut InputCoverage,
    ) -> Vec<FullContractResponse> {
        let mut all_msg_responses = Vec::new();

        chain.execute_with(|| {
            for message in &decoded_msgs.messages {
                let transfer_value = if message.is_payable {
                    message.value_token
                } else {
                    0
                };

                let result: FullContractResponse = self.setup.clone().call(
                    &message.payload,
                    message.origin.into(),
                    transfer_value,
                    self.ziggy_config.config.clone(),
                );

                coverage.add_cov(&result.debug_message);
                all_msg_responses.push(result);
            }
        });

        all_msg_responses
    }

    fn harness(
        self,
        transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
        bug_manager: &mut BugManager,
        input: &[u8],
    ) {
        let configuration = self.ziggy_config.config.to_owned();
        let decoded_msgs: OneInput =
            parse_input(input, transcoder_loader, configuration.to_owned());

        if Self::should_stop_now(bug_manager, decoded_msgs.messages.to_owned()) {
            return;
        }

        let mut chain = BasicExternalities::new(self.setup.genesis.clone());
        chain.execute_with(|| timestamp(0));

        let mut coverage: InputCoverage = Default::default();

        let all_msg_responses = self.execute_messages(&decoded_msgs, &mut chain, &mut coverage);

        chain.execute_with(|| {
            check_invariants(
                bug_manager,
                &all_msg_responses,
                &decoded_msgs,
                transcoder_loader,
            )
        });

        // If we are not in fuzzing mode, we save the coverage
        // If you ever wish to have real-time coverage while fuzzing (and a lose
        // of performance) Simply comment out the following line :)
        #[cfg(not(fuzzing))]
        {
            println!("[🚧UPDATE] Adding to the coverage file...");
            coverage
                .save(configuration.fuzz_output.unwrap_or_default())
                .expect("🙅 Cannot save the coverage");

            pretty_print(all_msg_responses, decoded_msgs);
        }

        // We now fake the coverage
        coverage.redirect_coverage();
    }

    fn exec_seed(self, seed: PathBuf) -> anyhow::Result<()> {
        let (mut transcoder_loader, mut invariant_manager) = self
            .to_owned()
            .init_fuzzer()
            .context("Couldn't grap the transcoder and the invariant manager")?;

        let data = fs::read(seed)?;
        self.harness(
            &mut transcoder_loader,
            &mut invariant_manager,
            data.as_bytes_ref(),
        );
        Ok(())
    }
}

fn write_dict_header(dict_file: &mut fs::File) -> io::Result<()> {
    writeln!(dict_file, "# Dictionary file for selectors")?;
    writeln!(
        dict_file,
        "# Lines starting with '#' and empty lines are ignored."
    )?;

    writeln!(dict_file, "delimiter=\"\x2A\x2A\x2A\x2A\x2A\x2A\x2A\x2A\"")
}

fn write_corpus_file(index: usize, selector: &Selector, corpus_dir: PathBuf) -> io::Result<()> {
    // 00010000 fa80c2f6 00
    let mut data = vec![0x00, 0x00, 0x00, 0x00, 0x01];
    let file_path = corpus_dir.join(format!("selector_{index}.bin"));
    data.extend_from_slice(selector.0.as_ref());
    fs::write(file_path, data)
}

fn write_dict_entry(dict_file: &mut fs::File, selector: &Selector) -> anyhow::Result<()> {
    writeln!(dict_file, "\"{}\"", selector)
        .with_context(|| format!("Couldn't write {selector} into the dict"))?;
    Ok(())
}

fn check_invariants(
    bug_manager: &mut BugManager,
    all_msg_responses: &[FullContractResponse],
    decoded_msgs: &OneInput,
    transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
) {
    all_msg_responses
        .iter()
        .filter(|response| bug_manager.is_contract_trapped(response))
        .for_each(|response| {
            bug_manager.display_trap(decoded_msgs.messages[0].clone(), response.clone());
        });

    if let Err(invariant_tested) =
        bug_manager.are_invariants_passing(decoded_msgs.messages[0].origin)
    {
        bug_manager.display_invariant(
            all_msg_responses.to_vec(),
            decoded_msgs.clone(),
            invariant_tested,
            transcoder_loader,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    #[test]
    fn test_parse_input() {
        let metadata_path = Path::new("sample/dns/target/ink/dns.json");
        let transcoder = Mutex::new(
            ContractMessageTranscoder::load(metadata_path)
                .expect("Failed to load `ContractMessageTranscoder`"),
        );

        let encoded_bytes =
            hex::decode("229b553f9400000000000000000027272727272727272700002727272727272727272727")
                .expect("Failed to decode hex string");

        assert!(
            transcoder
                .lock()
                .unwrap()
                .decode_contract_message(&mut &encoded_bytes[..])
                .is_ok(),
            "Failed to decode contract message"
        );

        let binding = transcoder.lock().unwrap();
        let messages = binding.metadata().spec().messages();
        assert!(!messages.is_empty(), "There should be some messages here");
    }
}
