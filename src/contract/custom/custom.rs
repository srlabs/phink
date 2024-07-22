use crate::contract::{
    custom::{
        DevelopperPreferences,
        Preferences,
    },
    runtime::{
        BalancesConfig,
        Contracts,
        RuntimeGenesisConfig,
    },
};
use pallet_contracts::Determinism;
use sp_core::{
    crypto::AccountId32,
    storage::Storage,
};
use sp_runtime::BuildStorage;
use std::fs;

/// This file is made to be customized
/// Feel free to remove, add, modify code :)
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

    /// TODO! Only for test purposes, this will crash necessarily :)
    /// We want for our test case to upload other contracts
    /// Most of the time, you might want this function to be empty
    fn on_contract_initialize() {
        fn on_contract_initialize() {
            let ink_fuzzed_path: &str = "ink_fuzzed_Cw0g";

            let adder = Contracts::bare_upload_code(
                AccountId32::new([1; 32]),
                match fs::read(format!("{}/target/ink/adder/adder.wasm", ink_fuzzed_path))
                {
                    Ok(data) => data.to_owned(),
                    Err(_) => {
                        println!("❌ Error reading adder wasm file");
                        return;
                    }
                },
                None,
                Determinism::Enforced,
            );

            match adder {
                Ok(code) => println!("ℹ️ Adder hash: {:?}", code.code_hash),
                Err(_) => println!("❌ Error uploading adder code"),
            }

            let accumulator = Contracts::bare_upload_code(
                AccountId32::new([1; 32]),
                match fs::read(format!(
                    "{}/target/ink/accumulator/accumulator.wasm",
                    ink_fuzzed_path
                )) {
                    Ok(data) => data.to_owned(),
                    Err(_) => {
                        println!("❌ Error reading accumulator wasm file");
                        return;
                    }
                },
                None,
                Determinism::Enforced,
            );

            match accumulator {
                Ok(code) => println!("ℹ️ Accumulator hash: {:?}", code.code_hash),
                Err(_) => println!("❌ Error uploading accumulator code"),
            }

            let subber = Contracts::bare_upload_code(
                AccountId32::new([1; 32]),
                match fs::read(format!(
                    "{}/target/ink/subber/subber.wasm",
                    ink_fuzzed_path
                )) {
                    Ok(data) => data.to_owned(),
                    Err(_) => {
                        println!("❌ Error reading subber wasm file");
                        return;
                    }
                },
                None,
                Determinism::Enforced,
            );

            match subber {
                Ok(code) => println!("ℹ️ Subber hash: {:?}", code.code_hash),
                Err(_) => println!("❌ Error uploading subber code"),
            }
        }
    }
}
