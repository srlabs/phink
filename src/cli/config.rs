use crate::cli::config::OriginFuzzingOption::{DisableOriginFuzzing, EnableOriginFuzzing};
use crate::contract::remote::{BalanceOf, ContractBridge, Test};
use crate::fuzzer::fuzz::MAX_MESSAGES_PER_EXEC;
use frame_support::weights::Weight;
use serde_derive::{Deserialize, Serialize};
use sp_core::crypto::AccountId32;
use std::fs;
use std::path::PathBuf;

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
    /// Fuzz the origin. If `false`, the fuzzer will execute each message with the same account.
    pub fuzz_origin: bool,  
    /// The gas limit enforced when executing the constructor
    pub default_gas_limit: Option<Weight>,
    /// The maximum amount of balance that can be charged from the caller to pay for the storage consumed.
    pub storage_deposit_limit: Option<BalanceOf<Test>>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            cores: Some(1),
            use_honggfuzz: false,
            deployer_address: ContractBridge::DEFAULT_DEPLOYER.into(),
            max_messages_per_exec: MAX_MESSAGES_PER_EXEC.into(),
            report_path: Some(PathBuf::from("output/coverage_report")),
            fuzz_origin: false,
            default_gas_limit: Option::from(ContractBridge::DEFAULT_GAS_LIMIT),
            storage_deposit_limit: None,
        }
    }
}
#[derive(Clone, Debug)]
pub enum OriginFuzzingOption {
    EnableOriginFuzzing,
    DisableOriginFuzzing,
}

impl Default for OriginFuzzingOption {
    fn default() -> Self {
        DisableOriginFuzzing
    }
}

impl Configuration {
    pub fn should_fuzz_origin(&self) -> OriginFuzzingOption {
        match self.fuzz_origin {
            true => EnableOriginFuzzing,
            false => DisableOriginFuzzing,
        }
    }

    pub fn load_config(file_path: &PathBuf) -> Configuration {
        let config_str = fs::read_to_string(file_path).unwrap_or_else(|err| {
            panic!("🚫 Can't read config: {}", err);
        });

        toml::from_str(&config_str).unwrap_or_else(|err| {
            panic!("❌ Can't parse config: {}", err);
        })
    }
}
