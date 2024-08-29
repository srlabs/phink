pub mod samples;

use crate::shared::samples::Sample;
use anyhow::{
    anyhow,
    Context,
    Result,
};
use assert_cmd::Command;
use phink_lib::{
    cli::config::Configuration,
    instrumenter::instrumented_path::InstrumentedPath,
};

use std::{
    collections::HashSet,
    ffi::OsStr,
    fs,
    io,
    io::Read,
    path::{
        Path,
        PathBuf,
    },
    process::{
        Child,
        Command as NativeCommand,
        Stdio,
    },
    thread,
    time::{
        Duration,
        Instant,
    },
};

pub const DEFAULT_TEST_PHINK_TOML: &str = "phink_temp_test.toml";

/// This function is a helper used to run tests, it mainly checks that the instrumented
/// folder doesn't exist yet, same for the fuzzing output, remove the temporary forked
/// Phink configuration file, executes the tests and then clean everything again
///
/// # Arguments
///
/// * `config`: A `Configuration` struct, the same one used for CLI
/// * `executed_test`: The function being executed that effectively performs the tests,
///   i.e functions containing `ensure!`
/// # Examples
///
/// ```
/// with_modified_phink_config(config.clone(), || {
///     instrument(Sample::Dummy);
///     Ok(())
/// });
/// ```
pub fn with_modified_phink_config<F>(
    config: &Configuration,
    executed_test: F,
) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    let _ = fs::remove_dir_all(
        &config
            .instrumented_contract_path
            .clone()
            .unwrap_or_default()
            .path,
    );
    let _ = fs::remove_dir_all(&config.fuzz_output.clone().unwrap_or_default());

    config.save_as_toml(DEFAULT_TEST_PHINK_TOML);

    // Executing the actual test
    let test_result = executed_test();

    // We remove the temp config file
    let _ = fs::remove_file(DEFAULT_TEST_PHINK_TOML);
    // We clean the instrumented path
    let _ = fs::remove_dir_all(
        &config
            .instrumented_contract_path
            .clone()
            .unwrap_or_default()
            .path,
    );
    let _ = fs::remove_dir_all(config.fuzz_output.clone().unwrap_or_default());

    test_result
}

/// This function is a helper that pops the fuzzer on a given `Configuration`, and execute
/// some tests during the campaign.
///
/// In other words, the test to be executed have a specific time (`timeout`) to pass. If
/// they can't `Ok()` until then, it fails.
/// # Arguments
///
/// * `config`: A `Configuration` struct, the same one used for CLI
/// * `timeout`: A timeout where the test would be considered as failed if the conditions
///   inside `executed_test` couldn't be met (i.e, it couldn't `Ok(())`)
/// * `executed_test`: The function being executed that effectively performs the tests,
///   i.e functions containing `ensure`
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
/// let test_passed = verify_during_fuzz(&config, Duration::from_secs(30), || {
///     ensure!(true);
///     Ok(())
/// });
/// ensure!(test_passed.is_ok());
/// ```
pub fn ensure_while_fuzzing<F>(
    config: &Configuration,
    timeout: Duration,
    mut executed_test: F,
) -> Result<()>
where
    F: FnMut() -> Result<()>,
{
    // We start the fuzzer
    let mut child = fuzz(
        config
            .clone()
            .instrumented_contract_path
            .unwrap_or_default(),
    );

    let start_time = Instant::now();

    // When the fuzzer is popped, we check if the test pass. If it does, we kill Ziggy and
    // we `Ok(())`
    loop {
        if let Ok(_) = executed_test() {
            child.kill().context("Failed to kill Ziggy")?;
            return Ok(());
        }

        if start_time.elapsed() > timeout {
            child.kill().context("Failed to kill Ziggy")?;
            // If we haven't return `Ok(())` early on, we `Err()` because we timeout.
            return Err(anyhow!(
                "Couldn't check the assert within the given timeout"
            ));
        }

        thread::sleep(Duration::from_secs(1));
    }
}
pub fn instrument(contract_path: Sample) {
    let mut cmd = Command::cargo_bin("phink").unwrap();
    let _binding = cmd
        .args(["--config", DEFAULT_TEST_PHINK_TOML])
        .arg("instrument")
        .arg(contract_path.path())
        .assert()
        .success();
}

pub fn fuzz(path_instrumented_contract: InstrumentedPath) -> Child {
    let mut child = NativeCommand::new("cargo")
        .arg("run")
        .arg("--")
        .args(["--config", DEFAULT_TEST_PHINK_TOML])
        .arg("fuzz")
        .arg(path_instrumented_contract.path.to_str().unwrap())
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start the process");
    child
}

pub fn find_string_in_rs_files(dir: &Path, target: &str) -> bool {
    fn file_contains_string(file_path: &Path, target: &str) -> bool {
        let mut file = fs::File::open(file_path).expect("Unable to open file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Unable to read file");
        content.contains(target)
    }

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.expect("Unable to get directory entry");
        let path = entry.path();

        if path.is_dir() {
            if find_string_in_rs_files(&path, target) {
                return true;
            }
        } else if path.extension() == Some(OsStr::new("rs")) {
            if file_contains_string(&path, target) {
                return true;
            }
        }
    }

    false
}

pub fn get_corpus_files(corpus_path: &PathBuf) -> HashSet<PathBuf> {
    println!("Got corpus files in: {:?}", corpus_path);
    fs::read_dir(corpus_path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect()
}
