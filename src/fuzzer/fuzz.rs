use crate::{
    cli::{
        ui::logger::LogWriter,
        ziggy::ZiggyConfig,
    },
    contract::{
        payload::PayloadCrafter,
        remote::{
            ContractSetup,
            FullContractResponse,
        },
        runtime::{
            RuntimeOrigin,
            Timestamp,
        },
        selectors::database::SelectorDatabase,
    },
    cover::coverage::InputCoverage,
    fuzzer::{
        environment::EnvironmentBuilder,
        fuzz::FuzzingMode::{
            ExecuteOneInput,
            Fuzz,
        },
        manager::CampaignManager,
        parser::{
            try_parse_input,
            OneInput,
            MIN_SEED_LEN,
        },
    },
    EmptyResult,
    ResultOf,
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
    pub fn new(ziggy_config: anyhow::Result<ZiggyConfig>) -> ResultOf<Self> {
        let config = ziggy_config?;
        Ok(Self {
            ziggy_config: config.to_owned(),
            setup: ContractSetup::initialize_wasm(config)?,
        })
    }

    pub fn execute_harness(self, mode: FuzzingMode) -> EmptyResult {
        match mode {
            Fuzz => {
                let manager = self.clone().init_fuzzer()?;
                ziggy::fuzz!(|data: &[u8]| {
                    self.harness(manager.to_owned(), data);
                });
            }
            ExecuteOneInput(seed_path) => {
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

    pub fn init_fuzzer(self) -> ResultOf<CampaignManager> {
        let contract_bridge = self.setup.clone();

        let invariants = PayloadCrafter::extract_invariants(&contract_bridge.json_specs)
            .context("No invariants found, check your contract")?;

        let conf = self.ziggy_config.config();
        let messages = PayloadCrafter::extract_all(conf.instrumented_contract().to_path_buf())
            .context("Couldn't extract all the messages selectors")?;

        let payable_messages = PayloadCrafter::extract_payables(&contract_bridge.json_specs)
            .context("Couldn't fetch payable messages")?;

        let mut database = SelectorDatabase::new();

        database.add_invariants(invariants);
        database.add_messages(messages);
        database.add_payables(payable_messages);

        let env_builder = EnvironmentBuilder::new(database.clone());

        env_builder
            .build_env(self.ziggy_config.to_owned())
            .context("Couldn't create corpus entries and dict")?;

        if conf.verbose {
            println!(
                "\n🚀 Now fuzzing `{}` ({})!\n",
                &contract_bridge.path_to_specs.as_os_str().to_str().unwrap(),
                &contract_bridge.contract_address
            );
        }

        CampaignManager::new(database, contract_bridge.clone(), conf.to_owned())
    }

    fn execute_messages(
        &self,
        input: &OneInput,
        chain: &mut BasicExternalities,
        coverage: &mut InputCoverage,
    ) -> Vec<FullContractResponse> {
        let mut responses = Vec::new();

        chain.execute_with(|| {
            for message in &input.messages {
                let transfer_value = if message.is_payable {
                    message.value_token
                } else {
                    0
                };

                let result: FullContractResponse = self.setup.clone().call(
                    &message.payload,
                    message.origin.into(),
                    transfer_value,
                    self.ziggy_config.config().clone(),
                );

                coverage.add_cov(result.clone().debug_message());
                responses.push(result);
            }
        });

        responses
    }

    pub fn harness(&self, manager: CampaignManager, input: &[u8]) {
        if input.len() < MIN_SEED_LEN {
            return;
        }

        let maybe_parsed_input: Option<OneInput> = try_parse_input(input, manager.to_owned());

        let parsed_input = match maybe_parsed_input {
            None => {
                return;
            }
            Some(parsed) => parsed,
        };

        let mut chain = BasicExternalities::new(self.setup.genesis.clone());
        chain.execute_with(|| {
            Timestamp::set(RuntimeOrigin::none(), 3000).unwrap();
        });

        let mut coverage = InputCoverage::new();
        let all_msg_responses = self.execute_messages(&parsed_input, &mut chain, &mut coverage);

        let cov = coverage.messages_coverage();
        // If we are not in fuzzing mode, we save the coverage
        // If you ever wish to have real-time coverage while fuzzing (and a lose
        // of performance) Simply comment out the following line :)
        #[cfg(not(fuzzing))]
        {
            parsed_input.pretty_print(all_msg_responses.clone());

            println!("[🚧UPDATE] Adding to the coverage file...");
            coverage
                .save(&manager.config().fuzz_output.unwrap_or_default())
                .expect("🙅 Cannot save the coverage");
            let debug = coverage.concatened_trace();
            println!("[🚧COVERAGE] Caught identifiers {cov:?}",);
            println!("[🚧DEBUG TRACE] Fetched the following trace: {debug:?}\n",);
        }
        // We now fake the coverage
        coverage.redirect_coverage(cov);

        // If the user has `show_ui` turned on, we save the fuzzed seed to display it on the UI
        if self.ziggy_config.config().show_ui {
            let seeder = LogWriter::new(parsed_input.to_owned(), coverage.to_owned());
            if LogWriter::should_save() {
                seeder.save(self.clone().ziggy_config.fuzz_output()).expect(
                    "\nYou should run `fuzz` at least once and have a valid `output` directory\n",
                );
            }
        }

        // We check the invariants at the end, as we might panic
        chain.execute_with(|| {
            manager.check_invariants(
                &all_msg_responses,
                &parsed_input,
                manager.config().catch_trapped_contract,
            )
        });
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
        EmptyResult,
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
            max_messages_per_exec: Some(4),
            ..Default::default()
        };
        ZiggyConfig::new_with_contract(config, PathBuf::from("sample/dns")).unwrap()
    }

    #[test]
    fn test_database_and_envbuilder() -> EmptyResult {
        let config = create_test_config();
        let contract_bridge = Fuzzer::new(Ok(config.clone()))?.setup;

        let invariants = PayloadCrafter::extract_invariants(&contract_bridge.json_specs).unwrap();

        let messages = PayloadCrafter::extract_all(config.contract_path()?.clone())?
            .into_iter()
            .filter(|s| !invariants.contains(s))
            .collect();

        let mut database = SelectorDatabase::new();
        database.add_invariants(invariants);
        database.add_messages(messages);

        let manager = CampaignManager::new(
            database.clone(),
            contract_bridge.clone(),
            config.config().to_owned(),
        )?;

        let env_builder = EnvironmentBuilder::new(database);

        env_builder.build_env(config.clone())?;

        let x = manager.database();
        let get_unique_messages = x.clone().get_unique_messages()?.len();

        assert_eq!(
            fs::read_dir(config.clone().fuzz_output().join("phink").join("corpus"))
                .expect("Failed to read directory")
                .count(),
            get_unique_messages
        );
        assert_eq!(get_unique_messages, 5 + 1); // msg + constructor

        let inv_counter = x.clone().invariants()?.len();
        assert_eq!(inv_counter, 1);

        assert_eq!(x.clone().messages()?.len(), get_unique_messages);

        let dict_path = config.fuzz_output().join("phink").join("selectors.dict");
        let dict: String = fs::read_to_string(dict_path.clone())?;
        assert!(dict.contains("********"));
        assert!(dict.contains("# Dictionary file for selector"));

        Ok(())
    }
    #[test]
    fn test_decode_constructor() {
        let metadata_path =
            Path::new("sample/multi-contract-caller/target/ink/multi_contract_caller.json");
        let transcoder = Mutex::new(
            ContractMessageTranscoder::load(metadata_path)
                .expect("Failed to load `ContractMessageTranscoder`"),
        );

        let encoded_bytes =
            hex::decode("9BAE9D5E5C1100007B000000ACAC0000CC5B763F7AA51000F4BD3F32F51151FF017FD22F9404D0308AFBDB3DE6F2E030E23910AC7DCDBB41BC52F1F2F923E49BAF32E9587DCD4D43D50408B62431D7B79C1A506DBEC4785423DDF36E66E2BEBA6CFEFCDD4F5708DFA3388E48").unwrap();
        let result = transcoder
            .lock()
            .unwrap()
            .decode_contract_constructor(&mut &encoded_bytes[..])
            .unwrap();
        // println!("{}", result);
        let expected = "new { init_value: 4444, version: 123, accumulator_code_hash: 0xacac0000cc5b763f7aa51000f4bd3f32f51151ff017fd22f9404d0308afbdb3d, adder_code_hash: 0xe6f2e030e23910ac7dcdbb41bc52f1f2f923e49baf32e9587dcd4d43d50408b6, subber_code_hash: 0x2431d7b79c1a506dbec4785423ddf36e66e2beba6cfefcdd4f5708dfa3388e48 }";
        assert_eq!(result.to_string(), expected);
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
