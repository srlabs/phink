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

use crate::ziggy::Ziggy;
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
 fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ℹ️ Setting AFL_FORKSRV_INIT_TMOUT to 10000000");

    set_var("AFL_FORKSRV_INIT_TMOUT", "10000000");

    if var("PHINK_FROM_ZIGGY").is_ok() {
        println!("{:?}", var("PHINK_FROM_ZIGGY"));
        Ok(())
    } else {
        //We execute handle_cli_mode(), then re-enter the main() function to go the the above if
        handle_cli_mode()
    }
}

fn handle_cli_mode() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = config::Configuration::load_config(&cli.config)?;

    match cli.command {
        Commands::Instrument { contract_path } => {
            let mut engine = Instrumenter::new(contract_path.clone());
            engine.instrument()?.build()?;

            println!(
                "🤞 Contract {} has been instrumented, and is now compiled!",
                contract_path.display()
            );
        }
        Commands::Fuzz { contract_path } => {
            let ziggy: Ziggy = Ziggy::new(config, contract_path);
            ziggy.ziggy_fuzz()?;
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
            Instrumenter::clean()?;
        }
    }

    Ok(())
}
