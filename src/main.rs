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
    about = "🐙 Phink: An ink! smart-contract property-based and coverage-guided fuzzer",
    long_about = None
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
    Fuzz(ContractPath),
    /// Instrument the ink! contract, and compile it with Phink features
    Instrument(ContractPath),
    /// Run all the seeds
    Run(ContractPath),
    /// Remove all the temporary files under /tmp/ink_fuzzed_*
    Clean,
    /// Generate a coverage report, only of the harness. You won't have your contract coverage here (mainly for debugging purposes only)
    HarnessCover(ContractPath),
    /// Generate a coverage report for your smart-contract
    Coverage(ContractPath),
    /// Execute one seed
    Execute {
        /// Seed to be executed
        seed: PathBuf,
        /// Path where the contract is located. It must be the root directory of the contract
        contract_path: PathBuf,
    },
}

#[derive(clap::Args, Debug)]
struct ContractPath {
    /// Path where the contract is located. It must be the root directory of the contract
    #[clap(value_parser)]
    contract_path: PathBuf,
}

fn main() {
    /// We execute `handle_cli()` first, then re-enter into `main()`
    if let Ok(config_str) = var("PHINK_START_FUZZING_WITH_CONFIG") {
        println!("ℹ️ Setting AFL_FORKSRV_INIT_TMOUT to 10000000");
        set_var("AFL_FORKSRV_INIT_TMOUT", "10000000");

        let config: ZiggyConfig = ZiggyConfig::parse(config_str);
        Fuzzer::execute_harness(Fuzz, config).unwrap();
    } else {
        handle_cli();
    }
}

fn handle_cli() {
    let cli = Cli::parse();
    let config = Configuration::load_config(&cli.config);

    match cli.command {
        Commands::Instrument(ContractPath) => {
            let mut engine = Instrumenter::new(ContractPath.contract_path.clone());
            engine.instrument().unwrap().build().unwrap();

            println!(
                "🤞 Contract {} has been instrumented and compiled!",
                ContractPath.contract_path.display()
            );
        }
        Commands::Fuzz(ContractPath) => {
            ZiggyConfig::new(config, ContractPath.contract_path)
                .ziggy_fuzz()
                .unwrap();
        }
        Commands::Run(ContractPath) => {
            ZiggyConfig::new(config, ContractPath.contract_path)
                .ziggy_run()
                .unwrap();
        }
        Commands::Execute {
            seed,
            contract_path,
        } => {
            let ziggy: ZiggyConfig = ZiggyConfig::new(config, contract_path);
            Fuzzer::execute_harness(ExecuteOneInput(seed), ziggy).unwrap();
        }
        Commands::HarnessCover(ContractPath) => {
            ZiggyConfig::new(config, ContractPath.contract_path)
                .ziggy_cover()
                .unwrap();
        }
        Commands::Coverage(ContractPath) => {
            CoverageTracker::generate(ZiggyConfig::new(config, ContractPath.contract_path));
        }
        Commands::Clean => {
            Instrumenter::clean().unwrap();
        }
    }
}
