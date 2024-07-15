use std::fs::File;
use std::io::{
    BufRead,
    Write,
};
use std::path::{
    Path,
    PathBuf,
};
use std::process::{
    Command,
    Stdio,
};
use std::{
    fs,
    io,
};

use serde_derive::{
    Deserialize,
    Serialize,
};

use crate::cli::config::Configuration;
use crate::fuzzer::fuzz::DICT_FILE;
use crate::fuzzer::parser::MIN_SEED_LEN;

pub enum ZiggyCommand {
    Run,
    Cover,
    Build,
    Fuzz,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZiggyConfig {
    pub config: Configuration,
    pub contract_path: PathBuf,
}

impl ZiggyConfig {
    pub const ALLOWLIST_PATH: &'static str = "./output/phink/allowlist.txt";
    pub const AFL_DEBUG: &'static str = "1";

    pub fn new(config: Configuration, contract_path: PathBuf) -> Self {
        Self { config, contract_path }
    }

    pub fn parse(config_str: String) -> Self {
        serde_json::from_str(&config_str).expect("❌ Failed to parse config")
    }

    /// This function execute `cargo ziggy + command + args`
    fn start(
        command: ZiggyCommand,
        args: Vec<String>,
        env: Vec<(String, String)>,
    ) -> io::Result<()> {
        let command_arg = Self::command_to_arg(&command)?;

        let mut binding = Command::new("cargo");
        let command_builder = binding
            .arg("ziggy")
            .arg(command_arg)
            .env("AFL_FORKSRV_INIT_TMOUT", "10000000")
            .env(
                "AFL_LLVM_ALLOWLIST",
                Path::new(Self::ALLOWLIST_PATH)
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            )
            .env("AFL_DEBUG", Self::AFL_DEBUG)
            .stdout(Stdio::piped());

        // If there are additional arguments, pass them to the command
        command_builder.args(args.iter());

        // If there is any env, pass them
        for (key, value) in env {
            command_builder.env(key, value);
        }

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
            },
        };
        Ok(command_arg.parse().unwrap())
    }

    pub fn ziggy_fuzz(&self) -> io::Result<()> {
        Self::start(ZiggyCommand::Build, vec![], vec![])?;
        println!("🏗️ Ziggy Build completed");

        let mut fuzzing_args = Vec::new();

        let jobs = format!("--jobs={}", self.config.cores.unwrap_or_default());
        let dict = format!("--dict={}", DICT_FILE);
        let minlength = format!("--minlength={}", MIN_SEED_LEN);

        fuzzing_args.push(jobs);
        fuzzing_args.push(dict);
        fuzzing_args.push(minlength);

        if !self.config.use_honggfuzz {
            fuzzing_args.push("--no-honggfuzz".parse().unwrap());
        }

        Self::start(
            ZiggyCommand::Fuzz,
            fuzzing_args,
            vec![(
                "PHINK_START_FUZZING_WITH_CONFIG".into(),
                serde_json::to_string(self).unwrap(),
            )],
        )?;
        Ok(())
    }

    pub fn ziggy_cover(&self) -> io::Result<()> {
        Self::start(
            ZiggyCommand::Cover,
            vec![],
            vec![(
                "PHINK_START_FUZZING_WITH_CONFIG".into(),
                serde_json::to_string(self).unwrap(),
            )],
        )?;
        Ok(())
    }

    pub fn ziggy_run(&self) -> io::Result<()> {
        Self::start(
            ZiggyCommand::Run,
            vec![],
            vec![(
                "PHINK_START_FUZZING_WITH_CONFIG".into(),
                serde_json::to_string(self).unwrap(),
            )],
        )?;
        Ok(())
    }

    /// Builds the LLVM allowlist if it doesn't already exist.
    fn build_llvm_allowlist() -> Result<(), io::Error> {
        let path = Path::new(Self::ALLOWLIST_PATH);

        if path.exists() {
            println!("❗ AFL_LLVM_ALLOWLIST already exists... skipping");
            return Ok(());
        }

        fs::create_dir_all(path.parent().unwrap())?;
        let mut allowlist_file = File::create(path)?;

        let functions =
            ["redirect_coverage*", "should_stop_now*", "parse_input*"];
        for func in &functions {
            writeln!(allowlist_file, "fun: {}", func)?;
        }

        println!("✅ AFL_LLVM_ALLOWLIST created successfully");
        Ok(())
    }
}
