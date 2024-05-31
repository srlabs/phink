#![recursion_limit = "1024"]

extern crate core;

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
#[clap(author, version, about)]
struct Cli {
    /// Path where the `lib.rs` is located
    #[clap(long, short, value_parser, required = false)]
    path: PathBuf,

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
    /// Instrument the ink! contract, and compile it with Phink features
    Instrument,
    /// Instrument and fuzz straight after
    InstrumentAndFuzz,
    /// Run all seeds
    Run,
    /// Generate a coverage
    Cover,
}

fn main() {
    env::set_var("AFL_FORKSRV_INIT_TMOUT", "10000000");

    if env::var("PHINK_FROM_ZIGGY").is_ok() {
        println!("🫢 Let's use Ziggy");
        let path =
            PathBuf::from(env::var("PHINK_CONTRACT_DIR").unwrap_or("sample/dns/".parse().unwrap()));
        let mut engine = instrument(path);

        start_fuzzer(&mut engine);
    } else {
        let cli = Cli::parse();

        match &cli.command {
            //TODO: Handle when CLI is just incorrect command, not just user doing ziggy run
            Commands::Instrument => {
                instrument(cli.path);
            }

            Commands::Fuzz => {
                let mut engine = InstrumenterEngine::new(cli.path.clone());

                start_cargo_ziggy_fuzz_process();

                if env::var("PHINK_START_FUZZING").is_ok() {
                    start_fuzzer(&mut engine);
                }
            }

            Commands::InstrumentAndFuzz => {
                let mut engine = instrument(cli.path);

                start_cargo_ziggy_fuzz_process();

                if env::var("PHINK_START_FUZZING").is_ok() {
                    start_fuzzer(&mut engine);
                }
            }
            Commands::Execute => {
                todo!();
            }

            Commands::Run => {
                let mut engine = instrument(cli.path);

                start_cargo_ziggy_run_process();

                if env::var("PHINK_START_FUZZING").is_ok() {
                    start_fuzzer(&mut engine);
                }
            }
            Commands::Cover => {
                todo!();
            }
        };
    }
    // // Can't use the CLI, it might be a direct Ziggy run
    // Err(error) => {
    //     // return;
    //     println!("🫢 You probably used Ziggy directly in CLI, right ?");
    //     println!("{:?}", error);
    //     let mut engine = InstrumenterEngine::new(PathBuf::from(
    //         env::var("PHINK_CONTRACT_DIR").unwrap_or("sample/dns/".parse().unwrap()),
    //     ));
    //
    //     start_fuzzer(&mut engine);
    // }
    // };
}

fn start_cargo_ziggy_fuzz_process() {
    let mut child = Command::new("cargo")
        .arg("ziggy")
        .arg("fuzz")
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

fn start_cargo_ziggy_run_process() {
    let mut child = Command::new("cargo")
        .arg("ziggy")
        .arg("run")
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
