use crate::config::Configuration;
use crate::fuzzer::fuzz::DICT_FILE;
use crate::fuzzer::parser::MIN_SEED_LEN;
use std::io::Write;
use std::{
    env::var,
    fs,
    fs::File,
    io,
    io::BufRead,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub enum ZiggyCommand {
    Run,
    Cover,
    Build,
    Fuzz,
}

#[derive(Clone, Debug)]
pub struct Ziggy {
    pub config: Configuration,
}

impl Ziggy {
    pub const ALLOWLIST_PATH: &'static str = "./output/phink/allowlist.txt";
    pub const AFL_DEBUG: &'static str = "1";

    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    /// This function execute cargo ziggy + command + args
    fn start(command: ZiggyCommand, args: Vec<String>) -> io::Result<()> {
        let command_arg = Self::command_to_arg(&command)?;

        let mut binding = Command::new("cargo");
        let mut command_builder = binding
            .arg("ziggy")
            .arg(command_arg)
            .env("AFL_LLVM_ALLOWLIST", Self::ALLOWLIST_PATH)
            .env("AFL_DEBUG", Self::AFL_DEBUG)
            .stdout(Stdio::piped());

        // If there are additional arguments, pass them to the command
        command_builder.args(args.iter());

        let mut ziggy_child = command_builder.spawn()?;

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

    fn command_to_arg(command: &ZiggyCommand) -> Result<String, io::Error> {
        let command_arg = match command {
            ZiggyCommand::Run => "run",
            ZiggyCommand::Cover => "cover",
            ZiggyCommand::Fuzz => "fuzz",
            ZiggyCommand::Build => {
                Self::build_llvm_allowlist()?;
                "build"
            }
        };
        Ok(command_arg.parse().unwrap())
    }

    pub fn ziggy_fuzz(&self) -> io::Result<()> {
        Self::start(ZiggyCommand::Build, vec![])?;

        println!("🏗️ ZiggyCommand::Build completed");

        let mut all_my_args = Vec::new();

        let jobs = format!("--jobs={}", self.config.cores.unwrap_or_default());
        let dict = format!("--dict={}", DICT_FILE);
        let minlength = format!("--minlength={}", MIN_SEED_LEN);

        all_my_args.push(jobs);
        all_my_args.push(dict);
        all_my_args.push(minlength);

        if !self.config.use_honggfuzz {
            all_my_args.push("--no-honggfuzz".parse().unwrap());
        }

        Self::start(ZiggyCommand::Fuzz, all_my_args)?;
        Ok(())
    }

    /// Builds the LLVM allowlist if it doesn't already exist.
     fn build_llvm_allowlist() -> Result<(), io::Error> {
        let path = Path::new(Self::ALLOWLIST_PATH);

        if path.exists() {
            println!("❗ Allowlist already exists... skipping");
            return Ok(());
        }

        fs::create_dir_all(path.parent().unwrap())?;
        let mut allowlist_file = File::create(path)?;

        let functions = ["redirect_coverage*", "should_stop_now*", "parse_input*"];
        for func in &functions {
            writeln!(allowlist_file, "fun: {}", func)?;
        }

        println!("✅ Allowlist created successfully");
        Ok(())
    }
}
