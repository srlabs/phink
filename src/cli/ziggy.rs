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

use std::{
    io::{
        self,
        Read,
    },
    process::{
        Command,
        Stdio,
    },
    sync::mpsc,
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
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode,
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{
        Constraint,
        Direction,
        Layout,
    },
    style::{
        Color,
        Modifier,
        Style,
    },
    text::{
        Line,
        Span,
    },
    widgets::{
        Block,
        Borders,
        Paragraph,
    },
    Frame,
    Terminal,
};

enum AppEvent {
    Tick,
    ZiggyOutput(String),
}

struct App {
    ziggy_output: Vec<String>,
}

impl App {
    fn new() -> App {
        App {
            ziggy_output: Vec::new(),
        }
    }
}
pub const AFL_DEBUG: &str = "1";
pub const AFL_FORKSRV_INIT_TMOUT: &str = "10000000";

#[derive(Copy, Clone, Debug)]
pub enum ZiggyCommand {
    Run,
    Cover,
    Build,
    Fuzz,
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
            println!("🖨️ Using PHINK_START_FUZZING_WITH_CONFIG = {}", config_str);
        }
        config
    }

    /// This function executes `cargo ziggy 'command' 'args'`
    fn start(
        &self,
        command: ZiggyCommand,
        args: Vec<String>,
        env: Vec<(String, String)>,
    ) -> io::Result<()> {
        let command_arg: String = match command {
            ZiggyCommand::Run => "run",
            ZiggyCommand::Cover => "cover",
            ZiggyCommand::Fuzz => "fuzz",
            ZiggyCommand::Build => {
                self.build_llvm_allowlist()?;
                "build"
            }
        }
        .parse()
        .unwrap();

        let mut binding = Command::new("cargo");
        let mut command_builder = binding
            .arg("ziggy")
            .arg(command_arg)
            .env("PHINK_FROM_ZIGGY", "1")
            .env("AFL_FORKSRV_INIT_TMOUT", AFL_FORKSRV_INIT_TMOUT)
            .env("AFL_DEBUG", AFL_DEBUG)
            .stdout(Stdio::null())
            .stderr(Stdio::null());

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

        // If there are additional arguments, pass them to the command
        command_builder.args(args.iter());

        for (key, value) in env {
            command_builder.env(key, value);
        }

        command_builder.spawn()?;

        // let status = ziggy_child.wait()?;

        Ok(())
    }

    fn ui(f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());

        let hello_world = Paragraph::new("Hello World!")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Greeting"));
        f.render_widget(hello_world, chunks[0]);

        let ziggy_output = Paragraph::new(Line::from(
            app.ziggy_output
                .iter()
                .map(|line| Span::raw(line))
                .collect::<Vec<_>>(),
        ))
        .scroll((app.ziggy_output.len() as u16, 0))
        .block(Block::default().borders(Borders::ALL).title("Ziggy Output"));
        f.render_widget(ziggy_output, chunks[1]);
    }

    pub fn ziggy_fuzz(&self) -> io::Result<()> {
        let fuzzoutput = &self.config.fuzz_output;
        let dict = PhinkFiles::new(fuzzoutput.clone().unwrap_or_default()).path(DictPath);

        let build_args = if !self.config.use_honggfuzz {
            vec!["--no-honggfuzz".parse().unwrap()]
        } else {
            vec!["".parse().unwrap()]
        };

        self.start(ZiggyCommand::Build, build_args, vec![])?;

        println!("🏗️ Ziggy Build completed");

        let mut fuzzing_args = vec![
            format!("--jobs={}", self.config.cores.unwrap_or_default()),
            format!("--dict={}", dict.to_str().unwrap()),
            format!("--minlength={}", MIN_SEED_LEN),
        ];
        if !self.config.use_honggfuzz {
            fuzzing_args.push("--no-honggfuzz".parse().unwrap())
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

        self.start(ZiggyCommand::Fuzz, fuzzing_args, fuzz_config)
    }

    pub fn ziggy_cover(&self) -> io::Result<()> {
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

    pub fn ziggy_run(&self) -> io::Result<()> {
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
