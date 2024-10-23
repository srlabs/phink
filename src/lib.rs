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
    instrumenter::{
        instrumentation::Instrumenter,
        seedgen::SeedExtractInjector,
        traits::visitor::ContractVisitor,
    },
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

/// This type is used to handle all the different errors on Phink. It's a simple biding to anyhow.
pub type EmptyResult = anyhow::Result<()>;
pub type ResultOf<T> = anyhow::Result<T>;

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
    /// Starts the fuzzing campaign. Instrumentation required before!
    Fuzz,
    /// Run the tests of the ink! smart-contract to execute the
    /// messages and extracts valid seeds fromit. For instance, if a test call three messages,
    /// those three messages will be extracted to be used as seeds inside the corpus directory
    GenerateSeed {
        /// Path where the contract is located. It must be the root directory of
        /// the contract
        contract: PathBuf,
        /// Path where the temporary contract will be compiled to. Optionnal. In `tmp` if not
        /// defined.
        compiled_directory: Option<PathBuf>,
    },
    /// Instrument the ink! contract, and compile it with Phink features
    Instrument(Contract),
    /// Run all the seeds
    Run,
    /// Generate a coverage report, only of the harness. You won't have your contract coverage here
    /// (mainly for debugging purposes only)
    HarnessCover,
    /// Generate a coverage report for your smart-contract
    Coverage(Contract),
    /// Execute one seed
    Execute {
        /// Seed to be executed
        seed: PathBuf,
    },
}

#[derive(clap::Args, Debug, Clone)]
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

fn handle_cli() -> EmptyResult {
    let cli = Cli::parse();
    let conf = &cli.config;
    if !conf.exists() {
        bail!(format!(
            "No configuration found at {}, please create a phink.toml",
            conf.to_str().unwrap(),
        ))
    }
    let config: Configuration = Configuration::try_from(conf)?;

    match cli.command {
        Commands::Instrument(contract_path) => {
            let z_config: ZiggyConfig = ZiggyConfig::new_with_contract(
                config.to_owned(),
                contract_path.contract_path.to_owned(),
            )
            .context("Couldn't generate handle the ZiggyConfig")?;

            let engine = Instrumenter::new(z_config.to_owned());
            engine
                .to_owned()
                .instrument()
                .context("Couldn't instrument")?;

            engine.build().context("Couldn't run the build")?;

            println!(
                "\n🤞 Contract '{}' has been instrumented and compiled.\n🤞 You can find the instrumented contract in `{}`",
                z_config.contract_path()?.display(),
                z_config.config().instrumented_contract().display()
            );
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
            let fuzzer = Fuzzer::new(ZiggyConfig::new(config))
                .context("Creating a new fuzzer instance faled")?;
            fuzzer.execute_harness(ExecuteOneInput(seed))
        }
        Commands::HarnessCover => {
            ZiggyConfig::new(config)
                .context("Couldn't generate handle the ZiggyConfig")?
                .ziggy_cover()
        }
        Commands::Coverage(contract_path) => {
            CoverageTracker::generate(
                ZiggyConfig::new_with_contract(config, contract_path.contract_path)
                    .context("Couldn't generate handle the ZiggyConfig")?,
            )
        }
        Commands::GenerateSeed {
            contract,
            compiled_directory,
        } => {
            let seeder = SeedExtractInjector::new(&contract, compiled_directory)?;
            seeder
                .prepare()
                .context(format!("Couldn't extract the seed from {contract:?}"))?;

            // seeder.run_tests().context("xxxxxxx")?;
            Ok(())
        }
    }
}
