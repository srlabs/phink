use serde_derive::{Deserialize, Serialize};
use sp_core::crypto::AccountId32;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Configuration {
    /// Path where the contract is located. It must be the root directory of the contract
    pub contract_path: PathBuf,
    /// Number of cores to use for Ziggy
    pub cores: Option<u8>,
    /// Also use Hongfuzz as a fuzzer
    pub use_honggfuzz: bool,
    // Origin deploying and instantiating the contract
    pub deployer_address: Option<AccountId32>,
    // Maximimum number of ink! message executed per seed
    pub max_messages_per_exec: Option<usize>,
    /// Output directory for the coverage report
    pub report_path: Option<PathBuf>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            contract_path: Default::default(),
            cores: Some(1),
            use_honggfuzz: false,
            deployer_address: None,
            max_messages_per_exec: Some(4),
            report_path: Some(PathBuf::from("output/coverage_report")),
        }
    }
}

impl Configuration {
    pub fn load_config(file_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(file_path)?;
        let config: Configuration = toml::from_str(&config_str)?;
        Ok(config)
    }
}
