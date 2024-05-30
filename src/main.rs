#![recursion_limit = "1024"]

extern crate core;

use std::process::Command;
use std::{env, fs, path::PathBuf};

use clap::Parser;

use crate::fuzzer::instrument::InkFilesPath;
use crate::{
    contract::remote::ContractBridge,
    fuzzer::engine::FuzzerEngine,
    fuzzer::fuzz::Fuzzer,
    fuzzer::instrument::{ContractBuilder, ContractInstrumenter, InstrumenterEngine},
};

mod contract;
mod fuzzer;
mod utils;

/// This struct defines the command line arguments expected by Phink.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Path where the JSON and WASM files are stored
    #[clap(long, value_parser)] //TODO: fix the required to true
    dir: PathBuf,

    /// Additional command to specify operation mode
    #[clap(subcommand)]
    command: Commands,
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
    handle_with_env();
    // new_main();
}

fn handle_with_env() {
    // If PHINK_CONTRACT_DIR is passed, this will be our contract location, sample/dns otherwise
    let contract_path =
        env::var("PHINK_CONTRACT_DIR").unwrap_or_else(|_| String::from("sample/dns/"));

    let engine = instrument_and_compile(contract_path);
    let finder = &engine.find().unwrap();

    deploy_and_fuzz(finder);
}

fn instrument_and_compile(contract_path: String) -> InstrumenterEngine {
    let mut engine = InstrumenterEngine::new(PathBuf::from(contract_path));

    if env::var("PHINK_INSTRUMENT_AND_BUILD").is_ok() {
        engine.instrument().unwrap().build().unwrap();
    }
    engine
}

fn deploy_and_fuzz(finder: &InkFilesPath) {
    match fs::read(&finder.wasm_path) {
        Ok(dns_wasm_bytes) => {
            let setup = ContractBridge::initialize_wasm(dns_wasm_bytes, &finder.specs_path);
            let fuzzer = Fuzzer::new(setup);
            fuzzer.fuzz();
        }
        Err(e) => {
            eprintln!("Error reading WASM file: {:?}", e);
        }
    }
}

fn new_main() {
    // let output = Command::new("cargo")
    //     .arg("ziggy")
    //     .arg("run")
    //     .output()
    //     .expect("Failed to execute command"); //todo! set min input size to 10, via cli :)

    let cli = Cli::parse();
    match &cli.command {
        Commands::Fuzz => {}
        Commands::Execute => {}
    }
}
