extern crate core;

use frame_support::{
    traits::fungible::Inspect,
    traits::fungible::Mutate,
    traits::Currency,
    traits::{OnFinalize, OnInitialize},
    weights::Weight,
};
use pallet_contracts::{chain_extension::SysConfig, Config};
use parity_scale_codec::{Decode, DecodeLimit, Encode};
use serde::Deserialize;
use sp_core::crypto::AccountId32;
use sp_runtime::traits::StaticLookup;
use sp_runtime::{traits::Hash, BuildStorage};
use std::error::Error;
use std::fs;

use crate::deploy::DeployedSetup;
use crate::fuzzer::ContractFuzzer;
use crate::{
    runtime::{AllPalletsWithSystem, BlockNumber, Timestamp, SLOT_DURATION},
    runtime::{Balance, Runtime, RuntimeOrigin},
};

type CodeHash<T> = <T as frame_system::Config>::Hash;
type BalanceOf<T> =
    <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
type Test = Runtime;
type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

mod deploy;
mod fuzzer;
mod payload;
mod runtime;

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

fn main() {
    let dns_wasm_bytes: Vec<u8> = include_bytes!(
        "\
        /Users/kevinvalerio/Desktop/phink/sample/dns/target/ink/dns.wasm\
        "
    )[..]
        .to_vec(); //full path is required for this damn macro...

    let dns_specs = fs::read_to_string("sample/dns/target/ink/dns.json").unwrap();

    let setup: DeployedSetup =
        DeployedSetup::initialize_contract(dns_wasm_bytes, dns_specs.clone());

    let fuzzer: ContractFuzzer = ContractFuzzer::new(setup);
    fuzzer.fuzz();
}

// On each iteration, we check if all stats are good.
fn check_invariants() {
    println!("WE PASSED!!!!!");
}
