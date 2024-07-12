use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};

use contract_transcode::ContractMessageTranscoder;
use frame_support::__private::BasicExternalities;
use sp_core::hexdisplay::AsBytesRef;

use crate::cli::ziggy::ZiggyConfig;
use crate::{
    contract::{
        payload::{PayloadCrafter, Selector},
        remote::{ContractBridge, FullContractResponse},
    },
    cover::coverage::Coverage,
    fuzzer::fuzz::FuzzingMode::{ExecuteOneInput, Fuzz},
    fuzzer::{
        bug::BugManager,
        engine::FuzzerEngine,
        parser::{parse_input, OneInput},
    },
    instrumenter::instrument::Instrumenter,
};

pub const CORPUS_DIR: &str = "./output/phink/corpus";
pub const DICT_FILE: &str = "./output/phink/selectors.dict";
pub const MAX_MESSAGES_PER_EXEC: usize = 4; // One execution contains maximum 4 messages.

pub enum FuzzingMode {
    ExecuteOneInput(PathBuf),
    Fuzz,
}

#[derive(Clone)]
pub struct Fuzzer {
    setup: ContractBridge,
    max_messages_per_exec: usize,
}

impl Fuzzer {
    pub fn new(setup: ContractBridge) -> Self {
        Self {
            setup,
            max_messages_per_exec: MAX_MESSAGES_PER_EXEC,
        }
    }

    pub fn set_max_messages_per_exec(&mut self, max_messages_per_exec: Option<usize>) {
        self.max_messages_per_exec = max_messages_per_exec.unwrap_or(MAX_MESSAGES_PER_EXEC);
    }

    pub fn execute_harness(mode: FuzzingMode, config: ZiggyConfig) -> io::Result<()> {
        let finder = Instrumenter::new(config.contract_path).find().unwrap();
        let wasm = fs::read(&finder.wasm_path)?;
        let setup = ContractBridge::initialize_wasm(
            wasm,
            &finder.specs_path,
            config
                .config
                .deployer_address
                .unwrap_or(ContractBridge::DEFAULT_DEPLOYER),
        );
        let mut fuzzer = Fuzzer::new(setup);

        match mode {
            Fuzz => {
                fuzzer.set_max_messages_per_exec(config.config.max_messages_per_exec);
                fuzzer.fuzz();
            }
            ExecuteOneInput(seed_path) => {
                fuzzer.exec_seed(seed_path);
            }
        }

        Ok(())
    }

    fn build_corpus_and_dict(selectors: &[Selector]) -> io::Result<()> {
        fs::create_dir_all(CORPUS_DIR)?;
        let mut dict_file = fs::File::create(DICT_FILE)?;

        write_dict_header(&mut dict_file)?;

        for (i, selector) in selectors.iter().enumerate() {
            write_corpus_file(i, selector)?;
            write_dict_entry(&mut dict_file, selector)?;
        }

        Ok(())
    }

    fn should_stop_now(bug_manager: &BugManager, decoded_msgs: &OneInput) -> bool {
        decoded_msgs.messages.is_empty()
            || decoded_msgs.messages.iter().any(|payload| {
                payload
                    .payload
                    .get(..4)
                    .and_then(|slice| slice.try_into().ok())
                    .map_or(false, |slice: &[u8; 4]| {
                        bug_manager.contains_selector(slice)
                    })
            })
    }
}

impl FuzzerEngine for Fuzzer {
    fn fuzz(self) {
        let (mut transcoder_loader, invariant_manager) = init_fuzzer(self.clone());

        ziggy::fuzz!(|data: &[u8]| {
            Self::harness(
                self.clone(),
                &mut transcoder_loader,
                &mut invariant_manager.clone(),
                data,
            );
        });
    }

    fn harness(
        client: Fuzzer,
        transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
        bug_manager: &mut BugManager,
        input: &[u8],
    ) {
        let decoded_msgs: OneInput =
            parse_input(input, transcoder_loader, client.max_messages_per_exec);

        if Self::should_stop_now(bug_manager, &decoded_msgs) {
            return;
        }

        let mut chain = BasicExternalities::new(client.setup.genesis.clone());
        chain.execute_with(|| <Fuzzer as FuzzerEngine>::timestamp(0));

        let mut coverage = Coverage::new();

        let all_msg_responses = execute_messages(&client, &decoded_msgs, &mut chain, &mut coverage);

        chain.execute_with(|| {
            check_invariants(
                bug_manager,
                &all_msg_responses,
                &decoded_msgs,
                transcoder_loader,
            )
        });
        // Pretty print all the calls of the current input
        <Fuzzer as FuzzerEngine>::pretty_print(all_msg_responses, decoded_msgs);

        // We now fake the coverage
        coverage.redirect_coverage();

        // If we are not in fuzzing mode, we save the coverage
        // If you ever wish to have real-time coverage while fuzzing (and a lose of performance)
        // Simply comment out the following line :)
        #[cfg(not(fuzzing))]
        {
            println!("[🚧UPDATE] Adding to the coverage file...");
            coverage.save().expect("🙅 Cannot save the coverage");
        }
    }

    fn exec_seed(self, seed: PathBuf) {
        let (mut transcoder_loader, mut invariant_manager) = init_fuzzer(self.clone());
        let data = fs::read(seed).unwrap();
        Self::harness(
            self,
            &mut transcoder_loader,
            &mut invariant_manager,
            data.as_bytes_ref(),
        );
    }
}

fn init_fuzzer(fuzzer: Fuzzer) -> (Mutex<ContractMessageTranscoder>, BugManager) {
    let transcoder_loader = Mutex::new(
        ContractMessageTranscoder::load(Path::new(&fuzzer.setup.path_to_specs))
            .expect("Failed to load ContractMessageTranscoder"),
    );

    let specs = &fuzzer.setup.json_specs;
    let selectors = PayloadCrafter::extract_all(specs);
    let invariants = PayloadCrafter::extract_invariants(specs)
        .expect("🙅 No invariants found, check your contract");

    let selectors_without_invariants: Vec<Selector> = selectors
        .into_iter()
        .filter(|s| !invariants.contains(s))
        .collect();

    let invariant_manager = BugManager::from(invariants, fuzzer.setup.clone());

    Fuzzer::build_corpus_and_dict(&selectors_without_invariants)
        .expect("🙅 Failed to create initial corpus");

    println!(
        "\n🚀  Now fuzzing `{}` ({})!\n",
        fuzzer.setup.path_to_specs.as_os_str().to_str().unwrap(),
        fuzzer.setup.contract_address
    );

    (transcoder_loader, invariant_manager)
}

fn write_dict_header(dict_file: &mut fs::File) -> io::Result<()> {
    writeln!(dict_file, "# Dictionary file for selectors")?;
    writeln!(
        dict_file,
        "# Lines starting with '#' and empty lines are ignored."
    )?;

    writeln!(dict_file, "delimiter=\"\x2A\x2A\x2A\x2A\x2A\x2A\x2A\x2A\"")
}

fn write_corpus_file(index: usize, selector: &Selector) -> io::Result<()> {
    let file_path = PathBuf::from(CORPUS_DIR).join(format!("selector_{}.bin", index));
    fs::write(file_path, selector)
}

fn write_dict_entry(dict_file: &mut fs::File, selector: &Selector) -> io::Result<()> {
    let selector_string = selector
        .iter()
        .map(|b| format!("\\x{:02X}", b))
        .collect::<String>();
    writeln!(dict_file, "\"{}\"", selector_string)
}

fn execute_messages(
    client: &Fuzzer,
    decoded_msgs: &OneInput,
    chain: &mut BasicExternalities,
    coverage: &mut Coverage,
) -> Vec<FullContractResponse> {
    let mut all_msg_responses = Vec::new();

    chain.execute_with(|| {
        for message in &decoded_msgs.messages {
            let transfer_value = if message.is_payable {
                message.value_token
            } else {
                0
            };

            let result: FullContractResponse =
                client
                    .setup
                    .clone()
                    .call(&message.payload, decoded_msgs.origin, transfer_value);

            coverage.add_cov(&result.debug_message);
            all_msg_responses.push(result);
        }
    });

    all_msg_responses
}

fn check_invariants(
    bug_manager: &mut BugManager,
    all_msg_responses: &[FullContractResponse],
    decoded_msgs: &OneInput,
    transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
) {
    for result in all_msg_responses {
        if bug_manager.is_contract_trapped(result) {
            bug_manager.display_trap(decoded_msgs.messages[0].clone(), result.clone());
        }
    }

    if let Err(invariant_tested) = bug_manager.are_invariants_passing(decoded_msgs.origin) {
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
    use std::path::Path;

    use super::*;

    #[test]
    fn test_parse_input() {
        let metadata_path = Path::new("sample/dns/target/ink/dns.json");
        let mut transcoder = Mutex::new(
            ContractMessageTranscoder::load(metadata_path)
                .expect("Failed to load ContractMessageTranscoder"),
        );

        let encoded_bytes =
            hex::decode("229b553f9400000000000000000027272727272727272700002727272727272727272727")
                .expect("Failed to decode hex string");

        let hex = transcoder
            .lock()
            .unwrap()
            .decode_contract_message(&mut &encoded_bytes[..])
            .expect("Failed to decode contract message");

        println!("{:#?}", hex);

        let binding = transcoder.lock().unwrap();
        let messages = binding.metadata().spec().messages();
        println!("{:#?}", messages);
    }
}
