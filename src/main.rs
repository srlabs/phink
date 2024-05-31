#![recursion_limit = "1024"]

extern crate core;

use env::{set_var, var};
use std::io::BufRead;
use std::process::{Command, Stdio};
use std::{env, fs, path::PathBuf};

use clap::Parser;
use sp_core::crypto::AccountId32;

use crate::fuzzer::parser::{MAX_SEED_LEN, MIN_SEED_LEN};
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
#[clap(
    author,
    version,
    about = "Phink is a command line tool for fuzzing ink! smart contracts.",
    long_about = "🐙 Phink, a ink! smart-contract property-based and coverage-guided fuzzer\n\n\
    Phink depends on various environment variables:

    \tPHINK_FROM_ZIGGY : Informs the tooling that the binary is being ran with Ziggy, and not directly from the CLI
    \tPHINK_CONTRACT_DIR : Location of the contract code-base. Can be automatically detected.
    \tPHINK_START_FUZZING : Tells the harness to start fuzzing. \n"
)]

struct Cli {
    /// Path where the `lib.rs` is located
    #[clap(long, short, value_parser)]
    path: Option<PathBuf>,

    /// Additional command to specify operation mode
    #[clap(subcommand)]
    command: Commands,
}

/// Commands supported by Phink
#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Starts the fuzzing process
    Fuzz,
    /// Execute one seed, currently in TODO!
    Execute,
    /// Instrument the ink! contract, and compile it with Phink features
    Instrument,
    /// Instrument and fuzz straight after
    InstrumentAndFuzz,
    /// Run all seeds
    Run,
    /// Generate a coverage, currently in TODO!
    Cover,
    /// Remove all the temporary files under `/tmp/ink_fuzzed_XXXX`
    Clean,
}

fn main() {

    if var("PHINK_FROM_ZIGGY").is_ok() {
        println!("ℹ️ Setting AFL_FORKSRV_INIT_TMOUT to 10000000");
        set_var("AFL_FORKSRV_INIT_TMOUT", "10000000");

        let path = var("PHINK_CONTRACT_DIR").map(PathBuf::from).expect(
            "\n🈲️ PHINK_CONTRACT_DIR is not set. \
                You can set it manually, it should contain the source code of your contract, \
                with or without the instrumented binary,\
                depending your options. \n\n",
        );

        let mut engine = instrument(path);

        start_fuzzer(&mut engine);
    } else {
        let cli = Cli::parse();

        match &cli.command {
            Commands::Instrument => {
                set_var("PHINK_CONTRACT_DIR", cli.path.unwrap());
                let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());
                instrument(contract_dir);
            }

            Commands::Fuzz => {
                set_var("PHINK_CONTRACT_DIR", cli.path.unwrap());
                let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());
                let mut engine = InstrumenterEngine::new(contract_dir);

                start_cargo_ziggy_fuzz_process(engine.clone().contract_dir);

                if var("PHINK_START_FUZZING").is_ok() {
                    start_fuzzer(&mut engine);
                }
            }

            Commands::InstrumentAndFuzz => {
                set_var("PHINK_CONTRACT_DIR", cli.path.unwrap());
                let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());
                let mut engine = instrument(contract_dir);

                start_cargo_ziggy_fuzz_process(engine.clone().contract_dir);

                if var("PHINK_START_FUZZING").is_ok() {
                    start_fuzzer(&mut engine);
                }
            }

            Commands::Run => {
                set_var("PHINK_CONTRACT_DIR", cli.path.unwrap());
                let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());
                let engine = instrument(contract_dir);
                start_cargo_ziggy_run_process(engine.contract_dir);
            }

            Commands::Execute => {
                todo!();
            }

            Commands::Cover => {
                todo!();
            }
            Commands::Clean => {
                InstrumenterEngine::clean().expect("🧼 Cannot execute the cleaning properly.");
            }
        };
    }
}

fn start_cargo_ziggy_fuzz_process(contract_dir: PathBuf) {
    let mut child = Command::new("cargo")
        .arg("ziggy")
        .arg("fuzz")
        .env("PHINK_CONTRACT_DIR", contract_dir)
        .env("PHINK_FROM_ZIGGY", "true")
        .env("PHINK_START_FUZZING", "true")
        .arg(format!("-g={}", MIN_SEED_LEN))
        .arg(format!("-G={}", MAX_SEED_LEN))
        .arg("--dict=./output/phink/selectors.dict")
        .stdout(Stdio::piped())
        .spawn()
        .expect("🙅 Failed to execute cargo ziggy fuzz...");

    if let Some(stdout) = child.stdout.take() {
        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    }

    let status = child.wait().expect("🙅 Failed to wait on child process...");
    if !status.success() {
        eprintln!("🙅 Command executed with failing error code");
    }
}

fn start_cargo_ziggy_run_process(contract_dir: PathBuf) {
    let mut child = Command::new("cargo")
        .arg("ziggy")
        .arg("run")
        .env("PHINK_CONTRACT_DIR", contract_dir)
        .env("PHINK_FROM_ZIGGY", "true")
        .env("PHINK_START_FUZZING", "true")
        .stdout(Stdio::piped())
        .spawn()
        .expect("🙅 Failed to execute cargo ziggy fuzz...");

    if let Some(stdout) = child.stdout.take() {
        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    }

    let status = child.wait().expect("🙅 Failed to wait on child process...");
    if !status.success() {
        eprintln!("🙅 Command executed with failing error code");
    }
}

fn instrument(path: PathBuf) -> InstrumenterEngine {
    let mut engine = InstrumenterEngine::new(path.clone());

    engine
        .instrument()
        .expect("🙅 Custom instrumentation failed")
        .build()
        .expect("🙅 Compilation with Phink features failed");

    println!(
        "🤞 Contract {:?} has been instrumented, and is now compiled!",
        path
    );

    engine
}

fn start_fuzzer(engine: &mut InstrumenterEngine) {
    let origin: AccountId32 = AccountId32::new([1; 32]);

    let finder = engine.find().unwrap();

    match fs::read(&finder.wasm_path) {
        Ok(wasm) => {
            let setup = ContractBridge::initialize_wasm(wasm, &finder.specs_path, origin);
            let fuzzer = Fuzzer::new(setup);
            fuzzer.fuzz();
        }
        Err(e) => {
            eprintln!("🙅 Error reading WASM file. {:?}", e);
        }
    }
}
