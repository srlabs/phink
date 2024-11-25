use crate::{
    cli::config::Configuration,
    fuzzer::parser::MIN_SEED_LEN,
    EmptyResult,
    ResultOf,
};
use io::BufReader;
use std::io::BufRead;

use serde_derive::{
    Deserialize,
    Serialize,
};
use std::{
    fs,
    path::PathBuf,
};

use crate::{
    cli::{
        config::{
            PFiles,
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
        ui::custom::CustomManager,
    },
    fuzzer::environment::AllowListBuilder,
};
use anyhow::{
    bail,
    Context,
};
use std::{
    cmp::PartialEq,
    fmt::{
        Display,
        Formatter,
    },
    io::{
        self,
    },
    process::{
        Command,
        Stdio,
    },
};
use PhinkEnv::{
    AllowList,
    FromZiggy,
    FuzzingWithConfig,
};

pub const AFL_FORKSRV_INIT_TMOUT: &str = "10000000";

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ZiggyCommand {
    Run,
    Cover,
    Build,
    Fuzz,
    Minimize,
}

impl Display for ZiggyCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cmd: &str = match &self {
            ZiggyCommand::Run => "run",
            ZiggyCommand::Cover => "cover",
            ZiggyCommand::Build => "build",
            ZiggyCommand::Fuzz => "fuzz",
            ZiggyCommand::Minimize => "minimize",
        };
        write!(f, "{}", cmd)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZiggyConfig {
    config: Configuration,
    contract_path: Option<PathBuf>,
}

impl Display for ZiggyConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl ZiggyConfig {
    pub fn new(config: Configuration) -> ResultOf<Self> {
        Self::is_valid(&config, None)?;

        Ok(Self {
            config,
            contract_path: None,
        })
    }

    pub fn new_with_contract(config: Configuration, contract_path: PathBuf) -> ResultOf<Self> {
        Self::is_valid(&config, Some(&contract_path))?;

        Ok(Self {
            config,
            contract_path: Some(contract_path),
        })
    }

    pub fn config(&self) -> &Configuration {
        &self.config
    }

    /// Returns the contract path of the ink! contract
    pub fn contract_path(&self) -> ResultOf<PathBuf> {
        self.contract_path.to_owned().context(
            "Contract path wasn't passed in the config, it is currently `None`.\
            Ensure that your `phink.toml` is properly configured",
        )
    }
    fn is_valid(config: &Configuration, contract_path: Option<&PathBuf>) -> EmptyResult {
        if let Some(path) = contract_path {
            if !path.exists() {
                bail!(format!(
                    "{path:?} doesn't exist; couldn't load this contract"
                ))
            }
        }

        if config.use_honggfuzz {
            bail!(
                "Please, set `use_honggfuzz` to `false`, as we do not currently support Honggfuzz
        due to ALLOW_LIST limitations in Honggfuzz"
            )
        }

        Ok(())
    }

    pub fn fuzz_output(self) -> PathBuf {
        self.config.fuzz_output.unwrap_or_default()
    }

    pub fn afl_debug<'a>(&self) -> &'a str {
        match self.config().verbose {
            true => "1",
            false => "0",
        }
    }

    pub fn parse(config_str: String) -> ResultOf<Self> {
        let config: Self =
            serde_json::from_str(&config_str).context("❌ Failed to parse config")?;
        if config.config().verbose {
            println!("🖨️ Using {} = {config_str}\n", FuzzingWithConfig);
        }
        Ok(config)
    }

    /// This function executes 'cargo ziggy `command` `args`'
    fn build_command(
        &self,
        command: ZiggyCommand,
        args: Option<Vec<String>>,
        env: Vec<(String, String)>,
    ) -> EmptyResult {
        AllowListBuilder::build(self.clone().fuzz_output())
            .context("Building LLVM allowlist failed")?;

        match command {
            ZiggyCommand::Cover | ZiggyCommand::Run | ZiggyCommand::Minimize => {
                self.exist_or_bail()?;
                self.native_ui(args, env, command)?;
            }
            ZiggyCommand::Fuzz => {
                self.exist_or_bail()?;
                if self.config.show_ui {
                    CustomManager::new(args, env, self.to_owned()).start()?;
                } else {
                    self.native_ui(args, env, command)?;
                }
            }
            ZiggyCommand::Build => {
                self.native_ui(args, env, command)?;
            }
        }

        Ok(())
    }

    fn exist_or_bail(&self) -> EmptyResult {
        let loc = &self.config().instrumented_contract();
        if !loc.exists() {
            bail!(format!(
                "The instrumented contract path `{}` doesn't exist, \
                ensure that you have properly instrumented your contract to the correct location",
                loc.to_str().unwrap()
            ))
        }
        Ok(())
    }

    fn native_ui(
        &self,
        maybe_args: Option<Vec<String>>,
        env: Vec<(String, String)>,
        ziggy_command: ZiggyCommand,
    ) -> EmptyResult {
        let mut binding = Command::new("cargo");
        let command_builder = binding
            .arg("ziggy")
            .arg(ziggy_command.to_string())
            .env(FromZiggy.to_string(), "1")
            .env(AflForkServerTimeout.to_string(), AFL_FORKSRV_INIT_TMOUT)
            .env(AflDebug.to_string(), self.afl_debug())
            .envs(env)
            .stdout(Stdio::piped());

        let output = self.to_owned().fuzz_output();
        let buf = PhinkFiles::new_by_ref(&output).path(PFiles::CorpusPath);
        let corpus = buf.to_str().unwrap();

        match ziggy_command {
            ZiggyCommand::Run => {
                command_builder.args(vec!["--inputs", corpus]);
                command_builder.args(vec!["--ziggy-output", output.to_str().unwrap()]);
            }
            ZiggyCommand::Minimize => {
                command_builder.args(vec!["--input-corput", corpus]);
                command_builder.args(vec!["--ziggy-output", output.to_str().unwrap()]);
                command_builder.args(vec!["--engine", "afl-plus-plus"]); // don't minimize with
                                                                         // honggfuzz
            }
            _ => {}
        }

        self.with_allowlist(command_builder)
            .context("Couldn't use the allowlist")?;

        if let Some(args) = maybe_args {
            command_builder.args(args.iter());
        }

        let mut ziggy_child = command_builder
            .spawn()
            .context("Spawning Ziggy was unsuccessfull..")?;

        if let Some(stdout) = ziggy_child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                println!("{}", line?);
            }
        }

        let status = ziggy_child.wait().context("Couldn't wait for Ziggy")?;
        if !status.success() {
            bail!("`cargo ziggy {ziggy_command}` failed ({status})");
        }

        Ok(())
    }

    /// Add the ALLOW_LIST file to a `Command`. This will be done only in not on macOS
    ///     - see https://github.com/rust-lang/rust/issues/127573
    ///     - see https://github.com/rust-lang/rust/issues/127577
    /// # Arguments
    ///
    /// * `command_builder`: The prepared command to which we'll add the AFL ALLOWLIST
    pub fn with_allowlist(&self, command_builder: &mut Command) -> EmptyResult {
        if cfg!(not(target_os = "macos")) {
            let allowlist = PhinkFiles::new(self.clone().fuzz_output()).path(AllowlistPath);
            command_builder.env(
                AllowList.to_string(),
                allowlist
                    .canonicalize()
                    .context("Couldn't canonicalize the allowlist path")?,
            );
        } else if self.config.verbose {
            println!("This is a macOS machine. We won't use the ALLOW_LIST. Performances will be drastically bad...");
        }
        Ok(())
    }

    pub fn ziggy_fuzz(&self) -> EmptyResult {
        let fuzzoutput = &self.config.fuzz_output;
        let dict = PhinkFiles::new(fuzzoutput.to_owned().unwrap_or_default()).path(DictPath);

        let build_args = if !self.config.use_honggfuzz {
            Some(vec!["--no-honggfuzz".parse()?])
        } else {
            None
        };

        self.build_command(ZiggyCommand::Build, build_args, vec![])?;

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

        self.build_command(ZiggyCommand::Fuzz, Some(fuzzing_args), fuzz_config)
    }

    pub fn ziggy_cover(&self) -> EmptyResult {
        self.build_command(
            ZiggyCommand::Cover,
            None,
            vec![(FuzzingWithConfig.to_string(), serde_json::to_string(self)?)],
        )?;
        Ok(())
    }

    pub fn ziggy_minimize(&self) -> EmptyResult {
        self.build_command(
            ZiggyCommand::Minimize,
            None,
            vec![(FuzzingWithConfig.to_string(), serde_json::to_string(self)?)],
        )?;
        Ok(())
    }

    pub fn ziggy_run(&self) -> EmptyResult {
        let covpath = PhinkFiles::new(self.clone().fuzz_output()).path(CoverageTracePath);

        // We clean up the old one first
        if fs::remove_file(&covpath).is_ok() {
            println!("💨 Removed previous coverage file at {covpath:?}")
        }

        self.build_command(
            ZiggyCommand::Run,
            None,
            vec![(FuzzingWithConfig.to_string(), serde_json::to_string(self)?)],
        )?;
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
        ZiggyConfig::new_with_contract(config, PathBuf::from("sample/dummy")).unwrap()
    }

    #[test]
    fn test_ziggy_config_new() {
        let config = create_test_config();
        assert!(config.config().verbose);
        assert_eq!(config.config().cores, Some(4));
        assert!(!config.config().use_honggfuzz);
        assert_eq!(
            config.config().fuzz_output,
            Some(PathBuf::from("/tmp/fuzz_output"))
        );
        assert_eq!(
            config.contract_path().unwrap(),
            PathBuf::from("sample/dummy")
        );
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
                      "catch_trapped_contract": false,
                      "show_ui":true
                   },
                   "contract_path":"/tmp/ink_fuzzed_3h4Wm/"
                }
        "#;
        let config = ZiggyConfig::parse(config_str.to_string()).unwrap();
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
            config.contract_path().unwrap(),
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
        let ziggy_config =
            ZiggyConfig::new_with_contract(config, PathBuf::from("sample/dummy")).unwrap();

        AllowListBuilder::build(ziggy_config.fuzz_output())?;

        let allowlist_path = PhinkFiles::new(temp_dir.path().to_path_buf()).path(AllowlistPath);
        assert!(allowlist_path.exists());

        let contents = fs::read_to_string(allowlist_path)?;
        assert!(contents.contains("fun: redirect_coverage*"));
        assert!(contents.contains("fun: parse_input*"));

        Ok(())
    }

    #[test]
    fn test_with_allowlist() -> EmptyResult {
        if cfg!(not(target_os = "macos")) {
            let temp_dir = tempdir()?;
            let config = Configuration {
                fuzz_output: Some(temp_dir.path().to_path_buf()),
                ..Default::default()
            };
            let ziggy_config =
                ZiggyConfig::new_with_contract(config, PathBuf::from("sample/dummy"))?;

            AllowListBuilder::build(ziggy_config.clone().fuzz_output())?;

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
                AllowList.to_string(),
                allowlist_path.to_str().unwrap().to_string()
            )));
        }
        Ok(())
    }

    #[test]
    fn test_start_build_command() -> EmptyResult {
        let config = create_test_config();
        let temp_dir = tempdir()?;

        env::set_var("CARGO_MANIFEST_DIR", temp_dir.path());

        let result = config.build_command(
            ZiggyCommand::Build,
            Some(vec!["--no-honggfuzz".to_string()]),
            vec![],
        );

        let success = result.is_ok();
        if success {
            println!("{result:?}",);
        }
        assert!(
            success,
            "One possibility could be `cargo afl config --build --verbose --force`"
        );
        Ok(())
    }
}
