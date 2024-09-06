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

use crate::cli::{
    config::{
        PFiles::{
            AllowlistPath,
            CoverageTracePath,
            DictPath,
        },
        PhinkFiles,
    },
    env::{
        PhinkEnv,
        PhinkEnv::{
            AflDebug,
            AflForkServerTimeout,
        },
    },
    ui,
};
use anyhow::{
    bail,
    Context,
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
use PhinkEnv::{
    AflLLVMAllowList,
    FromZiggy,
    FuzzingWithConfig,
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

    pub fn instrumented_path(self) -> PathBuf {
        self.config
            .instrumented_contract_path
            .unwrap_or_default()
            .path
    }

    pub fn parse(config_str: String) -> Self {
        let config: Self = serde_json::from_str(&config_str).expect("❌ Failed to parse config");
        if config.config.verbose {
            println!("🖨️ Using {} = {config_str}", FuzzingWithConfig);
        }
        config
    }

    /// This function executes `cargo ziggy 'command' 'args'`
    fn start(
        &self,
        command: ZiggyCommand,
        args: Vec<String>,
        env: Vec<(String, String)>,
    ) -> anyhow::Result<()> {
        let ziggy_command: String = match command {
            ZiggyCommand::Run => "run",
            ZiggyCommand::Cover => "cover",
            ZiggyCommand::Build => {
                self.build_llvm_allowlist()
                    .context("Buildin't LLVM allowlist failed")?;
                "build"
            }
            ZiggyCommand::Fuzz(..) => "fuzz",
        }
        .parse()?;

        if let ZiggyCommand::Fuzz(ui) = command {
            if ui {
                ui::tui::initialize_tui().unwrap();
            } else {
                self.native_ui(args, env, ziggy_command)?;
            }
        }

        Ok(())
    }

    fn native_ui(
        &self,
        args: Vec<String>,
        env: Vec<(String, String)>,
        ziggy_command: String,
    ) -> anyhow::Result<()> {
        use std::io::BufRead;
        let mut binding = Command::new("cargo");
        let command_builder = binding
            .arg("ziggy")
            .arg(ziggy_command)
            .env(FromZiggy.to_string(), "1")
            .env(AflForkServerTimeout.to_string(), AFL_FORKSRV_INIT_TMOUT)
            .env(AflDebug.to_string(), AFL_DEBUG)
            .stdout(Stdio::piped());

        let mut ziggy_child = command_builder
            .spawn()
            .context("Spawning Ziggy was unsuccessfull")?;

        if let Some(stdout) = ziggy_child.stdout.take() {
            let reader = io::BufReader::new(stdout);
            for line in reader.lines() {
                println!("{}", line?);
            }
        }

        let status = ziggy_child.wait().context("Couldn't wait for Ziggy")?;
        if !status.success() {
            bail!("`cargo ziggy` failed ({})", status);
        }
        self.with_allowlist(command_builder)
            .context("Couldn't use the allowlist")?;

        // If there are additional arguments, pass them to the command
        command_builder.args(args.iter());
        command_builder.envs(env);
        command_builder.spawn().context("Couldn't spawn Ziggy")?;
        Ok(())
    }

    /// Add the ALLOWLIST file to a `Command`. This will be done only in not on macOS
    ///     - see https://github.com/rust-lang/rust/issues/127573
    ///     - see https://github.com/rust-lang/rust/issues/127577
    /// # Arguments
    ///
    /// * `command_builder`: The prepared command to which we'll add the AFL ALLOWLIST
    fn with_allowlist(&self, command_builder: &mut Command) -> anyhow::Result<()> {
        if cfg!(not(target_os = "macos")) {
            let allowlist = PhinkFiles::new(self.config.fuzz_output.to_owned().unwrap_or_default())
                .path(AllowlistPath);
            command_builder.env(
                AflLLVMAllowList.to_string(),
                allowlist
                    .canonicalize()
                    .context("Couldn't canonicalize the allowlist path")?,
            );
        }
        Ok(())
    }

    pub fn ziggy_fuzz(&self) -> anyhow::Result<()> {
        let fuzzoutput = &self.config.fuzz_output;
        let dict = PhinkFiles::new(fuzzoutput.to_owned().unwrap_or_default()).path(DictPath);

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
            format!("--minlength={MIN_SEED_LEN}"),
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

        let fuzz_config = vec![(FuzzingWithConfig.to_string(), serde_json::to_string(self)?)];

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
            vec![(FuzzingWithConfig.to_string(), serde_json::to_string(self)?)],
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
            vec![(FuzzingWithConfig.to_string(), serde_json::to_string(self)?)],
        )?;
        Ok(())
    }

    /// Builds the LLVM allowlist if it doesn't already exist.
    fn build_llvm_allowlist(&self) -> io::Result<()> {
        let allowlist_path = PhinkFiles::new(self.config.fuzz_output.clone().unwrap_or_default())
            .path(AllowlistPath);

        if allowlist_path.exists() {
            println!("❗ {} already exists... skipping", AflLLVMAllowList);
            return Ok(());
        }

        fs::create_dir_all(allowlist_path.parent().unwrap())?;
        let mut allowlist_file = File::create(allowlist_path)?;

        let functions = ["redirect_coverage*", "should_stop_now*", "parse_input*"];
        for func in &functions {
            writeln!(allowlist_file, "fun: {}", func)?;
        }

        println!("✅ {} created successfully", AflLLVMAllowList);
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;

    fn create_test_config() -> ZiggyConfig {
        let config = Configuration {
            verbose: true,
            cores: Some(4),
            use_honggfuzz: false,
            fuzz_output: Some(PathBuf::from("/tmp/fuzz_output")),
            show_ui: false,
            ..Default::default()
        };
        ZiggyConfig::new(config, PathBuf::from("/path/to/contract"))
    }

    #[test]
    fn test_ziggy_config_new() {
        let config = create_test_config();
        assert!(config.config.verbose);
        assert_eq!(config.config.cores, Some(4));
        assert!(!config.config.use_honggfuzz);
        assert_eq!(
            config.config.fuzz_output,
            Some(PathBuf::from("/tmp/fuzz_output"))
        );
        assert_eq!(config.contract_path, PathBuf::from("/path/to/contract"));
    }

    #[test]
    fn test_ziggy_config_parse() {
        let config_str = r#"
                {
                   "config":{
                      "cores":2,
                      "use_honggfuzz":false,
                      "deployer_address":"5C62Ck4UrFPiBtoCmeSrgF7x9yv9mn38446dhCpsi2mLHiFT",
                      "max_messages_per_exec":4,
                      "report_path":"output/phink/contract_coverage",
                      "fuzz_origin":false,
                      "default_gas_limit":{
                         "ref_time":100000000000,
                         "proof_size":3145728
                      },
                      "storage_deposit_limit":"100000000000",
                      "instantiate_initial_value":"0",
                      "constructor_payload":"9BAE9D5E5C1100007B000000279C603E9D4B5C6C8C672893AB54D068CECCBFBEC619E56E819A7769EADCBD766D714E7624D4BE6A35BED20D0730277D0F3A13A7B01DCDA7CEDBF67FE3A4E95F0758D2DF54F30DD663424723E09A56B19E1325B830E6CCCCF63C6FF12B78C79A",
                      "verbose":false,
                      "show_ui":true
                   },
                   "contract_path":"/tmp/ink_fuzzed_3h4Wm/"
                }
        "#;
        let config = ZiggyConfig::parse(config_str.to_string());
        assert!(!config.config.verbose);
        assert!(config.config.show_ui);
        assert!(!config.config.use_honggfuzz);
        assert_eq!(
            config.config.storage_deposit_limit,
            Some("100000000000".into())
        );
        assert_eq!(config.config.cores, Some(2));
        assert_eq!(config.config.fuzz_output, Default::default());
        assert_eq!(
            config.contract_path,
            PathBuf::from("/tmp/ink_fuzzed_3h4Wm/")
        );
    }

    #[test]
    fn test_build_llvm_allowlist() -> io::Result<()> {
        let temp_dir = tempdir()?;
        let config = Configuration {
            fuzz_output: Some(temp_dir.path().to_path_buf()),
            ..Default::default()
        };
        let ziggy_config = ZiggyConfig::new(config, PathBuf::from("/path/to/contract"));

        ziggy_config.build_llvm_allowlist()?;

        let allowlist_path = PhinkFiles::new(temp_dir.path().to_path_buf()).path(AllowlistPath);
        assert!(allowlist_path.exists());

        let contents = fs::read_to_string(allowlist_path)?;
        assert!(contents.contains("fun: redirect_coverage*"));
        assert!(contents.contains("fun: should_stop_now*"));
        assert!(contents.contains("fun: parse_input*"));

        Ok(())
    }

    #[test]
    fn test_with_allowlist() -> anyhow::Result<()> {
        if cfg!(not(target_os = "macos")) {
            let temp_dir = tempdir()?;
            let config = Configuration {
                fuzz_output: Some(temp_dir.path().to_path_buf()),
                ..Default::default()
            };
            let ziggy_config = ZiggyConfig::new(config, PathBuf::from("/path/to/contract"));

            ziggy_config.build_llvm_allowlist()?;

            let mut command = Command::new("echo");
            ziggy_config.with_allowlist(&mut command)?;

            let allowlist_path = PhinkFiles::new(temp_dir.path().to_path_buf()).path(AllowlistPath);
            let env_vars: Vec<(String, String)> = command
                .get_envs()
                .map(|(k, v)| {
                    (
                        k.to_str().unwrap().to_string(),
                        v.unwrap().to_str().unwrap().to_string(),
                    )
                })
                .collect();

            assert!(env_vars.contains(&(
                AflLLVMAllowList.to_string(),
                allowlist_path.to_str().unwrap().to_string()
            )));
        }
        Ok(())
    }

    #[test]
    fn test_start_build_command() -> anyhow::Result<()> {
        let config = create_test_config();
        let temp_dir = tempdir()?;

        env::set_var("CARGO_MANIFEST_DIR", temp_dir.path());

        let result = config.start(
            ZiggyCommand::Build,
            vec!["--no-honggfuzz".to_string()],
            vec![],
        );
        assert!(result.is_ok());

        Ok(())
    }
}
