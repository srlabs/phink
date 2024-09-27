use crate::{
    cli::{
        config::{
            PFiles::CoverageTracePath,
            PhinkFiles,
        },
        ui::seed::SeedDisplayer,
        ziggy::ZiggyConfig,
    },
    contract::{
        payload::PayloadCrafter,
        remote::{
            ContractSetup,
            FullContractResponse,
        },
        selectors::{
            database::SelectorDatabase,
            selector::Selector,
        },
    },
    cover::coverage::InputCoverage,
    fuzzer::{
        engine::timestamp,
        environment::EnvironmentBuilder,
        fuzz::FuzzingMode::{
            ExecuteOneInput,
            Fuzz,
        },
        manager::CampaignManager,
        parser::{
            parse_input,
            Message,
            OneInput,
        },
    },
};
use anyhow::Context;
use frame_support::__private::BasicExternalities;
use sp_core::hexdisplay::AsBytesRef;
use std::{
    fs,
    path::PathBuf,
};

pub const MAX_MESSAGES_PER_EXEC: usize = 1; // One execution contains maximum 1 message.

pub enum FuzzingMode {
    ExecuteOneInput(PathBuf),
    Fuzz,
}

#[derive(Clone)]
pub struct Fuzzer {
    pub ziggy_config: ZiggyConfig,
    pub setup: ContractSetup,
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
                let manager = self.clone().init_fuzzer()?;
                ziggy::fuzz!(|data: &[u8]| {
                    Self::harness(&self, manager.to_owned(), data);
                });
            }
            ExecuteOneInput(seed_path) => {
                let covpath = PhinkFiles::new(config.clone().fuzz_output()).path(CoverageTracePath);
                let _ = fs::remove_file(covpath); // we also reset the cov map, doesn't matter if it fails
                let manager = self
                    .to_owned()
                    .init_fuzzer()
                    .context("Couldn't grap the transcoder and the invariant manager")?;

                let data = fs::read(seed_path).context("Couldn't read the seed")?;
                self.harness(manager, data.as_bytes_ref());
            }
        }

        Ok(())
    }

    fn should_stop_now(manager: &CampaignManager, messages: Vec<Message>) -> bool {
        // todo: need to refactor this

        messages.is_empty()
            || messages.iter().any(|payload| {
                payload
                    .payload
                    .get(..4)
                    .and_then(|slice| Selector::try_from(slice).ok())
                    .map_or(false, |slice: Selector| {
                        manager.database().invariants().unwrap().contains(&slice)
                    })
            })
    }

    pub fn init_fuzzer(self) -> anyhow::Result<CampaignManager> {
        let contract_bridge = self.setup.clone();

        let invariants = PayloadCrafter::extract_invariants(&contract_bridge.json_specs)
            .context("🙅 No invariants found, check your contract")?;

        let messages = PayloadCrafter::extract_all(self.ziggy_config.contract_path.to_owned())
            .context("Couldn't extract all the messages selectors")?;

        let payable_messages = PayloadCrafter::extract_payables(&contract_bridge.json_specs)
            .context("Couldn't fetch payable messages")?;

        let mut database = SelectorDatabase::new();
        database.add_invariants(invariants);
        database.add_messages(messages);
        database.add_payables(payable_messages);

        let manager = CampaignManager::new(
            database.clone(),
            contract_bridge.clone(),
            self.ziggy_config.config.to_owned(),
        );

        let env_builder = EnvironmentBuilder::new(database);

        env_builder
            .build_env(self.ziggy_config.fuzz_output())
            .context("🙅 Couldn't create corpus entries and dict")?;

        println!(
            "\n🚀  Now fuzzing `{}` ({})!\n",
            &contract_bridge.path_to_specs.as_os_str().to_str().unwrap(),
            &contract_bridge.contract_address
        );

        manager
    }

    fn execute_messages(
        &self,
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

                coverage.add_cov(&result.clone().debug_message());
                all_msg_responses.push(result);
            }
        });

        if self.ziggy_config.config.show_ui {
            let mut seeder = SeedDisplayer::new(
                decoded_msgs.to_owned(),
                coverage.to_owned(),
                all_msg_responses.clone(),
            );
            if seeder.should_save() {
                seeder.save(self.clone().ziggy_config.fuzz_output());
            }
        }

        all_msg_responses
    }

    pub fn harness(&self, manager: CampaignManager, input: &[u8]) {
        let decoded_msgs: OneInput = parse_input(input, manager.to_owned());

        if Self::should_stop_now(&manager, decoded_msgs.messages.to_owned()) {
            return;
        }

        let mut chain = BasicExternalities::new(self.setup.genesis.clone());
        chain.execute_with(|| timestamp(0));

        let mut coverage: InputCoverage = Default::default();

        let all_msg_responses = self.execute_messages(&decoded_msgs, &mut chain, &mut coverage);

        chain.execute_with(|| manager.check_invariants(&all_msg_responses, &decoded_msgs));

        // If we are not in fuzzing mode, we save the coverage
        // If you ever wish to have real-time coverage while fuzzing (and a lose
        // of performance) Simply comment out the following line :)
        #[cfg(not(fuzzing))]
        {
            println!("[🚧UPDATE] Adding to the coverage file...");
            coverage
                .save(manager.config().fuzz_output.unwrap_or_default())
                .expect("🙅 Cannot save the coverage");

            crate::fuzzer::engine::pretty_print(all_msg_responses, decoded_msgs);
        }

        // We now fake the coverage
        coverage.redirect_coverage(coverage.flatten_cov());
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cli::{
            config::Configuration,
            ziggy::ZiggyConfig,
        },
        contract::{
            payload::PayloadCrafter,
            selectors::database::SelectorDatabase,
        },
        fuzzer::{
            environment::EnvironmentBuilder,
            fuzz::Fuzzer,
            manager::CampaignManager,
        },
        instrumenter::path::InstrumentedPath,
    };
    use contract_transcode::ContractMessageTranscoder;

    use std::{
        fs,
        path::{
            Path,
            PathBuf,
        },
        sync::Mutex,
    };
    use tempfile::tempdir;

    fn create_test_config() -> ZiggyConfig {
        let config = Configuration {
            verbose: true,
            cores: Some(1),
            use_honggfuzz: false,
            fuzz_output: Some(tempdir().unwrap().into_path()),
            instrumented_contract_path: Some(InstrumentedPath::from("sample/dns")),
            show_ui: false,
            ..Default::default()
        };
        ZiggyConfig::new(config, PathBuf::from("sample/dns")).unwrap()
    }

    #[test]
    fn test_database_and_envbuilder() -> anyhow::Result<()> {
        let config = create_test_config();
        let contract_bridge = Fuzzer::new(config.clone())?.setup;

        let invariants = PayloadCrafter::extract_invariants(&contract_bridge.json_specs).unwrap();

        let messages = PayloadCrafter::extract_all(config.contract_path.clone())?
            .into_iter()
            .filter(|s| !invariants.contains(s))
            .collect();

        let mut database = SelectorDatabase::new();
        database.add_invariants(invariants);
        database.add_messages(messages);

        let manager = CampaignManager::new(
            database.clone(),
            contract_bridge.clone(),
            config.config.to_owned(),
        )?;

        let env_builder = EnvironmentBuilder::new(database);

        env_builder.build_env(config.clone().fuzz_output())?;

        let get_unique_messages = manager.database().get_unique_messages()?.len();

        assert_eq!(
            fs::read_dir(config.clone().fuzz_output().join("phink").join("corpus"))
                .expect("Failed to read directory")
                .count(),
            get_unique_messages
        );
        assert_eq!(get_unique_messages, 5 + 1); // msg + constructor

        let inv_counter = manager.database().invariants()?.len();
        assert_eq!(inv_counter, 1);

        assert_eq!(manager.database().messages()?.len(), get_unique_messages);

        let dict_path = config.fuzz_output().join("phink").join("selectors.dict");
        let dict: String = fs::read_to_string(dict_path.clone())?;
        assert_eq!(dict_path.iter().count(), 9);
        assert!(dict.contains("********"));
        assert!(dict.contains("# Dictionary file for selector"));
        // todo: not sure if we keep the dict
        // assert!(dict.contains("9bae9d5e"));
        // assert!(dict.contains("229b553f"));
        // assert!(dict.contains("b8a4d3d9"));
        // assert!(dict.contains("84a15da1"));
        // assert!(dict.contains("d259f7ba"));
        // assert!(dict.contains("07fcd0b1"));
        //   1   │ # Dictionary file for selectors
        //    2   │ # Lines starting with '#' and empty lines are ignored.
        //    3   │ delimiter="********"
        //    4   │ "9bae9d5e"
        //    5   │ "229b553f"
        //    6   │ "b8a4d3d9"
        //    7   │ "84a15da1"
        //    8   │ "d259f7ba"
        //    9   │ "07fcd0b1"

        Ok(())
    }
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

    #[test]
    fn test_parse_dummy() {
        let metadata_path = Path::new("sample/dummy/target/ink/dummy.json");
        let transcoder = Mutex::new(
            ContractMessageTranscoder::load(metadata_path)
                .expect("Failed to load `ContractMessageTranscoder`"),
        );

        let encoded_bytes = hex::decode("fa80c2f600").expect("Failed to decode hex string");

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
