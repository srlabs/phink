#![recursion_limit = "1024"]

extern crate core;

use env::{set_var, var};
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, fs, io, path::PathBuf};

use clap::Parser;
use sp_core::crypto::AccountId32;
use sp_core::hexdisplay::AsBytesRef;

use FuzzingMode::ExecuteOneInput;

use crate::{
    contract::remote::ContractBridge,
    fuzzer::engine::FuzzerEngine,
    fuzzer::fuzz::Fuzzer,
    fuzzer::instrument::{ContractBuilder, ContractInstrumenter, InstrumenterEngine},
    fuzzer::parser::MIN_SEED_LEN,
    fuzzer::report::CoverageTracker,
    FuzzingMode::FuzzMode,
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
    Using Ziggy `PHINK_CONTRACT_DIR=/tmp/ink_fuzzed_QEBAC/ PHINK_FROM_ZIGGY=true PHINK_START_FUZZING=true cargo ziggy run`"
)]
struct Cli {
    /// Additional command to specify operation mode
    #[clap(subcommand)]
    command: Commands,
}

/// Commands supported by Phink
#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Starts the fuzzing process. Instrumentation required before!
    Fuzz {
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
        /// Number of cores to use for Ziggy
        #[clap(long, short, value_parser, default_value = "1")]
        cores: Option<u8>,
        /// Add Hongfuzz as a fuzzer... or not
        #[clap(long, short, value_parser, default_value = "false")]
        use_honggfuzz: bool,
    },
    /// Instrument the ink! contract, and compile it with Phink features
    Instrument {
        /// Path where the contract is located. It must be the root directory of the contract path (where the `lib.rs` is located)
        #[clap(value_parser)]
        contract_path: PathBuf,
    },
    /// Run all the seeds
    Run {
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
    },
    /// Remove all the temporary files under `/tmp/ink_fuzzed_XXXX`
    Clean,
    /// Generate a coverage, only of the harness. You won't have your contract coverage here.
    /// It's mainly for debugging purposes inly
    HarnessCover {
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
    },
    /// Generate a coverage, for your smart-contract
    Coverage {
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
        /// Output directory for the coverage report
        #[clap(value_parser, default_value = "coverage_report")]
        report_path: PathBuf,
    },
    /// Execute one seed
    Execute {
        /// Path to the file containing the input seed
        #[clap(value_parser)]
        seed_path: PathBuf,
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
    },
}

pub enum ZiggyCommand {
    Run,
    Cover,
    Build,
}

pub enum FuzzingMode {
    ExecuteOneInput(Box<[u8]>),
    FuzzMode,
}

fn main() {
    if var("PHINK_FROM_ZIGGY").is_ok() {
        println!("ℹ️ Setting AFL_FORKSRV_INIT_TMOUT to 10000000");
        unsafe {
            set_var("AFL_FORKSRV_INIT_TMOUT", "10000000");
        }

        let path = var("PHINK_CONTRACT_DIR").map(PathBuf::from).expect(
            "\n🈲️ PHINK_CONTRACT_DIR is not set. \
                You can set it manually, it should contain the source code of your contract, \
                with or without the instrumented binary,\
                depending your options. \n\n",
        );
        // Here, the contract is already instrumented
        if var("PHINK_START_FUZZING").is_ok() {
            println!("Starting the fuzzer");
            execute_harness(&mut InstrumenterEngine { contract_dir: path }, FuzzMode);
        } else {
            let seed = var("PHINK_EXECUTE_THIS_SEED");
            if seed.is_ok() {
                println!("Executing one seed: {:?}", seed);
                let data = fs::read(Path::new(&seed.unwrap())).expect("Unable to read file");
                execute_harness(
                    &mut InstrumenterEngine { contract_dir: path },
                    ExecuteOneInput(Box::from(data)),
                );
            }
        }
    } else {
        let cli = Cli::parse();

        match &cli.command {
            Commands::Instrument { contract_path } => {
                instrument(contract_path);
            }

            Commands::Fuzz {
                contract_path,
                cores,
                use_honggfuzz,
            } => {
                unsafe {
                    set_var("PHINK_CONTRACT_DIR", contract_path);
                    set_var("PHINK_START_FUZZING", "true");
                    set_var("PHINK_CORES", cores.unwrap_or(1).to_string());
                }
                let cores: u8 = var("PHINK_CORES").map_or(1, |v| v.parse().unwrap_or(1));
                let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());
                let mut engine = InstrumenterEngine::new(contract_dir.clone());

                // Even if the build should be processed within the fuzz command after this lien
                // We still manually execute ziggy build, to ensure that the ALLOW_LIST works correctly
                start_cargo_ziggy_not_fuzzing_process(contract_dir.clone(), ZiggyCommand::Build);

                println!("ZiggyCommand::Build over");

                start_cargo_ziggy_fuzz_process(cores, use_honggfuzz);

                if var("PHINK_START_FUZZING").is_ok() {
                    execute_harness(&mut engine, FuzzMode);
                }
            }

            Commands::Run { contract_path } => {
                unsafe {
                    set_var("PHINK_CONTRACT_DIR", contract_path);
                }
                let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());
                start_cargo_ziggy_not_fuzzing_process(contract_dir, ZiggyCommand::Run);
            }

            Commands::Execute {
                seed_path,
                contract_path,
            } => {
                unsafe {
                    set_var("PHINK_CONTRACT_DIR", contract_path);
                }

                let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());
                let mut engine = InstrumenterEngine::new(contract_dir);
                let data = fs::read(seed_path).expect("🤔 Unable to read the seed, maybe you opened the directory instead of the seed itself ?");

                execute_harness(&mut engine, ExecuteOneInput(Box::from(data)));
            }

            Commands::HarnessCover { contract_path } => {
                unsafe {
                    set_var("PHINK_CONTRACT_DIR", contract_path);
                }
                let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());
                start_cargo_ziggy_not_fuzzing_process(contract_dir, ZiggyCommand::Cover);
            }

            Commands::Coverage {
                contract_path,
                report_path,
            } => {
                //todo: if .cov doesn't exist, we execute the start_cargo_ziggy_not_fuzzing_process(contract_dir, ZiggyCommand::Run)
                use std::io::Read;

                let mut file = File::open("./output/phink/traces.cov").unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();

                let mut tracker = CoverageTracker::new(&contents);

                tracker
                    .process_file(format!("{}{}", contract_path.display(), "/lib.rs").as_str())
                    .expect("🙅Cannot process file"); //todo: should do it for every file

                tracker
                    .generate_report(report_path.to_str().unwrap())
                    .expect("🙅Cannot generate coverage report");
            }
            Commands::Clean => {
                InstrumenterEngine::clean().expect("🧼 Cannot execute the cleaning properly.");
            }
        };
    }
}

fn start_cargo_ziggy_fuzz_process(cores: u8, use_honggfuzz: &bool) {
    let mut command = Command::new("cargo");
    command.arg("ziggy").arg("fuzz");

    if !use_honggfuzz {
        command.arg("--no-honggfuzz");
    }

    command
        .arg(format!("--jobs={}", cores))
        .arg(format!("--minlength={}", MIN_SEED_LEN))
        .arg("--dict=./output/phink/selectors.dict")
        .env("PHINK_FROM_ZIGGY", "true")
        .stdout(Stdio::piped());

    let mut child = command
        .spawn()
        .expect("🙅 Failed to execute cargo ziggy fuzz...");

    if let Some(stdout) = child.stdout.take() {
        let reader = io::BufReader::new(stdout);
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

fn start_cargo_ziggy_not_fuzzing_process(contract_dir: PathBuf, command: ZiggyCommand) {
    let mut allowlist_path = String::new();
    let command_arg = match command {
        ZiggyCommand::Run => "run",
        ZiggyCommand::Cover => "cover",
        ZiggyCommand::Build => {
            allowlist_path = build_llvm_allowlist().expect("🙅 Couldn't write the LLVM allowlist");
            "build"
        }
    };

    let mut ziggy_child = Command::new("cargo")
        .arg("ziggy")
        .arg(command_arg)
        .env("PHINK_CONTRACT_DIR", contract_dir)
        .env("PHINK_FROM_ZIGGY", "true")
        .env("PHINK_START_FUZZING", "true")
        .env("AFL_LLVM_ALLOWLIST", allowlist_path)
        .env("AFL_DEBUG", "1")
        .stdout(Stdio::piped())
        .spawn()
        .expect("🙅 Failed to execute cargo ziggy command...");

    if let Some(stdout) = ziggy_child.stdout.take() {
        let reader = io::BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(e) => eprintln!("🙅 Error reading line: {}", e),
            }
        }
    }

    let status = ziggy_child
        .wait()
        .expect("🙅 Failed to wait on child process...");
    if !status.success() {
        eprintln!("🙅 Command executed with failing error code");
    }
}

// This function creates the allowlist for AFL... thanks to that feature we have a now craaaaazy blazing fast fuzzer :)
fn build_llvm_allowlist() -> Result<String, io::Error> {
    let file_path = "./output/phink/allowlist.txt";

    // Check if the file already exists
    let path = Path::new(file_path);
    return if path.exists() {
        println!("❗ Allowlist already exists... skipping ");
        Ok(fs::canonicalize(path)?.display().to_string())
    } else {
        fs::create_dir_all("./output/phink/")?;

        let mut allowlist_file = File::create(file_path)?;
        writeln!(allowlist_file, "fun: redirect_coverage*")?;
        writeln!(allowlist_file, "fun: should_stop_now*")?;
        writeln!(allowlist_file, "fun: parse_input*")?;

        println!("✅ Allowlist created successfully");
        Ok(fs::canonicalize(path)?.display().to_string())
    };
}

fn instrument(path: &PathBuf) -> InstrumenterEngine {
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

fn execute_harness(engine: &mut InstrumenterEngine, fuzzing_mode: FuzzingMode) {
    let origin: AccountId32 = AccountId32::new([1; 32]); //TODO: This should be configurable

    let finder = engine.find().unwrap();

    match fs::read(&finder.wasm_path) {
        Ok(wasm) => {
            let setup = ContractBridge::initialize_wasm(wasm, &finder.specs_path, origin);
            let fuzzer = Fuzzer::new(setup);
            match fuzzing_mode {
                FuzzMode => {
                    fuzzer.fuzz();
                }
                ExecuteOneInput(seed) => {
                    fuzzer.exec_seed(seed.as_bytes_ref());
                }
            }
        }
        Err(e) => {
            eprintln!(
                "🙅 Error reading WASM file. Maybe the JSON file is not equal to the WASM file ?\
            Please, ensure that both are identitical.\
            Example: mycontract.wasm and mycontract.json.\
            (more details: {})",
                e
            );
        }
    }
}
