#![recursion_limit = "1024"]

extern crate core;

use frame_support::traits::fungible::Inspect;
use pallet_contracts::Config;

use crate::fuzzer::instrument::CoverageEngine;
use crate::fuzzer::libafl::LibAFLFuzzer;
use crate::{
    contract::remote::ContractBridge, contract::runtime::Runtime, fuzzer::engine::FuzzerEngine,
    fuzzer::ziggy::ZiggyFuzzer,
};
use clap::Parser;
use sp_core::crypto::AccountId32;
use sp_runtime::traits::StaticLookup;
use std::{fs, path::PathBuf};

mod contract;
mod fuzzer;

/// TODO: Use Clippy
fn main() {
    let dir = PathBuf::from("sample/dns");

    let engine = CoverageEngine::new(dir).instrument().build();

    let dns_wasm_bytes: Vec<u8> = fs::read(engine.wasm_path.clone()).unwrap().to_vec();
    println!("{:?}", engine.wasm_path);

    let setup: ContractBridge = ContractBridge::initialize_wasm(dns_wasm_bytes, engine.specs_path);
    let fuzzer: LibAFLFuzzer = LibAFLFuzzer::new(setup);

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
            let setup: ContractBridge = ContractBridge::initialize_wasm(
                fs::read(&cli.wasm).unwrap().to_vec(),
                PathBuf::from(&cli.specs),
            );

            let fuzzer: ZiggyFuzzer = ZiggyFuzzer::new(setup);
            fuzzer.fuzz();
        }
    }
}
