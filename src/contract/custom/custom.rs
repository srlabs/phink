use crate::{
    contract::{
        custom::preferences::{
            DevelopperPreferences,
            Preferences,
        },
        runtime::{
            AccountId,
            BalancesConfig,
            Contracts,
            Runtime,
            RuntimeGenesisConfig,
            RuntimeOrigin,
        },
    },
    EmptyResult,
};
use anyhow::{
    bail,
    Context,
};
use pallet_contracts::Determinism;
use sp_core::{
    crypto::AccountId32,
    storage::Storage,
};
use sp_runtime::BuildStorage;
use std::{
    fs,
    str::FromStr,
};

/// This file is made to be customized. Feel free to remove, add, modify code
impl DevelopperPreferences for Preferences {
    fn runtime_storage() -> Storage {
        let specific_address =
            AccountId32::from_str("5CJwK57RASwYZexBDvxoybV7BoRTbGtxckrWKbtpBug35yx2").unwrap();
        let mut balances = (0..u8::MAX)
            .map(|i| ([i; 32].into(), 10000000000000000000 * 2))
            .collect::<Vec<_>>();

        balances.push((specific_address.into(), 10000000000000000000 * 2));

        let storage = RuntimeGenesisConfig {
            balances: BalancesConfig { balances },
            ..Default::default()
        }
        .build_storage()
        .unwrap();
        storage
    }

    /// We want for our test case to upload other contracts
    /// Most of the time, you might want this function to be empty
    fn on_contract_initialize() -> EmptyResult {
        let ink_fuzzed_path: &str = "/tmp/ink_fuzzed_UfY2T";

        let adder = Contracts::bare_upload_code(
            AccountId32::new([1; 32]),
            fs::read(format!("{ink_fuzzed_path}/target/ink/adder/adder.wasm"))
                .with_context(|| "❌ Error reading adder wasm file")?,
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
            ))
            .with_context(|| "❌ Error reading accumulator wasm file")?,
            None,
            Determinism::Enforced,
        );

        match accumulator {
            Ok(code) => println!("ℹ️ Accumulator hash: {:?}", code.code_hash),
            Err(_) => bail!("❌ Error uploading accumulator code"),
        }

        let subber = Contracts::bare_upload_code(
            AccountId32::new([1; 32]),
            fs::read(format!("{ink_fuzzed_path}/target/ink/subber/subber.wasm"))
                .with_context(|| "❌ Error reading accumulator wasm file")?,
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
