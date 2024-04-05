#![recursion_limit = "1024"]

use std::fs;

use ext::ExtBuilder;
use frame::deps::frame_system;
use frame::prelude::Weight;
use frame::testing_prelude::{assert_ok, BuildStorage};
use frame::traits::{Currency, OnGenesis};
use pallet_contracts::{Code, CollectEvents, DebugInfo};
use sp_core::crypto::AccountId32;

use crate::mocks::{Balances, Contracts, RuntimeOrigin};

type CodeHash<T> = <T as frame_system::Config>::Hash;

mod ext;
mod mocks;

pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);
pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

fn main() {
    ExtBuilder::default().build().execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 10000000000000000000 * 2);
        let contract =
            fs::read("/Users/kevinvalerio/Desktop/phink/sample/flipper/target/ink/flipper.wasm")
                .unwrap();
        let res = Contracts::bare_instantiate(
            ALICE,
            0,
            GAS_LIMIT,
            None,
            Code::Upload(contract),
            vec![1], //new(true) for Flipper
            vec![0x41, 0x41, 0x41, 0x41],
            DebugInfo::UnsafeDebug,
            CollectEvents::Skip,
        )
        .result
        .unwrap();

        println!("res.result.data: {:?}", res.result.data);

        use parity_scale_codec::Encode;
        assert_ok!(Contracts::call(
            RuntimeOrigin::signed(ALICE),
            res.account_id.clone(),
            0,
            GAS_LIMIT,
            None,
            "flip".encode(),
        ));

        //Should assert that we have flip false
    });
}
// ziggy::fuzz!(|data: &[u8]| {
//     if data.len() > 100000 {
//         return;
//     };
// });
// pub fn compile_module<T>(
// fn legacy_compile_module<T>( //for wat files
