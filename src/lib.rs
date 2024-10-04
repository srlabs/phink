#![feature(os_str_display)]
#![feature(duration_millis_float)]
#![recursion_limit = "1024"]

extern crate core;
use crate::{
    cli::{
        config::Configuration,
        env::{
            PhinkEnv,
            PhinkEnv::FromZiggy,
        },
        format_error,
        ziggy::ZiggyConfig,
    },
    cover::report::CoverageTracker,
    fuzzer::fuzz::{
        Fuzzer,
        FuzzingMode::{
            ExecuteOneInput,
            Fuzz,
        },
    },
    instrumenter::instrumentation::Instrumenter,
};
use anyhow::{
    bail,
    Context,
};
use clap::Parser;
use std::{
    env::var,
    path::PathBuf,
};
use PhinkEnv::FuzzingWithConfig;

pub mod cli;
pub mod contract;
pub mod cover;
pub mod fuzzer;
pub mod instrumenter;

/// This struct defines the command line arguments expected by Phink.
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "🐙 Phink: An ink! smart-contract property-based and coverage-guided fuzzer",
    long_about = None
)]
struct Cli {
    /// Order to execute (if you start here, instrument then fuzz suggested)
    #[clap(subcommand)]
    command: Commands,

    /// Path to the Phink configuration file.
    #[clap(long, short, value_parser, default_value = "phink.toml")]
    config: PathBuf,
}

#[derive(clap::Subcommand, Debug)]
#[allow(deprecated)]
enum Commands {
    /// Starts the fuzzing process. Instrumentation required before!
    Fuzz,
    /// Instrument the ink! contract, and compile it with Phink features
    Instrument(Contract),
    /// Run all the seeds
    Run,
    /// Generate a coverage report, only of the harness. You won't have your contract coverage here
    /// (mainly for debugging purposes only)
    HarnessCover(Contract),
    /// Generate a coverage report for your smart-contract
    Coverage(Contract),
    /// Execute one seed
    Execute {
        /// Seed to be executed
        seed: PathBuf,
    },
}

#[derive(clap::Args, Debug)]
struct Contract {
    /// Path where the contract is located. It must be the root directory of
    /// the contract
    #[clap(value_parser)]
    contract_path: PathBuf,
}
pub fn main() {
    // We execute `handle_cli()` first, then re-enter into `main()`
    if let Ok(config_str) = var(FuzzingWithConfig.to_string()) {
        if var(FromZiggy.to_string()).is_ok() {
            let config = ZiggyConfig::parse(config_str);
            match Fuzzer::new(config) {
                Ok(fuzzer) => {
                    if let Err(e) = fuzzer.execute_harness(Fuzz) {
                        eprintln!("{}", format_error(e));
                    }
                }
                Err(e) => {
                    eprintln!("{}", format_error(e));
                }
            }
        }
    } else if let Err(e) = handle_cli() {
        eprintln!("{}", format_error(e));
    }
}

fn handle_cli() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let conf = &cli.config;
    if !conf.exists() {
        bail!(format!(
            "No configuration found at {}, please create a phink.toml",
            conf.to_str().unwrap(),
        ))
    }
    let config: Configuration = Configuration::try_from(conf).unwrap_or_default();

    match cli.command {
        Commands::Instrument(contract_path) => {
            let z_config: ZiggyConfig = ZiggyConfig::new_with_contract(
                config.to_owned(),
                contract_path.contract_path.to_owned(),
            )
            .context("Couldn't generate handle the ZiggyConfig")?;

            let engine = Instrumenter::new(z_config);
            engine
                .to_owned()
                .instrument()
                .context("Couldn't instrument")?;
            engine.build().context("Couldn't run the build")?;
            Ok(())
        }
        Commands::Fuzz => {
            ZiggyConfig::new(config)
                .context("Couldn't generate handle the ZiggyConfig")?
                .ziggy_fuzz()
        }
        Commands::Run => {
            ZiggyConfig::new(config)
                .context("Couldn't generate handle the ZiggyConfig")?
                .ziggy_run()
        }
        Commands::Execute { seed } => {
            let fuzzer = Fuzzer::new(
                ZiggyConfig::new(config).context("Couldn't generate handle the ZiggyConfig")?,
            )?;
            fuzzer.execute_harness(ExecuteOneInput(seed))
        }
        Commands::HarnessCover(contract_path) => {
            ZiggyConfig::new_with_contract(config, contract_path.contract_path)
                .context("Couldn't generate handle the ZiggyConfig")?
                .ziggy_cover()
        }
        Commands::Coverage(contract_path) => {
            CoverageTracker::generate(
                ZiggyConfig::new_with_contract(config, contract_path.contract_path)
                    .context("Couldn't generate handle the ZiggyConfig")?,
            )
        }
    }
}
