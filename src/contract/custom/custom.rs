use crate::contract::{
    custom::preferences::{
        DevelopperPreferences,
        Preferences,
    },
    runtime::{
        BalancesConfig,
        Contracts,
        RuntimeGenesisConfig,
    },
};
use anyhow::bail;
use pallet_contracts::Determinism;
use sp_core::{
    crypto::AccountId32,
    storage::Storage,
};
use sp_runtime::BuildStorage;
use std::fs;

/// This file is made to be customized. Feel free to remove, add, modify code
impl DevelopperPreferences for Preferences {
    fn runtime_storage() -> Storage {
        let storage = RuntimeGenesisConfig {
            balances: BalancesConfig {
                balances: (0..u8::MAX) // Lot of money for Alice, Bob ... Ferdie
                    .map(|i| [i; 32].into())
                    .collect::<Vec<_>>()
                    .iter()
                    .cloned()
                    .map(|k| (k, 10000000000000000000 * 2))
                    .collect(),
            },
            ..Default::default()
        }
        .build_storage()
        .unwrap();
        storage
    }

    /// TODO! Only for test purposes, this will crash necessarily
    /// We want for our test case to upload other contracts
    /// Most of the time, you might want this function to be empty
    fn on_contract_initialize() -> anyhow::Result<()> {
        let ink_fuzzed_path: &str = "/tmp/ink_fuzzed_UfY2T";

        let adder = Contracts::bare_upload_code(
            AccountId32::new([1; 32]),
            fs::read(format!("{ink_fuzzed_path}/target/ink/adder/adder.wasm"))?,
            None,
            Determinism::Enforced,
        );

        match adder {
            Ok(code) => println!("ℹ️ Adder hash: {:?}", code.code_hash),
            Err(_) => bail!("❌ Error uploading adder code"),
        }

        let accumulator = Contracts::bare_upload_code(
            AccountId32::new([1; 32]),
            fs::read(format!(
                "{ink_fuzzed_path}/target/ink/accumulator/accumulator.wasm",
            ))?,
            None,
            Determinism::Enforced,
        );

        match accumulator {
            Ok(code) => println!("ℹ️ Accumulator hash: {:?}", code.code_hash),
            Err(_) => bail!("❌ Error uploading accumulator code"),
        }

        let subber = Contracts::bare_upload_code(
            AccountId32::new([1; 32]),
            fs::read(format!("{ink_fuzzed_path}/target/ink/subber/subber.wasm"))?,
            None,
            Determinism::Enforced,
        );

        match subber {
            Ok(code) => println!("ℹ️ Subber hash: {:?}", code.code_hash),
            Err(_) => bail!("❌ Error uploading subber code"),
        }
        Ok(())
    }
}
