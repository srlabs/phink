use crate::{
    cli::{
        config::OriginFuzzingOption::{
            DisableOriginFuzzing,
            EnableOriginFuzzing,
        },
        ui::logger::LAST_SEED_FILENAME,
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
use anyhow::{
    bail,
    Context,
};
use frame_support::weights::Weight;
use serde_derive::{
    Deserialize,
    Serialize,
};
use sp_core::crypto::AccountId32;
use std::{
    fmt::{
        Display,
        Formatter,
    },
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
    /// If `true`, the fuzzer will detect trapped contracts (`ContractTrapped`) as a bug. Set this
    /// to false if you just want to catch invariants. Set this to true if you want any kind of
    /// bugs.
    pub catch_trapped_contract: bool,
    /// If `true`, Phink will run the tests of the ink! contract to execute the messages and
    /// extracts valid seeds inside. For instance if a test call three messages, those three
    /// messages will be SCALE-encoded, and put inside external files.
    pub generate_seeds: bool,
}

impl Configuration {
    pub fn deployer_address(&self) -> &AccountId32 {
        self.deployer_address
            .as_ref()
            .unwrap_or(&ContractSetup::DEFAULT_DEPLOYER)
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            cores: Some(1),
            use_honggfuzz: false,
            fuzz_origin: false,
            deployer_address: Some(ContractSetup::DEFAULT_DEPLOYER),
            max_messages_per_exec: Some(MAX_MESSAGES_PER_EXEC),
            report_path: Some(PathBuf::from("output/coverage_report")),
            default_gas_limit: Some(ContractSetup::DEFAULT_GAS_LIMIT),
            storage_deposit_limit: Some("100000000000".into()),
            instantiate_initial_value: None,
            constructor_payload: None,
            verbose: true,
            instrumented_contract_path: Some(InstrumentedPath::default()),
            fuzz_output: Some(PathBuf::from("output")),
            show_ui: true,
            catch_trapped_contract: false,
            generate_seeds: true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    AFLLog,
    LastSeed,
}
#[derive(Clone, Debug)]
pub struct PhinkFiles {
    output: PathBuf,
}
impl Display for PhinkFiles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.output.to_str().unwrap())
    }
}

impl PhinkFiles {
    const PHINK_PATH: &str = "phink";

    pub fn new(output: PathBuf) -> Self {
        Self { output }
    }
    pub fn new_by_ref(output: &PathBuf) -> Self {
        Self {
            output: output.to_owned(),
        }
    }
    pub fn output(self) -> PathBuf {
        self.output
    }

    pub fn make_all(self) -> Self {
        fs::create_dir_all(self.clone().output().join(Self::PHINK_PATH)).unwrap();
        self
    }

    pub fn path(&self, file: PFiles) -> PathBuf {
        match file {
            PFiles::CoverageTracePath => self.output.join(Self::PHINK_PATH).join("traces.cov"),
            PFiles::AllowlistPath => self.output.join(Self::PHINK_PATH).join("allowlist.txt"),
            PFiles::DictPath => self.output.join(Self::PHINK_PATH).join("selectors.dict"),
            PFiles::CorpusPath => self.output.join(Self::PHINK_PATH).join("corpus"),
            PFiles::AFLLog => {
                self.output
                    .join(Self::PHINK_PATH)
                    .join("logs")
                    .join("afl.log")
            }
            PFiles::LastSeed => {
                self.output
                    .join(Self::PHINK_PATH)
                    .join("logs")
                    .join(LAST_SEED_FILENAME)
            }
        }
    }
}

impl TryFrom<String> for Configuration {
    type Error = anyhow::Error;
    fn try_from(config_str: String) -> anyhow::Result<Self> {
        let config: Configuration = match toml::from_str(&config_str) {
            Ok(config) => config,
            Err(e) => bail!("Can't parse config: {e}"),
        };

        if Configuration::parse_balance(&config.storage_deposit_limit.clone()).is_none() {
            bail!("Cannot parse string to `u128` for `storage_deposit_limit`, check your configuration file");
        }

        Ok(config)
    }
}

impl TryFrom<&PathBuf> for Configuration {
    type Error = anyhow::Error;
    fn try_from(path: &PathBuf) -> anyhow::Result<Self> {
        match fs::read_to_string(path) {
            Ok(config) => config.try_into(),
            Err(err) => bail!("🚫 Can't read config: {err}"),
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

    pub fn instrumented_contract(&self) -> PathBuf {
        self.instrumented_contract_path
            .clone()
            .unwrap_or_default()
            .path
    }

    pub fn save_as_toml(&self, to: &str) -> anyhow::Result<()> {
        let toml_str =
            toml::to_string(self).with_context(|| "Couldn't serialize to toml".to_string())?;
        let mut file = File::create(to).with_context(|| format!("Couldn't create file {}", to))?;
        file.write_all(toml_str.as_bytes())?;
        Ok(())
    }

    pub fn parse_balance(value: &Option<String>) -> Option<BalanceOf<Runtime>> {
        // Currently, TOML & Serde don't handle parsing `u128` 🤡
        // So we need to parse it as a `string`... to then revert it to `u128`
        // (which is `BalanceOf<T>`)
        value.clone().and_then(|s| s.parse::<u128>().ok())
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
            catch_trapped_contract: false,
            default_gas_limit: Some(Weight::from_parts(100_000_000_000, 0)),
            storage_deposit_limit: Some("1000000000".into()),
            instantiate_initial_value: Some("500".into()),
            constructor_payload: Some("0x1234".into()),
            verbose: true,
            instrumented_contract_path: Some(InstrumentedPath::from("/tmp/instrumented")),
            fuzz_output: Some(PathBuf::from("/tmp/fuzz_output")),
            show_ui: false,
            ..Default::default()
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
        assert_eq!(Configuration::parse_balance(&Some("100".into())), Some(100));
        assert_eq!(Configuration::parse_balance(&Some("0".into())), Some(0));
        assert_eq!(
            Configuration::parse_balance(&Some("18446744073709551615".into())),
            Some(18446744073709551615)
        );
        assert_eq!(Configuration::parse_balance(&None), None);
        assert_eq!(Configuration::parse_balance(&Some("invalid".into())), None);
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
            catch_trapped_contract = true
            show_ui = true
            generate_seeds = false
        "#;

        let config: Configuration = config_str.to_string().try_into().unwrap();
        assert_eq!(config.cores, Some(4));
        assert!(config.use_honggfuzz);
        assert!(config.fuzz_origin);
        assert_eq!(config.max_messages_per_exec, Some(20));
        assert_eq!(config.storage_deposit_limit, Some("200000000000".into()));
        assert!(!config.verbose);
        assert!(config.show_ui);
        assert!(!config.generate_seeds);
    }

    #[test]
    fn test_try_from_string_invalid_config() {
        let invalid_config_str = r#"
            cores = "invalid"
            storage_deposit_limit = "not_a_number"
        "#;

        let result: anyhow::Result<Configuration> = invalid_config_str.to_string().try_into();
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

        let lastseed = output.join("phink").join("logs").join(LAST_SEED_FILENAME);
        assert_eq!(phink_files.path(PFiles::LastSeed), lastseed);
        assert_eq!(
            lastseed.to_str().unwrap(),
            "/tmp/phink_output/phink/logs/last_seed.phink"
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
