use crate::{
    cli::config::OriginFuzzingOption::{
        DisableOriginFuzzing,
        EnableOriginFuzzing,
    },
    contract::{
        remote::{
            BalanceOf,
            ContractSetup,
        },
        runtime::Runtime,
    },
    fuzzer::fuzz::MAX_MESSAGES_PER_EXEC,
    instrumenter::path::InstrumentedPath,
};
use anyhow::Context;
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
    /// Make Phink more verbose
    pub verbose: bool,
    /// Path where the instrumented contract will be stored after running `phink
    /// instrument mycontract` By default, we create a random folder in
    /// `/tmp/ink_fuzzed_XXXX`
    pub instrumented_contract_path: Option<InstrumentedPath>,
    /// Path where Ziggy will drop everything (logs, corpus, etc). If `None`, it'll be
    /// `output/` by default
    pub fuzz_output: Option<PathBuf>,
    /// Use the Phink UI. If set to `false`, the Ziggy native UI will be used.
    pub show_ui: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            cores: Some(1),
            use_honggfuzz: false,
            fuzz_origin: false,
            deployer_address: ContractSetup::DEFAULT_DEPLOYER.into(),
            max_messages_per_exec: MAX_MESSAGES_PER_EXEC.into(),
            report_path: Some(PathBuf::from("output/coverage_report")),
            default_gas_limit: Some(ContractSetup::DEFAULT_GAS_LIMIT),
            storage_deposit_limit: Some("100000000000".into()),
            instantiate_initial_value: None,
            constructor_payload: None,
            verbose: true,
            instrumented_contract_path: Some(InstrumentedPath::default()),
            fuzz_output: None,
            show_ui: true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum OriginFuzzingOption {
    EnableOriginFuzzing,
    #[default]
    DisableOriginFuzzing,
}

#[derive(Copy, Clone, Debug)]
pub enum PFiles {
    CoverageTracePath,
    AllowlistPath,
    DictPath,
    CorpusPath,
}
#[derive(Clone, Debug)]
pub struct PhinkFiles {
    output: PathBuf,
}
impl PhinkFiles {
    pub fn new(output: PathBuf) -> Self {
        Self { output }
    }

    pub fn output(self) -> PathBuf {
        self.output
    }
    pub fn path(&self, file: PFiles) -> PathBuf {
        const PHINK_PATH: &str = "phink";
        match file {
            PFiles::CoverageTracePath => self.output.join(PHINK_PATH).join("traces.cov"),
            PFiles::AllowlistPath => self.output.join(PHINK_PATH).join("allowlist.txt"),
            PFiles::DictPath => self.output.join(PHINK_PATH).join("selectors.dict"),
            PFiles::CorpusPath => self.output.join(PHINK_PATH).join("corpus"),
        }
    }
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

    pub fn save_as_toml(&self, to: &str) -> anyhow::Result<()> {
        let toml_str =
            toml::to_string(self).with_context(|| "Couldn't serialize to toml".to_string())?;
        let mut file = File::create(to).with_context(|| format!("Couldn't create file {}", to))?;
        file.write_all(toml_str.as_bytes())?;
        Ok(())
    }

    pub fn parse_balance(value: Option<String>) -> Option<BalanceOf<Runtime>> {
        // Currently, TOML & Serde don't handle parsing `u128` 🤡
        // So we need to parse it as a `string`... to then revert it to `u128`
        // (which is `BalanceOf<T>`)
        value.and_then(|s| s.parse::<u128>().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn create_test_config() -> Configuration {
        Configuration {
            cores: Some(2),
            use_honggfuzz: true,
            deployer_address: Some(
                AccountId32::from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap(),
            ),
            max_messages_per_exec: Some(10),
            report_path: Some(PathBuf::from("/tmp/report")),
            fuzz_origin: true,
            default_gas_limit: Some(Weight::from_parts(100_000_000_000, 0)),
            storage_deposit_limit: Some("1000000000".into()),
            instantiate_initial_value: Some("500".into()),
            constructor_payload: Some("0x1234".into()),
            verbose: true,
            instrumented_contract_path: Some(InstrumentedPath::from("/tmp/instrumented")),
            fuzz_output: Some(PathBuf::from("/tmp/fuzz_output")),
            show_ui: false,
        }
    }

    #[test]
    fn test_default_configuration() {
        let default_config = Configuration::default();
        assert_eq!(default_config.cores, Some(1));
        assert!(!default_config.use_honggfuzz);
        assert!(!default_config.fuzz_origin);
        assert_eq!(
            default_config.max_messages_per_exec,
            Some(MAX_MESSAGES_PER_EXEC)
        );
        assert_eq!(
            default_config.report_path,
            Some(PathBuf::from("output/coverage_report"))
        );
        assert_eq!(
            default_config.default_gas_limit,
            Some(ContractSetup::DEFAULT_GAS_LIMIT)
        );
        assert_eq!(
            default_config.storage_deposit_limit,
            Some("100000000000".into())
        );
        assert!(default_config.show_ui);
    }

    #[test]
    fn test_should_fuzz_origin() {
        let mut config = create_test_config();
        assert_eq!(config.should_fuzz_origin(), EnableOriginFuzzing);

        config.fuzz_origin = false;
        assert_eq!(config.should_fuzz_origin(), DisableOriginFuzzing);
    }

    #[test]
    fn test_parse_balance() {
        assert_eq!(Configuration::parse_balance(Some("100".into())), Some(100));
        assert_eq!(Configuration::parse_balance(Some("0".into())), Some(0));
        assert_eq!(
            Configuration::parse_balance(Some("18446744073709551615".into())),
            Some(18446744073709551615)
        );
        assert_eq!(Configuration::parse_balance(None), None);
        assert_eq!(Configuration::parse_balance(Some("invalid".into())), None);
    }

    #[test]
    fn test_try_from_string() {
        let config_str = r#"
            cores = 4
            use_honggfuzz = true
            fuzz_origin = true
            max_messages_per_exec = 20
            storage_deposit_limit = "200000000000"
            verbose = false
            show_ui = true
        "#;

        let config: Configuration = config_str.to_string().try_into().unwrap();
        assert_eq!(config.cores, Some(4));
        assert!(config.use_honggfuzz);
        assert!(config.fuzz_origin);
        assert_eq!(config.max_messages_per_exec, Some(20));
        assert_eq!(config.storage_deposit_limit, Some("200000000000".into()));
        assert!(!config.verbose);
        assert!(config.show_ui);
    }

    #[test]
    fn test_try_from_string_invalid_config() {
        let invalid_config_str = r#"
            cores = "invalid"
            storage_deposit_limit = "not_a_number"
        "#;

        let result: Result<Configuration, String> = invalid_config_str.to_string().try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_phink_files() {
        let output = PathBuf::from("/tmp/phink_output");
        let phink_files = PhinkFiles::new(output.clone());

        assert_eq!(
            phink_files.path(PFiles::CoverageTracePath),
            output.join("phink").join("traces.cov")
        );
        assert_eq!(
            phink_files.path(PFiles::AllowlistPath),
            output.join("phink").join("allowlist.txt")
        );
        assert_eq!(
            phink_files.path(PFiles::DictPath),
            output.join("phink").join("selectors.dict")
        );
        assert_eq!(
            phink_files.path(PFiles::CorpusPath),
            output.join("phink").join("corpus")
        );
    }

    #[test]
    fn test_save_as_toml() {
        use tempfile::tempdir;

        let config = create_test_config();
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("config.toml");

        assert!(config.save_as_toml(file_path.to_str().unwrap()).is_ok());

        let saved_config: Configuration = Configuration::try_from(&file_path).unwrap();
        assert_eq!(saved_config, config);
    }
}
