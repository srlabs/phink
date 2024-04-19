extern crate core;

use frame_support::traits::fungible::Inspect;
use pallet_contracts::Config;

use sp_core::crypto::AccountId32;
use sp_runtime::traits::StaticLookup;

use sp_core::H256;
use std::fs;

use crate::deploy::ContractBridge;
use crate::fuzzer::ContractFuzzer;
use crate::runtime::Runtime;

type BalanceOf<T> =
    <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
type Test = Runtime;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

mod deploy;
mod fuzzer;
mod invariants;
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

    let setup: ContractBridge =
        ContractBridge::initialize_contract(dns_wasm_bytes, dns_specs.clone());

    let fuzzer: ContractFuzzer = ContractFuzzer::new(setup);
    fuzzer.fuzz();
}
