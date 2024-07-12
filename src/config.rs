use serde_derive::{Deserialize, Serialize};
use sp_core::crypto::AccountId32;
use std::fs;
use std::path::PathBuf;
use crate::contract::remote::ContractBridge;
use crate::fuzzer::fuzz::MAX_MESSAGES_PER_EXEC;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Configuration {
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
            cores: Some(1),
            use_honggfuzz: false,
            deployer_address: ContractBridge::DEFAULT_DEPLOYER.into(),
            max_messages_per_exec: MAX_MESSAGES_PER_EXEC.into(),
            report_path: Some(PathBuf::from("output/coverage_report")),
        }
    }
}

impl Configuration {
    pub fn load_config(file_path: &PathBuf) -> Configuration {
        let config_str = fs::read_to_string(file_path).expect("can't read config");
        toml::from_str(&config_str).expect("can't parse config")
    }
}
