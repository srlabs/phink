#![recursion_limit = "256"]

extern crate core;

use frame_support::traits::fungible::Inspect;
use pallet_contracts::Config;

use sp_core::crypto::AccountId32;
use sp_runtime::traits::StaticLookup;

use std::fs;
use std::path::PathBuf;
use crate::contract::remote::ContractBridge;
use crate::contract::runtime::Runtime;
use crate::fuzz::engine::FuzzerEngine;
use crate::fuzz::fuzz::ZiggyFuzzer;


type BalanceOf<T> =
<<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
type Test = Runtime;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;


mod fuzz;
mod contract;

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

fn main() {
    let dns_wasm_bytes: Vec<u8> = fs::read("sample/dns/target/ink/dns.wasm").unwrap().to_vec();
    let dns_specs = PathBuf::from("sample/dns/target/ink/dns.json");

    let setup: ContractBridge =
        ContractBridge::initialize_contract(dns_wasm_bytes, dns_specs);

    let fuzzer: ZiggyFuzzer = ZiggyFuzzer::new(setup);
    fuzzer.fuzz();
}
