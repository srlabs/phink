extern crate core;

use std::error::Error;
use std::fs;

use frame_support::{
    __private::BasicExternalities,
    assert_ok,
    traits::fungible::Inspect,
    traits::fungible::Mutate,
    traits::Currency,
    traits::{OnFinalize, OnInitialize},
    weights::Weight,
};
use pallet_contracts::{
    chain_extension::SysConfig, Code, CollectEvents, Config, DebugInfo, Determinism,
};
use parity_scale_codec::{DecodeLimit, Encode, Decode};
use serde::Deserialize;
use serde_json::Value;
use sp_core::{crypto::AccountId32, storage::Storage};
use sp_io::hashing::blake2_256;
use sp_io::TestExternalities;
use sp_runtime::{traits::Hash, BuildStorage};

use crate::externalities::ExtBuilder;
use crate::runtime::RuntimeCall;
use crate::{
    contract_helper::AccountIdOf,
    runtime::{
        AccountId, AllPalletsWithSystem, BalancesConfig, BlockNumber, RuntimeGenesisConfig,
        Timestamp, SLOT_DURATION,
    },
    runtime::{Balance, Contracts, Runtime, RuntimeOrigin, System},
};

type CodeHash<T> = <T as frame_system::Config>::Hash;
type BalanceOf<T> =
<<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
type Test = Runtime;

mod contract_helper;
mod externalities;
mod ink_helper;
mod runtime;
mod wasm_helper;

pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);
pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([2u8; 32]);
pub const ENOUGH: Balance = Balance::MAX / 2;

fn main() {
    let wasm_blob: Vec<u8> =
        fs::read("/Users/kevinvalerio/Desktop/phink/sample/flipper/target/ink/flipper.wasm")
            .unwrap();

    let wasm_bytes: Vec<u8> =
        include_bytes!("/Users/kevinvalerio/Desktop/phink/sample/flipper/target/ink/flipper.wasm")
            [..]
            .to_vec();

    let flipper_specs = fs::read_to_string(
        "/Users/kevinvalerio/Desktop/phink/sample/flipper/target/ink/flipper.json",
    )
        .unwrap();

    let (genesis, contract_addr) = initialize_contract(wasm_blob, wasm_bytes);
    fuzz_contract(genesis, contract_addr, flipper_specs);
}

fn initialize_contract(wasm_blob: Vec<u8>, wasm_bytes: Vec<u8>) -> (Storage, AccountId32) {
    let mut contract_addr: AccountIdOf<Test> = AccountId32::new([42u8; 32]); // Dummy account
    let code_hash = <<Test as SysConfig>::Hashing as Hash>::hash(wasm_blob.as_slice());

    let genesis_storage: Storage = {
        let storage = RuntimeGenesisConfig {
            balances: BalancesConfig {
                balances: (0..5)
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
        let mut chain = BasicExternalities::new(storage.clone());
        chain.execute_with(|| {
            // let _ = <Test as Config>::Currency::set_balance(&ALICE, 1_000_000);

            assert_ok!(Contracts::upload_code(
                RuntimeOrigin::signed(ALICE),
                wasm_bytes,
                None,
                Determinism::Relaxed
            ));

            contract_addr = contract_helper::bare_instantiate(Code::Existing(code_hash))
                .data(hex::decode("9bae9d5e").unwrap())
                .build_and_unwrap_account_id();

            assert!(
                pallet_contracts::migration::v13::ContractInfoOf::<Test>::contains_key(
                    &contract_addr
                )
            );
        });
        chain.into_storages()
    };

    (genesis_storage, contract_addr)
}

fn fuzz_contract(
    genesis_storage: Storage,
    contract_addr: AccountIdOf<Test>,
    flipper_specs: String,
) {
    let all_selectors: Vec<[u8; 4]> = ink_helper::extract_selectors(flipper_specs);

    ziggy::fuzz!(|data: &[u8]| {
        if data.len() > 300 {
            return;
        }
        let selector_slice = u32::from_ne_bytes(data[0..4].try_into().unwrap());

        if (selector_slice) > all_selectors.len() as u32 {
            return;
        }

        let mut chain = BasicExternalities::new(genesis_storage.clone());

        let mut block: u32 = 1;

        let fuzzed_func = all_selectors[selector_slice as usize];
        println!("Fuzzed selector: {:?} ({:?})", fuzzed_func, hex::encode(fuzzed_func));
        let arguments = &data[5..];

        chain.execute_with(|| {
            Timestamp::set(RuntimeOrigin::none(), block as u64 * SLOT_DURATION).unwrap();

            let lapse: u32 = 0; //for now, we set lapse always to zero

            if lapse > 0 {
                <AllPalletsWithSystem as OnFinalize<BlockNumber>>::on_finalize(block);
                block += u32::from(lapse);
                <AllPalletsWithSystem as OnInitialize<BlockNumber>>::on_initialize(block);
                Timestamp::set(RuntimeOrigin::none(), SLOT_DURATION * block as u64).unwrap();
            }

            let full_call = {
                let args: ([u8; 4], Vec<u8>) = (fuzzed_func, arguments.encode()[1..].to_vec());
                args.encode()
            };

             println!("full_args: {:?}", full_call);

            let result = contract_helper::bare_call(contract_addr.clone())
                .debug(DebugInfo::UnsafeDebug)
                .determinism(Determinism::Relaxed)
                .data(full_call)
                .build();

            // .data([message_to_bytes!("flip").to_vec(), data.encode()].concat())

            println!("    result:     {result:?}");
        });

    });
}
