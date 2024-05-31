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
    \tPHINK_START_FUZZING : Tells the harness to start fuzzing. \n\
    \n
    Using Ziggy `PHINK_CONTRACT_DIR=/tmp/ink_fuzzed_QEBAC/ PHINK_FROM_ZIGGY=true cargo ziggy run`"
)]

struct Cli {
    /// Path where the `lib.rs` is located
    #[clap(long, short, value_parser)]
    path: Option<PathBuf>,

    /// Number of cores to use for Ziggy
    #[clap(long, short, value_parser, default_value = "1")]
    cores: Option<u8>,

    /// Additional command to specify operation mode
    #[clap(subcommand)]
    command: Commands,
}

/// Commands supported by Phink
#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Starts the fuzzing process. Instrumentation required before!
    Fuzz,
    /// Instrument the ink! contract, and compile it with Phink features
    Instrument,
    /// Run all the seeds
    Run,
    /// Remove all the temporary files under `/tmp/ink_fuzzed_XXXX`
    Clean,
    /// Generate a coverage, only of the harness
    Cover,
    /// Execute one seed, currently in TODO!
    Execute,
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
        // Here, the contract is already instrumented

        start_fuzzer(&mut InstrumenterEngine { contract_dir: path });
    } else {
        let cli = Cli::parse();

        match &cli.command {
            Commands::Instrument => {
                let contract_dir = PathBuf::from(cli.path.unwrap());
                instrument(contract_dir);
            }

            Commands::Fuzz => {
                set_var("PHINK_CONTRACT_DIR", cli.path.unwrap());
                set_var("PHINK_CORES", cli.cores.unwrap().to_string());

                let cores: u8 = var("PHINK_CORES").map_or(1, |v| v.parse().unwrap_or(1));
                let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());
                let mut engine = InstrumenterEngine::new(contract_dir);

                start_cargo_ziggy_fuzz_process(engine.clone().contract_dir, cores);

                if var("PHINK_START_FUZZING").is_ok() {
                    start_fuzzer(&mut engine);
                }
            }

            Commands::Run => {
                set_var(
                    "PHINK_CONTRACT_DIR",
                    cli.path.expect("📂 Contract path is expected"),
                );
                let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());
                start_cargo_ziggy_command_process(contract_dir, ZiggyCommand::Run);
            }

            Commands::Execute => {
                todo!();
            }

            Commands::Cover => {
                set_var(
                    "PHINK_CONTRACT_DIR",
                    cli.path.expect("📂 Contract path is expected"),
                );
                let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());
                start_cargo_ziggy_command_process(contract_dir, ZiggyCommand::Cover);
            }
            Commands::Clean => {
                InstrumenterEngine::clean().expect("🧼 Cannot execute the cleaning properly.");
            }
        };
    }
}


pub enum ZiggyCommand {
    Run,
    Cover
}

fn start_cargo_ziggy_fuzz_process(contract_dir: PathBuf, cores: u8) {
    let mut child = Command::new("cargo")
        .arg("ziggy")
        .arg("fuzz")
        .arg("--no-honggfuzz")
        .arg(format!("--jobs={}", cores))
        .env("PHINK_CONTRACT_DIR", contract_dir)
        .env("PHINK_FROM_ZIGGY", "true")
        .env("AFL_FORKSRV_INIT_TMOUT", "10000000")
        .arg(format!("--minlength={}", MIN_SEED_LEN))
        .arg(format!("--maxlength={}", MAX_SEED_LEN))
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

fn start_cargo_ziggy_command_process(contract_dir: PathBuf, command: ZiggyCommand) {
    let command_arg = match command {
        ZiggyCommand::Run => "run",
        ZiggyCommand::Cover => "cover",
    };

    let mut child = Command::new("cargo")
        .arg("ziggy")
        .arg(command_arg)
        .env("PHINK_CONTRACT_DIR", contract_dir)
        .env("PHINK_FROM_ZIGGY", "true")
        .env("PHINK_START_FUZZING", "true")
        .stdout(Stdio::piped())
        .spawn()
        .expect("🙅 Failed to execute cargo ziggy command...");

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
