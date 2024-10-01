use crate::cli::{
    config::{
        PFiles,
        PhinkFiles,
    },
    env::PhinkEnv::{
        AflDebug,
        AflForkServerTimeout,
        FromZiggy,
    },
    ui::ratatui::CustomUI,
    ziggy::{
        ZiggyConfig,
        AFL_FORKSRV_INIT_TMOUT,
    },
};
use anyhow::{
    bail,
    Context,
};
use std::{
    fs,
    process::{
        Child,
        Command,
        Stdio,
    },
    sync::mpsc,
    thread,
    thread::sleep,
    time::{
        Duration,
        Instant,
    },
};

#[derive(Clone, Debug)]
pub struct CustomManager {
    args: Vec<String>,
    env: Vec<(String, String)>,
    ziggy_config: ZiggyConfig,
}

impl CustomManager {
    pub fn new(args: Vec<String>, env: Vec<(String, String)>, ziggy_config: ZiggyConfig) -> Self {
        // If it exists, we remove the `LAST_SEED_FILENAME`
        let _ = fs::remove_file(
            PhinkFiles::new(ziggy_config.config.fuzz_output.clone().unwrap())
                .path(PFiles::LastSeed),
        );

        Self {
            args,
            env,
            ziggy_config,
        }
    }

    pub fn cmd_fuzz(self) -> anyhow::Result<Child> {
        let mut binding = Command::new("cargo");
        let command_builder = binding
            .arg("ziggy")
            .arg("fuzz")
            .env(FromZiggy.to_string(), "1")
            .env(AflForkServerTimeout.to_string(), AFL_FORKSRV_INIT_TMOUT)
            .env(AflDebug.to_string(), self.ziggy_config.afl_debug())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        self.ziggy_config
            .with_allowlist(command_builder)
            .context("Couldn't use the allowlist")?;

        command_builder.args(self.args.iter());
        command_builder.envs(self.env);

        let child = command_builder
            .spawn()
            .context("Spawning Ziggy was unsuccessful")?;

        Ok(child)
    }

    pub fn start(self) -> anyhow::Result<()> {
        let (tx, rx) = mpsc::channel();
        let cloned_config = self.ziggy_config.clone();

        thread::spawn(move || {
            let fuzzer_pid = self.cmd_fuzz();
            tx.send(fuzzer_pid).unwrap();
        });

        let child: Child = rx.recv()??;

        let mut ratatui = CustomUI::new(&cloned_config);
        let start_time = Instant::now();
        let mut print_err = true;
        loop {
            if start_time.elapsed() > Duration::new(120, 0) {
                bail!("Couldn't instantiate the custom UI within 120 seconds...");
            }
            if ratatui.is_err() {
                if print_err {
                    println!("⏰ Waiting for AFL++ to finish the dry run ");
                    print_err = false;
                }
                ratatui = CustomUI::new(&cloned_config);
                sleep(Duration::from_millis(100));
            } else {
                break;
            }
        }

        ratatui
            .context("Couldn't create ratatui with 'new'")?
            .initialize_tui(child)
            .context("Couldn't initialize ratatui")?;
        Ok(())
    }
}
