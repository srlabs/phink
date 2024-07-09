#![recursion_limit = "1024"]

extern crate core;

use env::{set_var, var};
use std::{
    env, fs,
    fs::File,
    io,
    io::Read,
    io::{BufRead, Write},
    path::Path,
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::Parser;
use sp_core::{
    crypto::{AccountId32, Ss58Codec},
    hexdisplay::AsBytesRef,
};

use FuzzingMode::ExecuteOneInput;

use crate::fuzzer::fuzz::MAX_MESSAGES_PER_EXEC;
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

/// This struct defines the command line arguments expected by Phink.
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Phink is a command line tool for fuzzing ink! smart contracts.",
    long_about = "🐙 Phink, an ink! smart-contract property-based and coverage-guided fuzzer

Environment variables:
    PHINK_FROM_ZIGGY: Informs the tooling that the binary is being run with Ziggy, not directly from the CLI
    PHINK_CONTRACT_DIR: Location of the contract code-base. Can be automatically detected.
    PHINK_START_FUZZING: Tells the harness to start fuzzing.

Using Ziggy: PHINK_CONTRACT_DIR=/tmp/ink_fuzzed_QEBAC/ PHINK_FROM_ZIGGY=true PHINK_START_FUZZING=true cargo ziggy run"
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
        #[clap(long, short, value_parser)]
        cores: Option<u8>,
        /// Also use Hongfuzz as a fuzzer
        #[clap(long, short, value_parser, default_value = "false")]
        use_honggfuzz: bool,
        // Origin deploying and instantiating the contract
        #[clap(long, short, value_parser)]
        deployer_address: Option<AccountId32>,
        // Maximimum number of ink! message executed per seed
        #[clap(long, short, value_parser)]
        max_messages_per_exec: Option<usize>,
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
        // Origin deploying and instantiating the contract
        #[clap(long, short, value_parser)]
        deployer_address: Option<AccountId32>,
    },
    /// Remove all the temporary files under `/tmp/ink_fuzzed_XXXX`
    Clean,
    /// Generate a coverage, only of the harness. You won't have your contract coverage here.
    /// It's mainly for debugging purposes inly
    HarnessCover {
        /// Path where the contract is located. It must be the root directory of the contract
        #[clap(value_parser)]
        contract_path: PathBuf,
        // Origin deploying and instantiating the contract
        #[clap(long, short, value_parser)]
        deployer_address: Option<AccountId32>,
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
        // Origin deploying and instantiating the contract
        #[clap(long, short, value_parser)]
        deployer_address: Option<AccountId32>,
    },
}

pub enum ZiggyCommand {
    Run,
    Cover,
    Build,
}

pub enum FuzzingMode {
    ExecuteOneInput(Box<[u8]>),
    FuzzMode(Option<usize>),
}

pub const DEFAULT_DEPLOYER: AccountId32 = AccountId32::new([0u8; 32]);

fn main() -> io::Result<()> {
    if var("PHINK_FROM_ZIGGY").is_ok() {
        handle_ziggy_mode()
    } else {
        handle_cli_mode()
    }
}

fn handle_ziggy_mode() -> io::Result<()> {
    println!("ℹ️ Setting AFL_FORKSRV_INIT_TMOUT to 10000000");
    unsafe {
        set_var("AFL_FORKSRV_INIT_TMOUT", "10000000");
    }

    let path = var("PHINK_CONTRACT_DIR").map(PathBuf::from).expect(
        "📙 PHINK_CONTRACT_DIR is not set. Set it manually to the source code of your contract.",
    );

    let deployer_address = var("PHINK_ACCOUNT_DEPLOYER")
        .ok()
        .and_then(|addr| AccountId32::from_string(&addr).ok());

    let max_messages_per_exec = var("PHINK_MAX_MESSAGES_PER_EXEC")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut engine = InstrumenterEngine::new(path);
    if var("PHINK_START_FUZZING").is_ok() {
        println!("🏃Starting the fuzzer");
        execute_harness(
            &mut engine,
            FuzzMode(Some(max_messages_per_exec)),
            deployer_address,
        )?;
    }

    Ok(())
}

fn handle_cli_mode() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Instrument { contract_path } => {
            let mut engine = InstrumenterEngine::new(contract_path.to_path_buf());
            engine
                .instrument()
                .expect("Custom instrumentation failed")
                .build()
                .expect("Compilation with Phink features failed");
            println!(
                "🤞 Contract {} has been instrumented, and is now compiled!",
                contract_path.display()
            );
        }
        Commands::Fuzz {
            contract_path,
            cores,
            use_honggfuzz,
            deployer_address,
            max_messages_per_exec,
        } => {
            handle_fuzz_command(
                contract_path,
                cores.unwrap_or(1),
                use_honggfuzz,
                deployer_address,
                max_messages_per_exec,
            )?;
        }
        Commands::Run {
            contract_path,
            deployer_address,
        } => {
            set_env_vars(&contract_path, &deployer_address, &None);
            start_cargo_ziggy(&contract_path, ZiggyCommand::Run)?
        }
        Commands::Execute {
            seed_path,
            contract_path,
            deployer_address,
        } => {
            handle_execute_command(seed_path, contract_path, deployer_address)?;
        }
        Commands::HarnessCover {
            contract_path,
            deployer_address,
        } => {
            set_env_vars(&contract_path, &deployer_address, &None);
            start_cargo_ziggy(&contract_path, ZiggyCommand::Cover)?
        }
        Commands::Coverage {
            contract_path,
            report_path,
        } => {
            handle_coverage_command(contract_path, report_path);
        }
        Commands::Clean => {
            InstrumenterEngine::clean()?;
        }
    }

    Ok(())
}

fn handle_fuzz_command(
    contract_path: PathBuf,
    cores: u8,
    use_honggfuzz: bool,
    deployer_address: Option<AccountId32>,
    max_messages_per_exec: Option<usize>,
) -> io::Result<()> {
    set_env_vars(&contract_path, &deployer_address, &max_messages_per_exec);

    let contract_dir = PathBuf::from(var("PHINK_CONTRACT_DIR").unwrap());

    start_cargo_ziggy(&contract_dir, ZiggyCommand::Build)?;
    println!("🏗️ ZiggyCommand::Build completed");

    start_cargo_ziggy_fuzz(cores, use_honggfuzz)?;
    Ok(())
}
fn set_env_vars(
    contract_path: &Path,
    deployer_address: &Option<AccountId32>,
    max_messages_per_exec: &Option<usize>,
) {
    unsafe {
        set_var("PHINK_CONTRACT_DIR", contract_path);
        set_var("PHINK_START_FUZZING", "true");
        set_var(
            "PHINK_MAX_MESSAGES_PER_EXEC",
            max_messages_per_exec
                .unwrap_or(MAX_MESSAGES_PER_EXEC)
                .to_string(),
        );
        set_var(
            "PHINK_ACCOUNT_DEPLOYER",
            deployer_address
                .clone()
                .unwrap_or_else(|| DEFAULT_DEPLOYER)
                .to_string(),
        );
    }
}

fn start_cargo_ziggy_fuzz(cores: u8, use_honggfuzz: bool) -> io::Result<()> {
    let mut command = Command::new("cargo");
    command
        .arg("ziggy")
        .arg("fuzz")
        .arg(format!("--jobs={}", cores))
        .arg(format!("--minlength={}", MIN_SEED_LEN))
        .arg("--dict=./output/phink/selectors.dict")
        .env("PHINK_FROM_ZIGGY", "true")
        .stdout(Stdio::piped());

    if !use_honggfuzz {
        command.arg("--no-honggfuzz");
    }

    let mut child = command.spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let reader = io::BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    let status = child.wait()?;
    if !status.success() {
        eprintln!("Command executed with failing error code");
    }

    Ok(())
}

fn start_cargo_ziggy(contract_dir: &Path, command: ZiggyCommand) -> io::Result<()> {
    let (command_arg, allowlist_path) = match command {
        ZiggyCommand::Run => ("run", String::new()),
        ZiggyCommand::Cover => ("cover", String::new()),
        ZiggyCommand::Build => ("build", build_llvm_allowlist()?),
    };

    let mut ziggy_child = Command::new("cargo")
        .arg("ziggy")
        .arg(command_arg)
        .env("PHINK_CONTRACT_DIR", contract_dir)
        .env("PHINK_FROM_ZIGGY", "true")
        .env("PHINK_START_FUZZING", "true")
        .env("AFL_LLVM_ALLOWLIST", &allowlist_path)
        .env("AFL_DEBUG", "1")
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(stdout) = ziggy_child.stdout.take() {
        let reader = io::BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    let status = ziggy_child.wait()?;
    if !status.success() {
        eprintln!("`cargo ziggy` failed");
    }

    Ok(())
}

fn build_llvm_allowlist() -> io::Result<String> {
    let file_path = "./output/phink/allowlist.txt";
    let path = Path::new(file_path);
    if path.exists() {
        println!("❗ Allowlist already exists... skipping ");
        return Ok(fs::canonicalize(path)?.display().to_string());
    }
    fs::create_dir_all("./output/phink/")?;
    let mut allowlist_file = File::create(file_path)?;
    for func in ["redirect_coverage*", "should_stop_now*", "parse_input*"] {
        writeln!(allowlist_file, "fun: {}", func)?;
    }
    println!("✅ Allowlist created successfully");
    Ok(fs::canonicalize(path)?.display().to_string())
}

fn execute_harness(
    engine: &mut InstrumenterEngine,
    fuzzing_mode: FuzzingMode,
    deployer_id: Option<AccountId32>,
) -> io::Result<()> {
    let contract_deployer_origin = deployer_id.unwrap_or_else(|| DEFAULT_DEPLOYER);
    let finder = engine.find().unwrap();

    let wasm = fs::read(&finder.wasm_path)?;
    let setup = ContractBridge::initialize_wasm(wasm, &finder.specs_path, contract_deployer_origin);
    let mut fuzzer = Fuzzer::new(setup);

    match fuzzing_mode {
        FuzzMode(max_messages_per_exec) => {
            fuzzer.set_max_messages_per_exec(max_messages_per_exec);
            fuzzer.fuzz();
        }
        ExecuteOneInput(seed) => {
            fuzzer.exec_seed(seed.as_bytes_ref());
        }
    }

    Ok(())
}

fn handle_execute_command(
    seed_path: PathBuf,
    contract_path: PathBuf,
    deployer_address: Option<AccountId32>,
) -> io::Result<()> {
    set_env_vars(&contract_path, &deployer_address, &None);

    let mut engine = InstrumenterEngine::new(contract_path);
    let data = fs::read(seed_path)?;

    execute_harness(
        &mut engine,
        ExecuteOneInput(Box::from(data)),
        deployer_address,
    )
}

fn handle_coverage_command(contract_path: PathBuf, report_path: PathBuf) {
    //todo: if .cov doesn't exist, we execute the start_cargo_ziggy_not_fuzzing_process(contract_dir, ZiggyCommand::Run)

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
