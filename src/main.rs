#![recursion_limit = "1024"]

use std::fs;

use ext::ExtBuilder;
use frame::deps::frame_support::__private::BasicExternalities;
use frame::deps::frame_system;

use frame::prelude::Weight;
use frame::primitives::Hash;
use frame::testing_prelude::{assert_ok, BuildStorage};
use frame::traits::{Currency, Dispatchable, OnFinalize, OnGenesis, OnInitialize};
use pallet_contracts::{
    Code, CollectEvents, Config, DebugInfo, Determinism, InstantiateReturnValue,
};
use parity_scale_codec::DecodeLimit;
use parity_scale_codec::Encode;
use sp_core::crypto::AccountId32;
use sp_core::storage::Storage;

use crate::mocks::{
    AccountId, AllPalletsWithSystem, Balance, Balances, BalancesConfig, BlockNumber, Contracts,
    Runtime, RuntimeCall, RuntimeGenesisConfig, RuntimeOrigin, Timestamp, SLOT_DURATION,
};

type CodeHash<T> = <T as frame_system::Config>::Hash;

mod ext;
mod mocks;
mod contract_kevin;

pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);
pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const ENOUGH: Balance = Balance::MAX / 2;

fn main() {
    let accounts: Vec<AccountId> = (0..5).map(|i| [i; 32].into()).collect();
    let contract =
        fs::read("/Users/kevinvalerio/Desktop/phink/sample/flipper/target/ink/flipper.wasm")
            .unwrap();
    let mut contract_id: AccountId32 = AccountId32::new([0x42; 32]);
    // let (wasm, code_hash) =
    //     compile_wat::<Runtime>(include_bytes!("../tests/fixtures/event_size.wat")).unwrap();

    let genesis_storage: Storage = {
        let storage = RuntimeGenesisConfig {
            balances: BalancesConfig {
                balances: accounts
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
            // old_contract_call(contract, &mut contract_id);

          let toz =  Contracts::instantiate_with_code(RuntimeOrigin::from(ALICE), ENOUGH, GAS_LIMIT, None, wasm, vec![], vec![])
                .unwrap();
            // let addr = Contracts::contract_address(&ALICE, &code_hash, &[]);

            Contracts::call(
                ALICE,
                toz.,
                0,
                GAS_LIMIT * 2,
                None,
                <Runtime as Config>::Schedule::get()
                    .limits
                    .payload_len
                    .encode(),
            )
                .unwrap();
        });
        chain.into_storages()
    };

    fuzz(&contract_id, &genesis_storage);
}

fn old_contract_call(contract: Vec<u8>, mut contract_id: &mut AccountId32) {
    let upload = Contracts::bare_upload_code(ALICE, contract, None, Determinism::Enforced).unwrap();
    contract_id = Contracts::bare_instantiate(
        ALICE,
        0,
        GAS_LIMIT,
        None,
        Code::Existing(upload.code_hash),
        vec![],
        vec![],
        DebugInfo::UnsafeDebug,
        CollectEvents::Skip,
    )
        .result
        .unwrap()
        .account_id;

    println!("contract_id: {:#?}", contract_id);
    println!("upload: {:#?}", upload);

    let toz = Contracts::call(
        RuntimeOrigin::signed(ALICE),
        contract_id.clone(),
        0,
        GAS_LIMIT,
        None,
        vec![0x0],
    );
    println!("call == {:?}", toz);

    println!("---------------");
}

fn fuzz(contract_id: &AccountId32, genesis_storage: &Storage) {
    ziggy::fuzz!(|data: &[u8]| {
        let mut chain = BasicExternalities::new(genesis_storage.clone());
        let mut block: u32 = 1;

        chain.execute_with(|| {
            let mut lapse: u32 = 1;
            Timestamp::set(RuntimeOrigin::none(), block as u64 * SLOT_DURATION).unwrap();

            <AllPalletsWithSystem as OnFinalize<BlockNumber>>::on_finalize(block);
            block += u32::from(lapse);
            <AllPalletsWithSystem as OnInitialize<BlockNumber>>::on_initialize(block);
            Timestamp::set(RuntimeOrigin::none(), SLOT_DURATION * block as u64).unwrap();

            let toz = Contracts::call(
                RuntimeOrigin::signed(ALICE),
                contract_id.clone(),
                0,
                GAS_LIMIT,
                None,
                Vec::from(data),
            );
            #[cfg(not(fuzzing))]
            println!("    call:       {toz:?}");
            lapse += 1;
        });
    });
}

pub fn compile_wat<T>(wat_bytes: &[u8]) -> wat::Result<(Vec<u8>, <T::Hashing as Hash>::Output)>
    where
        T: frame_system::Config,
{
    let wasm_binary = wat::parse_bytes(wat_bytes)?.into_owned();
    let code_hash = T::Hashing::hash(&wasm_binary);
    Ok((wasm_binary, code_hash))
}
