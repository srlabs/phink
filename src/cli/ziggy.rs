use crate::{
    cli::config::Configuration,
    fuzzer::parser::MIN_SEED_LEN,
};

use serde_derive::{
    Deserialize,
    Serialize,
};
use std::{
    fs,
    fs::File,
    io::Write,
    path::PathBuf,
};

use crate::cli::config::{
    PFiles::{
        AllowListPath,
        CoverageTracePath,
        DictPath,
    },
    PhinkFiles,
};
use crossterm::{
    event,
    event::Event,
};
use ratatui::{
    text::Text,
    Frame,
};
use std::{
    io::{
        self,
    },
    process::{
        Command,
        Stdio,
    },
};

pub const AFL_DEBUG: &str = "1";
pub const AFL_FORKSRV_INIT_TMOUT: &str = "10000000";

#[derive(Copy, Clone, Debug)]
pub enum ZiggyCommand {
    Run,
    Cover,
    Build,
    Fuzz(bool),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZiggyConfig {
    pub config: Configuration,
    pub contract_path: PathBuf,
}

impl ZiggyConfig {
    pub fn new(config: Configuration, contract_path: PathBuf) -> Self {
        Self {
            config,
            contract_path,
        }
    }

    pub fn parse(config_str: String) -> Self {
        let config: Self = serde_json::from_str(&config_str).expect("❌ Failed to parse config");
        if config.config.verbose {
            println!("🖨️ Using PHINK_START_FUZZING_WITH_CONFIG = {config_str}",);
        }
        config
    }

    /// Basic hello world for TUI
    fn initialize_tui() -> Result<(), Box<dyn std::error::Error>> {
        let mut terminal = ratatui::init();
        loop {
            terminal
                .draw(|frame: &mut Frame| {
                    frame.render_widget(Text::raw("Hello World!"), frame.area())
                })
                .expect("Failed to draw");
            if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
                break;
            }
        }
        ratatui::restore();
        Ok(())
    }

    /// This function executes `cargo ziggy 'command' 'args'`
    fn start(
        &self,
        command: ZiggyCommand,
        args: Vec<String>,
        env: Vec<(String, String)>,
    ) -> anyhow::Result<()> {
        let use_ui: bool;
        let ziggy_command: String = match command {
            ZiggyCommand::Run => "run",
            ZiggyCommand::Cover => "cover",
            ZiggyCommand::Build => {
                self.build_llvm_allowlist()?;
                "build"
            }
            ZiggyCommand::Fuzz(ui) => {
                use_ui = ui;
                "fuzz"
            }
        }
        .parse()?;

        let mut binding = Command::new("cargo");
        let command_builder = binding
            .arg("ziggy")
            .arg(ziggy_command)
            .env("PHINK_FROM_ZIGGY", "1")
            .env("AFL_FORKSRV_INIT_TMOUT", AFL_FORKSRV_INIT_TMOUT)
            .env("AFL_DEBUG", AFL_DEBUG)
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        self.with_allowlist(command_builder)?;

        // If there are additional arguments, pass them to the command
        command_builder.args(args.iter());
        command_builder.envs(env);
        command_builder.spawn()?;

        // let status = ziggy_child.wait()?;

        Ok(())
    }

    fn with_allowlist(&self, mut command_builder: &mut Command) -> anyhow::Result<()> {
        // Add `AFL_LLVM_ALLOWLIST` if not on macOS
        // See https://github.com/rust-lang/rust/issues/127573
        // See https://github.com/rust-lang/rust/issues/127577
        if cfg!(not(target_os = "macos")) {
            let allowlist = PhinkFiles::new(self.config.fuzz_output.clone().unwrap_or_default())
                .path(AllowListPath);

            command_builder = command_builder.env(
                "AFL_LLVM_ALLOWLIST",
                allowlist.canonicalize()?.to_str().unwrap(),
            );
        }
        Ok(())
    }

    pub fn ziggy_fuzz(&self) -> anyhow::Result<()> {
        let fuzzoutput = &self.config.fuzz_output;
        let dict = PhinkFiles::new(fuzzoutput.clone().unwrap_or_default()).path(DictPath);

        let build_args = if !self.config.use_honggfuzz {
            vec!["--no-honggfuzz".parse()?]
        } else {
            vec!["".parse()?]
        };

        self.start(ZiggyCommand::Build, build_args, vec![])?;

        println!("🏗️ Ziggy Build completed");

        let mut fuzzing_args = vec![
            format!("--jobs={}", self.config.cores.unwrap_or_default()),
            format!("--dict={}", dict.to_str().unwrap()),
            format!("--minlength={}", MIN_SEED_LEN),
        ];
        if !self.config.use_honggfuzz {
            fuzzing_args.push("--no-honggfuzz".parse()?)
        }

        if fuzzoutput.is_some() {
            fuzzing_args.push(format!(
                "--ziggy-output={}",
                fuzzoutput.clone().unwrap().to_str().unwrap()
            ))
        }

        let fuzz_config = vec![(
            "PHINK_START_FUZZING_WITH_CONFIG".to_string(),
            serde_json::to_string(self)?,
        )];

        self.start(
            ZiggyCommand::Fuzz(self.config.show_ui),
            fuzzing_args,
            fuzz_config,
        )
    }

    pub fn ziggy_cover(&self) -> anyhow::Result<()> {
        self.start(
            ZiggyCommand::Cover,
            vec![],
            vec![(
                "PHINK_START_FUZZING_WITH_CONFIG".into(),
                serde_json::to_string(self).unwrap(),
            )],
        )?;
        Ok(())
    }

    pub fn ziggy_run(&self) -> anyhow::Result<()> {
        let covpath = PhinkFiles::new(self.config.fuzz_output.clone().unwrap_or_default())
            .path(CoverageTracePath);

        // We clean up the old one first
        if fs::remove_file(covpath).is_ok() {
            println!("💨 Removed previous coverage file")
        }

        self.start(
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
    fn build_llvm_allowlist(&self) -> io::Result<()> {
        let allowlist_path = PhinkFiles::new(self.config.fuzz_output.clone().unwrap_or_default())
            .path(AllowListPath);

        if allowlist_path.exists() {
            println!("❗ AFL_LLVM_ALLOWLIST already exists... skipping");
            return Ok(());
        }

        fs::create_dir_all(allowlist_path.parent().unwrap())?;
        let mut allowlist_file = File::create(allowlist_path)?;

        let functions = ["redirect_coverage*", "should_stop_now*", "parse_input*"];
        for func in &functions {
            writeln!(allowlist_file, "fun: {}", func)?;
        }

        println!("✅ AFL_LLVM_ALLOWLIST created successfully");
        Ok(())
    }
}
