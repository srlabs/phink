// #![recursion_limit = "256"]
//
// extern crate core;
//
// use frame_support::traits::fungible::Inspect;
// use pallet_contracts::Config;
//
// use sp_core::crypto::AccountId32;
// use sp_runtime::traits::StaticLookup;
//
// use sp_core::H256;
// use std::{fs, path::PathBuf};
//
// use crate::{fuzzer::ZiggyContractFuzer, remote::ContractBridge, runtime::Runtime};
// use clap::{ArgGroup, Parser};
// use crate::fuzzer_engine::FuzzerEngine;
//
// type BalanceOf<T> =
// <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
// type Test = Runtime;
// type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
//
// mod coverage;
// mod fuzzer;
// mod invariants;
// mod payload;
// mod remote;
// mod runtime;
// mod fuzzer_engine;
//
// pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
//
// /// This struct defines the command line arguments expected by Phink.
// #[derive(Parser, Debug)]
// #[clap(author, version, about)]
// struct Cli {
//     /// Path to the WASM ink! smart-contract
//     #[clap(long, value_parser, required = true)]
//     wasm: PathBuf,
//
//     /// Path to the json specs file to be used
//     #[clap(long, value_parser, required = true)]
//     specs: PathBuf,
//
//     /// Additional command to specify operation mode
//     #[clap(subcommand)]
//     command: Commands,
// }
//
// /// Commands supported by Phink
// #[derive(clap::Subcommand, Debug)]
// enum Commands {
//     /// Starts the fuzzing process
//     Fuzz,
// }
//
// // --specs="sample/dns/target/ink/dns.json"  --wasm="/Users/kevinvalerio/Desktop/phink/sample/dns/target/ink/instrumented-module.wasm"
//
// fn main() {
//     let cli = Cli::parse();
//     match &cli.command {
//         Commands::Fuzz => {
//             //TODO! not include_bytes!() ?
//             let wasm_bytes = fs::read(&cli.wasm).expect("Failed to read WASM ink! smart-contract");
//             let specs_path = cli.specs;
//             let specs = fs::read_to_string(specs_path).expect("Failed to read JSON specs file");
//
//             let setup = ContractBridge::initialize_contract(wasm_bytes, specs);
//             let fuzzer = ZiggyContractFuzer::new(setup);
//             fuzzer.fuzz();
//         }
//     }
// }
