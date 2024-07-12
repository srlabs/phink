#![recursion_limit = "1024"]

extern crate core;

use clap::Parser;
use serde::Deserialize;
use sp_core::{crypto::Ss58Codec, hexdisplay::AsBytesRef};
use std::env::{set_var, var};
use std::{
    io::Read,
    io::{BufRead, Write},
    path::PathBuf,
};

use crate::config::Configuration;
use crate::fuzzer::fuzz::Fuzzer;
use crate::fuzzer::fuzz::FuzzingMode::FuzzMode;
use crate::ziggy::FullConfig;
use crate::{
    fuzzer::engine::FuzzerEngine,
    instrumenter::cleaner::Cleaner,
    instrumenter::instrument::{ContractBuilder, ContractInstrumenter, Instrumenter},
};

mod config;
mod contract;
mod cover;
mod fuzzer;
mod instrumenter;
mod ziggy;

/// This struct defines the command line arguments expected by Phink.
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Phink is a command line tool for fuzzing ink! smart contracts.",
    long_about = "🐙 Phink, an ink! smart-contract property-based and coverage-guided fuzzer"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(long, short, value_parser, default_value = "config.toml")]
    config: PathBuf,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Starts the fuzzing process. Instrumentation required before!
    Fuzz {
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
    },
    /// Instrument the ink! contract, and compile it with Phink features
    Instrument {
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
    },
    /// Run all the seeds
    Run {
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
    },
    /// Remove all the temporary files under `/tmp/ink_fuzzed_*`
    Clean,
    /// Generate a coverage report, only of the harness. You won't have your contract coverage here (mainly for debugging purposes only)
    HarnessCover {
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
    },
    /// Generate a coverage report for your smart-contract
    Coverage {
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
    },
    /// Execute one seed
    Execute {
        /// Path to the file containing the input seed
        #[clap(value_parser)]
        seed: PathBuf,
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
    },
}
fn main() {
    // We execute `handle_cli_mode()` first, then re-enter into `main()`
    if var("PHINK_START_FUZZING_WITH_CONFIG").is_ok() {
        handle_ziggy();
    } else {
        handle_cli_mode();
    }
}

fn handle_ziggy() {
    println!("ℹ️ Setting AFL_FORKSRV_INIT_TMOUT to 10000000");
    set_var("AFL_FORKSRV_INIT_TMOUT", "10000000");
    println!("{:?}", var("PHINK_START_FUZZING_WITH_CONFIG"));

    let config: FullConfig =
        serde_json::from_str(var("PHINK_START_FUZZING_WITH_CONFIG").unwrap().as_str()).unwrap();

    let mut instrumenter = Instrumenter::new(config.clone().contract_path);

    Fuzzer::execute_harness(&mut instrumenter, FuzzMode(), config).unwrap();
}

fn handle_cli_mode() {
    let cli = Cli::parse();
    let config = Configuration::load_config(&cli.config);

    match cli.command {
        Commands::Instrument { contract_path } => {
            let mut engine = Instrumenter::new(contract_path.clone());
            engine.instrument().unwrap().build().unwrap();

            println!(
                "🤞 Contract {} has been instrumented, and is now compiled!",
                contract_path.display()
            );
        }
        Commands::Fuzz { contract_path } => {
            let ziggy: FullConfig = FullConfig::new(config, contract_path);
            ziggy.ziggy_fuzz().unwrap();
        }
        Commands::Run { contract_path } => {
            todo!()
        }
        Commands::Execute {
            seed: seed_path,
            contract_path,
        } => {
            todo!()
        }
        Commands::HarnessCover { contract_path } => {
            todo!()
        }
        Commands::Coverage { contract_path } => {
            todo!()
        }
        Commands::Clean => {
            Instrumenter::clean().unwrap();
        }
    }
}
