#![recursion_limit = "1024"]

extern crate core;

use std::process::Command;
use std::{env, fs, path::PathBuf};

use clap::Parser;

use crate::{
    contract::remote::ContractBridge,
    fuzzer::engine::FuzzerEngine,
    fuzzer::fuzz::Fuzzer,
    fuzzer::instrument::{ContractBuilder, ContractInstrumenter, CoverageEngine},
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
    old_main();
    // new_main();
}

fn old_main() {
    let mut engine = match env::var("PHINK_CONTRACT_DIR") {
        Ok(folder) => CoverageEngine::new(PathBuf::from(folder)).find().unwrap(),
        Err(_) => CoverageEngine::new(PathBuf::from("sample/dns"))
            .instrument()
            .unwrap()
            .build()
            .unwrap(),
    };

    // let output = Command::new("cargo")
    //     .arg("ziggy")
    //     .arg("run")
    //     .output()
    //     .expect("Failed to execute command");

    match fs::read(&engine.wasm_path) {
        Ok(dns_wasm_bytes) => {
            let setup = ContractBridge::initialize_wasm(dns_wasm_bytes, engine.specs_path);
            let fuzzer = Fuzzer::new(setup);
            println!("Now fuzzing `{:?}` !", engine.wasm_path);
            fuzzer.fuzz();
        }
        Err(e) => {
            eprintln!("Error reading WASM file: {:?}", e);
        }
    }
}
fn new_main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Fuzz => {
            let dir = cli.dir.clone();
            let mut engine = CoverageEngine::new(dir)
                .instrument()
                .unwrap()
                .build()
                .unwrap();
            match fs::read(&engine.wasm_path) {
                Ok(dns_wasm_bytes) => {
                    let setup = ContractBridge::initialize_wasm(dns_wasm_bytes, engine.specs_path);
                    let fuzzer = Fuzzer::new(setup);
                    println!("Now fuzzing `{:?}` !", engine.wasm_path);
                    fuzzer.fuzz();
                }
                Err(e) => {
                    eprintln!("Error reading WASM file: {:?}", e);
                }
            }
        }
        Commands::Execute => {}
    }
}
