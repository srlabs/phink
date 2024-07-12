#![recursion_limit = "1024"]

extern crate core;

use std::{
    io::Read,
    io::{BufRead, Write},
    path::PathBuf,
};

use clap::Parser;
use serde::Deserialize;
use sp_core::{crypto::Ss58Codec, hexdisplay::AsBytesRef};

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
    Fuzz,
    /// Instrument the ink! contract, and compile it with Phink features
    Instrument,
    /// Run all the seeds
    Run,
    /// Remove all the temporary files under `/tmp/ink_fuzzed_*`
    Clean,
    /// Generate a coverage report, only of the harness. You won't have your contract coverage here (mainly for debugging purposes only)
    HarnessCover,
    /// Generate a coverage report for your smart-contract
    Coverage,
    /// Execute one seed
    Execute {
        /// Path to the file containing the input seed
        #[clap(value_parser)]
        seed: PathBuf,
    },
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = config::Configuration::load_config(&cli.config)?;

    // Only one env variable needed : config file

    match cli.command {
        Commands::Instrument => {
            let mut engine = Instrumenter::new(config.contract_path.clone());
            engine.instrument()?.build()?;

            println!(
                "🤞 Contract {} has been instrumented, and is now compiled!",
                config.contract_path.display()
            );
        }
        Commands::Fuzz => {
            let ziggy: Ziggy = Ziggy::new(config);
            ziggy.ziggy_fuzz()?;
        }
        Commands::Run => {
            todo!()
        }
        Commands::Execute { seed: seed_path } => {
            todo!()
        }
        Commands::HarnessCover => {
            todo!()
        }
        Commands::Coverage => {
            todo!()
        }
        Commands::Clean => {
            Instrumenter::clean()?;
        }
    }

    Ok(())
}
