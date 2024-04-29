#![recursion_limit = "256"]

extern crate core;

use frame_support::traits::fungible::Inspect;
use pallet_contracts::Config;

use sp_core::crypto::AccountId32;
use sp_runtime::traits::StaticLookup;

use crate::{
    contract::remote::ContractBridge, contract::runtime::Runtime, fuzzer::engine::FuzzerEngine,
    fuzzer::fuzz::ZiggyFuzzer,
};
use clap::Parser;
use std::{fs, path::PathBuf};

mod contract;
mod fuzzer;

type BalanceOf<T> =
    <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
type Test = Runtime;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

/// TODO: Use Clippy
fn main() {
    let dns_wasm_bytes: Vec<u8> = fs::read("sample/dns/target/ink/dns.wasm").unwrap().to_vec();
    let dns_specs = PathBuf::from("sample/dns/target/ink/dns.json");

    let setup: ContractBridge = ContractBridge::initialize_contract(dns_wasm_bytes, dns_specs);

    let fuzzer: ZiggyFuzzer = ZiggyFuzzer::new(setup);
    fuzzer.fuzz();
}

/// This struct defines the command line arguments expected by Phink.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Path to the WASM ink! smart-contract
    #[clap(long, value_parser, required = true)]
    wasm: PathBuf,

    /// Path to the json specs file to be used
    #[clap(long, value_parser, required = true)]
    specs: PathBuf,

    /// Additional command to specify operation mode
    #[clap(subcommand)]
    command: Commands,
}

/// Commands supported by Phink
#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Starts the fuzzing process
    Fuzz,
}

fn _main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Fuzz => {
            let setup: ContractBridge = ContractBridge::initialize_contract(
                fs::read(&cli.wasm).unwrap().to_vec(),
                PathBuf::from(&cli.specs),
            );

            let fuzzer: ZiggyFuzzer = ZiggyFuzzer::new(setup);
            fuzzer.fuzz();
        }
    }
}
