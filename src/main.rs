extern crate core;

use contract_transcode::ContractMessageTranscoder;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

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
use parity_scale_codec::{Decode, DecodeLimit, Encode};
use serde::Deserialize;
use serde_json::Value;
use sp_core::{crypto::AccountId32, storage::Storage};
use sp_io::hashing::blake2_256;
use sp_io::TestExternalities;
use sp_runtime::{traits::Hash, BuildStorage};

use crate::runtime::RuntimeCall;
use crate::{
    extrinsics::AccountIdOf,
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

mod extrinsics;
mod runtime;
mod selectors;

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const ENOUGH: Balance = Balance::MAX / 2;

fn main() {
    let dns_wasm: Vec<u8> = fs::read("sample/dns/target/ink/dns.wasm").unwrap();
    let dns_wasm_bytes: Vec<u8> =
        include_bytes!("/Users/kevinvalerio/Desktop/phink/sample/dns/target/ink/dns.wasm")[..]
            .to_vec(); //full path is required for this damn macro...
    let dns_specs = fs::read_to_string("sample/dns/target/ink/dns.json").unwrap();
    let dns_toml = PathBuf::from("sample/dns/Cargo.toml");

    let (genesis, contract_addr) = initialize_contract(dns_wasm, dns_wasm_bytes, dns_specs.clone());
    fuzz_contract(genesis, contract_addr, dns_specs);
}

fn initialize_contract(
    wasm_blob: Vec<u8>,
    wasm_bytes: Vec<u8>,
    json_specs: String,
) -> (Storage, AccountId32) {
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
            assert_ok!(Contracts::upload_code(
                RuntimeOrigin::signed(ALICE),
                wasm_bytes,
                None,
                Determinism::Relaxed
            ));

            let default_ctr = selectors::constructor(json_specs);

            #[cfg(not(fuzzing))]
            println!("constructor : {:?}", hex::encode(default_ctr.clone()));

            contract_addr = extrinsics::bare_instantiate(Code::Existing(code_hash))
                .data(Vec::from(default_ctr.clone()))
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

fn fuzz_contract(genesis_storage: Storage, contract_addr: AccountIdOf<Test>, json_specs: String) {
    let all_selectors: Vec<[u8; 4]> = selectors::extract_all(json_specs);
    let transcoder_loader = Arc::new(Mutex::new(
        ContractMessageTranscoder::load(Path::new("sample/dns/target/ink/dns.json")).unwrap(),
    ));

    ziggy::fuzz!(|data: &[u8]| {
        if data.len() > 400 || data.len() < 4 {
            return;
        }

        let selector_slice = u32::from_ne_bytes(data[0..4].try_into().unwrap());

        if selector_slice >= all_selectors.len() as u32 {
            return;
        }
        let fuzzed_func = all_selectors[selector_slice as usize];
        let arguments = &data[4..];
        let full_call = {
            let args: ([u8; 4], Vec<u8>) = (fuzzed_func, arguments.encode().to_vec());
            args.encode()
        };

        let decoded_msg = transcoder_loader
            .lock()
            .unwrap()
            .decode_contract_message(&mut &*full_call);
        if let Err(_) = decoded_msg {
            return;
        }

        let mut chain = BasicExternalities::new(genesis_storage.clone());
        chain.execute_with(|| {
            timestamp();

            let result = extrinsics::bare_call(contract_addr.clone())
                .debug(DebugInfo::UnsafeDebug)
                .determinism(Determinism::Relaxed)
                .data(full_call.clone())
                .build();

            check_invariants();

            #[cfg(not(fuzzing))]
            {
                println!("{} ......... {}", decoded_msg.unwrap().to_string(), hex::encode(full_call.clone()));
                println!("{result:?}\n");
            }
        });
    });
}

// On each iteration, we check if all stats are good.
fn check_invariants() {
    println!("WE PASSED!!!!!");
}

fn timestamp() {
    let mut block: u32 = 1;

    Timestamp::set(RuntimeOrigin::none(), block as u64 * SLOT_DURATION).unwrap();

    let lapse: u32 = 0; //for now, we set lapse always to zero

    if lapse > 0 {
        <AllPalletsWithSystem as OnFinalize<BlockNumber>>::on_finalize(block);
        block += u32::from(lapse);
        <AllPalletsWithSystem as OnInitialize<BlockNumber>>::on_initialize(block);
        Timestamp::set(RuntimeOrigin::none(), SLOT_DURATION * block as u64).unwrap();
    }
}
