#![recursion_limit = "1024"]

extern crate core;

use frame_support::traits::fungible::Inspect;
use pallet_contracts::Config;

use crate::fuzzer::instrument::CoverageEngine;
use crate::fuzzer::libafl::LibAFLFuzzer;
use crate::{
    contract::remote::ContractBridge, contract::runtime::Runtime, fuzzer::engine::FuzzerEngine,
};
use clap::Parser;
use sp_core::crypto::AccountId32;
use sp_runtime::traits::StaticLookup;
use std::{fs, path::PathBuf};

mod contract;
mod fuzzer;
mod utils;

/// This struct defines the command line arguments expected by Phink.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Path where the JSON and WASM files are stored
    #[clap(long, value_parser, required = true)]
    dir: PathBuf,

    /// Additional command to specify operation mode
    #[clap(subcommand)]
    command: Commands,

    /// Activate TUI mode for LibAFL
    #[clap(long)]
    ui: bool,
}

/// Commands supported by Phink
#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Starts the fuzzing process
    Fuzz,
    /// Execute one seed
    Execute,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Fuzz => {
            let dir = cli.dir.clone();
            let engine = CoverageEngine::new(dir).instrument().build();
            match fs::read(&engine.wasm_path) {
                Ok(dns_wasm_bytes) => {
                    let setup = ContractBridge::initialize_wasm(dns_wasm_bytes, engine.specs_path);
                    let fuzzer = LibAFLFuzzer::new(setup);
                    println!("Now fuzzing `{:?}` !", engine.wasm_path);
                    fuzzer.setup();
                }
                Err(e) => {
                    eprintln!("Error reading WASM file: {:?}", e);
                }
            }
        }
        Commands::Execute => {}
    }
}
