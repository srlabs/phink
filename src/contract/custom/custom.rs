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
        let adder = Contracts::bare_upload_code(
            AccountId32::new([1; 32]),
            fs::read("/tmp/ink_fuzzed_Cw0gY/target/ink/adder/adder.wasm")
                .unwrap()
                .to_owned(),
            None,
            Determinism::Enforced,
        );
        println!("ℹ️ Adder hash: {:?}", adder.unwrap().code_hash);

        let accumulator = Contracts::bare_upload_code(
            AccountId32::new([1; 32]),
            fs::read("/tmp/ink_fuzzed_Cw0gY/target/ink/accumulator/accumulator.wasm")
                .unwrap()
                .to_owned(),
            None,
            Determinism::Enforced,
        );
        println!(
            "ℹ️ Accumulator hash: {:?}",
            accumulator.unwrap_or().code_hash
        );

        let subber = Contracts::bare_upload_code(
            AccountId32::new([1; 32]),
            fs::read("/tmp/ink_fuzzed_Cw0gY/target/ink/subber/subber.wasm")
                .unwrap()
                .to_owned(),
            None,
            Determinism::Enforced,
        );

        println!("ℹ️ Subber hash: {:?}", subber.unwrap().code_hash);
    }
}
