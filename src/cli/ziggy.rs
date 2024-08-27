use crate::{
    cli::config::Configuration,
    cover::coverage::COVERAGE_PATH,
    fuzzer::{
        fuzz::DICT_FILE,
        parser::MIN_SEED_LEN,
    },
};

use serde_derive::{
    Deserialize,
    Serialize,
};
use std::{
    fs,
    fs::File,
    io::Write,
    path::{
        Path,
        PathBuf,
    },
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
    thread,
    time::Duration,
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
    pub const ALLOWLIST_PATH: &'static str = "./output/phink/allowlist.txt";
    pub const AFL_DEBUG: &'static str = "1";
    pub const AFL_FORKSRV_INIT_TMOUT: &'static str = "10000000";

    pub fn new(config: Configuration, contract_path: PathBuf) -> Self {
        Self {
            config,
            contract_path,
        }
    }

    pub fn parse(config_str: String) -> Self {
        let config: Self =
            serde_json::from_str(&config_str).expect("❌ Failed to parse config");
        if config.config.verbose {
            println!("🖨️ Using PHINK_START_FUZZING_WITH_CONFIG = {}", config_str);
        }
        config
    }

    /// This function executes `cargo ziggy 'command' 'args'`
    fn start(
        command: ZiggyCommand,
        args: Vec<String>,
        env: Vec<(String, String)>,
    ) -> io::Result<()> {
        let command_arg = Self::command_to_arg(&command)?;
        let mut binding = Command::new("cargo");
        let mut command_builder = binding
            .arg("ziggy")
            .arg(command_arg)
            .env("PHINK_FROM_ZIGGY", "1")
            .env("AFL_FORKSRV_INIT_TMOUT", Self::AFL_FORKSRV_INIT_TMOUT)
            .env("AFL_DEBUG", Self::AFL_DEBUG)
            .stdout(Stdio::piped());

        // Add `AFL_LLVM_ALLOWLIST` if not on macOS
        // See https://github.com/rust-lang/rust/issues/127573
        // See https://github.com/rust-lang/rust/issues/127577
        if cfg!(not(target_os = "macos")) {
            command_builder = command_builder.env(
                "AFL_LLVM_ALLOWLIST",
                Path::new(Self::ALLOWLIST_PATH)
                    .canonicalize()?
                    .to_str()
                    .unwrap(),
            );
        }

        // If there are additional arguments, pass them to the command
        command_builder.args(args.iter());

        for (key, value) in env {
            command_builder.env(key, value);
        }

        let mut ziggy_child = command_builder.spawn()?;
        let stdout = ziggy_child.stdout.take().expect("Failed to capture stdout");

        // Wait for the child process to finish
        let status = ziggy_child.wait()?;
        if !status.success() {
            eprintln!("🚫 Can't start cargo ziggy, command failed");
        }

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
        let build_args = if !self.config.use_honggfuzz {
            vec!["--no-honggfuzz".parse().unwrap()]
        } else {
            vec!["".parse().unwrap()]
        };

        Self::start(ZiggyCommand::Build, build_args, vec![])?;

        println!("🏗️ Ziggy Build completed");

        let mut fuzzing_args = vec![
            format!("--jobs={}", self.config.cores.unwrap_or_default()),
            format!("--dict={}", DICT_FILE),
            format!("--minlength={}", MIN_SEED_LEN),
        ];
        if !self.config.use_honggfuzz {
            fuzzing_args.push("--no-honggfuzz".parse().unwrap())
        }

        if self.config.fuzz_output.is_some() {
            fuzzing_args.push(format!(
                "--ziggy-output={}",
                self.config.fuzz_output.clone().unwrap().to_str().unwrap()
            ))
        }

        let fuzz_config = vec![(
            "PHINK_START_FUZZING_WITH_CONFIG".to_string(),
            serde_json::to_string(self)?,
        )];

        Self::start(ZiggyCommand::Fuzz, fuzzing_args, fuzz_config)
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
        // We clean up the old one first
        match fs::remove_file(COVERAGE_PATH) {
            Ok(_) => println!("💨 Removed previous coverage file"),
            Err(_) => {}
        }

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

        let functions = ["redirect_coverage*", "should_stop_now*", "parse_input*"];
        for func in &functions {
            writeln!(allowlist_file, "fun: {}", func)?;
        }

        println!("✅ AFL_LLVM_ALLOWLIST created successfully");
        Ok(())
    }
}
