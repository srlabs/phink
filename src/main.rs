#![recursion_limit = "1024"]

extern crate core;

use clap::Parser;
use std::{
    env::{set_var, var},
    path::PathBuf,
};

use crate::{
    cli::config::Configuration,
    cli::ziggy::ZiggyConfig,
    cover::report::CoverageTracker,
    fuzzer::fuzz::Fuzzer,
    fuzzer::fuzz::FuzzingMode::ExecuteOneInput,
    fuzzer::fuzz::FuzzingMode::Fuzz,
    instrumenter::cleaner::Cleaner,
    instrumenter::instrument::{ContractBuilder, ContractInstrumenter, Instrumenter},
};

mod cli;
mod contract;
mod cover;
mod fuzzer;
mod instrumenter;

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
    let config_var = var("PHINK_START_FUZZING_WITH_CONFIG");
    // We execute handle_cli_mode() first, then re-enter into main()
    if let Ok(config_str) = config_var {
        let config: ZiggyConfig = serde_json::from_str(&config_str).unwrap();
        handle_ziggy(config);
    } else {
        handle_cli_mode();
    }
}

fn handle_ziggy(config: ZiggyConfig) {
    println!("ℹ️ Setting AFL_FORKSRV_INIT_TMOUT to 10000000");
    set_var("AFL_FORKSRV_INIT_TMOUT", "10000000");

    Fuzzer::execute_harness(Fuzz, config).unwrap();
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
            let ziggy: ZiggyConfig = ZiggyConfig::new(config, contract_path);
            ziggy.ziggy_fuzz().unwrap();
        }
        Commands::Run { contract_path } => {
            let ziggy: ZiggyConfig = ZiggyConfig::new(config, contract_path);
            ziggy.ziggy_run().unwrap();
        }
        Commands::Execute {
            seed,
            contract_path,
        } => {
            let ziggy: ZiggyConfig = ZiggyConfig::new(config, contract_path);

            Fuzzer::execute_harness(ExecuteOneInput(seed), ziggy).unwrap();
        }
        Commands::HarnessCover { contract_path } => {
            let ziggy: ZiggyConfig = ZiggyConfig::new(config, contract_path);
            ziggy.ziggy_cover().unwrap();
        }
        Commands::Coverage { contract_path } => {
            CoverageTracker::generate(ZiggyConfig::new(config, contract_path));
        }
        Commands::Clean => {
            Instrumenter::clean().unwrap();
        }
    }
}
