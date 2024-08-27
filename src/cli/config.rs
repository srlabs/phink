use crate::{
    cli::config::OriginFuzzingOption::{
        DisableOriginFuzzing,
        EnableOriginFuzzing,
    },
    contract::{
        remote::{
            BalanceOf,
            ContractBridge,
        },
        runtime::Runtime,
    },
    fuzzer::fuzz::MAX_MESSAGES_PER_EXEC,
    instrumenter::instrumented_path::InstrumentedPath,
};
use frame_support::weights::Weight;
use serde_derive::{
    Deserialize,
    Serialize,
};
use sp_core::crypto::AccountId32;
use std::{
    fs,
    fs::File,
    io::Write,
    path::PathBuf,
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Configuration {
    /// Number of cores to use for Ziggy
    pub cores: Option<u8>,
    /// Also use Hongfuzz as a fuzzer
    pub use_honggfuzz: bool,
    // Origin deploying and instantiating the contract
    pub deployer_address: Option<AccountId32>,
    // Maximum number of ink! message executed per seed
    pub max_messages_per_exec: Option<usize>,
    /// Output directory for the coverage report
    pub report_path: Option<PathBuf>,
    /// Fuzz the origin. If `false`, the fuzzer will execute each message with
    /// the same account.
    pub fuzz_origin: bool,
    /// The gas limit enforced when executing the constructor
    pub default_gas_limit: Option<Weight>,
    /// The maximum amount of balance that can be charged from the caller to
    /// pay for the storage consumed.
    pub storage_deposit_limit: Option<String>,
    /// The `value` being transferred to the new account during the contract
    /// instantiation
    pub instantiate_initial_value: Option<String>,
    /// In the case where you wouldn't have any default constructor in you
    /// smart contract, i.e `new()` (without parameters), then you would
    /// need to specify inside the config file the `Vec<u8>` representation
    /// of the SCALE-encoded data of your constructor. This typically
    /// involved the four first bytes of the constructor' selector,
    /// followed by the payload.
    pub constructor_payload: Option<String>,
    /// Make Phink verbose to stdout
    pub verbose: bool,
    /// Path where the instrumented contract will be stored after running `phink
    /// instrument mycontract` By default, we create a random folder in
    /// `/tmp/ink_fuzzed_XXXX`
    pub instrumented_contract_path: Option<InstrumentedPath>,
    /// Path where Ziggy will drop everything (logs, corpus, etc). If None, it'll be
    /// output/ by default
    pub fuzz_output: Option<PathBuf>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            cores: Some(1),
            use_honggfuzz: false,
            fuzz_origin: false,
            deployer_address: ContractBridge::DEFAULT_DEPLOYER.into(),
            max_messages_per_exec: MAX_MESSAGES_PER_EXEC.into(),
            report_path: Some(PathBuf::from("output/coverage_report")),
            default_gas_limit: Some(ContractBridge::DEFAULT_GAS_LIMIT),
            storage_deposit_limit: Some("100000000000".into()),
            instantiate_initial_value: None,
            constructor_payload: None,
            verbose: true,
            instrumented_contract_path: Some(InstrumentedPath::default()),
            fuzz_output: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum OriginFuzzingOption {
    EnableOriginFuzzing,
    #[default]
    DisableOriginFuzzing,
}

impl TryFrom<String> for Configuration {
    type Error = String;
    fn try_from(config_str: String) -> Result<Self, Self::Error> {
        let config: Configuration = match toml::from_str(&config_str) {
            Ok(config) => config,
            Err(e) => return Err(format!("❌ Can't parse config: {}", e)),
        };

        if Configuration::parse_balance(config.storage_deposit_limit.clone()).is_none() {
            return Err("❌ Cannot parse string to `u128` for `storage_deposit_limit`, check your configuration file".into());
        }

        Ok(config)
    }
}

impl TryFrom<&PathBuf> for Configuration {
    type Error = String;
    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        match fs::read_to_string(path) {
            Ok(config) => config.try_into(),
            Err(err) => Err(format!("🚫 Can't read config: {}", err)),
        }
    }
}

impl Configuration {
    pub fn should_fuzz_origin(&self) -> OriginFuzzingOption {
        match self.fuzz_origin {
            true => EnableOriginFuzzing,
            false => DisableOriginFuzzing,
        }
    }

    pub fn save_as_toml(&self, to: &str) {
        let toml_str = toml::to_string(self).unwrap();
        let mut file = File::create(to).unwrap();
        file.write_all(toml_str.as_bytes()).unwrap();
    }

    pub fn parse_balance(value: Option<String>) -> Option<BalanceOf<Runtime>> {
        // Currently, TOML & Serde don't handle parsing `u128` 🤡
        // So we need to parse it as a `string`... to then revert it to `u128`
        // (which is `BalanceOf<T>`)
        value.and_then(|s| s.parse::<u128>().ok()).map(|v| v)
    }
}
