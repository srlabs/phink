use crate::cli::{
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
use anyhow::Context;
use std::{
    process::{
        Child,
        Command,
        Stdio,
    },
    sync::mpsc,
    thread,
};

#[derive(Clone, Debug)]
pub struct CustomManager {
    args: Vec<String>,
    env: Vec<(String, String)>,
    ziggy_config: ZiggyConfig,
}

impl CustomManager {
    pub fn new(args: Vec<String>, env: Vec<(String, String)>, ziggy_config: ZiggyConfig) -> Self {
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

        let pid: Child = rx.recv()??;

        let ratatui = CustomUI::new(&cloned_config).context("Couldn't create the custom UI ")?;

        ratatui.initialize_tui(pid)?;
        Ok(())
    }
}
