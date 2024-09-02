pub mod samples;

use crate::shared::samples::Sample;
use anyhow::{
    anyhow,
    ensure,
    Context,
    Result,
};
use assert_cmd::Command;
use phink_lib::{
    cli::config::Configuration,
    instrumenter::instrumented_path::InstrumentedPath,
};

use assert_cmd::assert::Assert;
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs,
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
#[allow(clippy::too_long_first_doc_paragraph)]

pub const DEFAULT_TEST_PHINK_TOML: &str = "phink_temp_test.toml";

/// This function is a helper used to run tests, it mainly checks that the instrumented
/// folder doesn't exist yet, same for the fuzzing output, remove the temporary forked
/// Phink configuration file, executes the tests and then clean everything again
///
/// # Arguments
///
/// * `config`: A `Configuration` struct, the same one used for CLI
/// * `executed_test`: The function being executed that effectively performs the tests, i.e
///   functions containing `ensure!`
/// # Examples
///
/// ```
/// with_modified_phink_config(config.clone(), || {
///     instrument(Sample::Dummy);
///     Ok(())
/// });
/// ```
pub fn with_modified_phink_config<F>(config: &Configuration, executed_test: F) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    try_cleanup_instrumented(config);
    try_cleanup_fuzzoutput(config);
    config.save_as_toml(DEFAULT_TEST_PHINK_TOML)?;

    // Executing the actual test
    let test_result = executed_test();

    // Ensure the config file wasn't removed by the tests (it shouldn't)
    ensure!(
        PathBuf::from(DEFAULT_TEST_PHINK_TOML).exists(),
        "{DEFAULT_TEST_PHINK_TOML} doesn't exist"
    );

    // We remove the temp config file
    let _ = fs::remove_file(DEFAULT_TEST_PHINK_TOML);
    // We clean the instrumented path
    try_cleanup_instrumented(config);
    try_cleanup_fuzzoutput(config);

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
/// * `timeout`: A timeout where the test would be considered as failed if the conditions inside
///   `executed_test` couldn't be met (i.e, it couldn't `Ok(())`)
/// * `executed_test`: The function being executed that effectively performs the tests, i.e
///   functions containing `ensure`
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
    // we `Ok(())`, we don't need to fuzz furthermore.
    loop {
        let test_result = executed_test();
        if test_result.is_ok() {
            child.kill().context("Failed to kill Ziggy")?;
            return Ok(());
        }

        if start_time.elapsed() > timeout {
            child.kill().context("Failed to kill Ziggy")?;
            // If we haven't return `Ok(())` early on, we `Err()` because we timeout.
            return bail!(
                "Couldn't check the assert within the given timeout. Here is the latest error we've got: {:?}", test_result.unwrap_err()
            );
        }

        // We perform the tests every `1` second
        thread::sleep(Duration::from_secs(1));
    }
}

#[must_use]
pub fn afl_log_didnt_fail(output: &Path) -> bool {
    let log_path = output.join("logs").join("afl.log");

    match fs::read_to_string(log_path) {
        Ok(content) => {
            // this is a string that is present in AFL dashboard
            content.contains("findings in depth")
        }
        _ => false,
    }
}

/// Try to clean up the path where the instrumented contract is. If it fails, it doesn't matter
pub fn try_cleanup_instrumented(config: &Configuration) {
    let _ = fs::remove_dir_all(config.clone().instrumented_contract_path.unwrap().path);
}

/// Try to clean up the path where the output of the fuzzing campaign is. If it fails, it doesn't
/// matter
pub fn try_cleanup_fuzzoutput(config: &Configuration) {
    let output = config.clone().fuzz_output.unwrap_or_default();
    match fs::remove_dir_all(&output) {
        Ok(()) => {
            println!("Removed {}", output.display());
        }
        Err(_) => {
            println!("**DIDN'T** removed {}", output.display());
        }
    };
}

/// Simple `phink` bin pop from cargo to instrument `contract_path`
/// ** Important **
/// This should only be used in test !
#[must_use]
pub fn instrument(contract_path: Sample) -> Assert {
    let mut cmd = Command::cargo_bin("phink").unwrap();
    cmd.args(["--config", DEFAULT_TEST_PHINK_TOML])
        .arg("instrument")
        .arg(contract_path.path())
        .assert()
        .success()
}

/// Simple `phink` bin pop from cargo to fuzz `path_instrumented_contract`
/// ** Important **
/// This should only be used in test !
#[must_use]
pub fn fuzz(path_instrumented_contract: InstrumentedPath) -> Child {
    let child = NativeCommand::new("cargo")
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

/// Return `true` if `target` is found in any `*.rs` file of `dir`, otherwise `false`
#[must_use]
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
        } else if path.extension() == Some(OsStr::new("rs")) && file_contains_string(&path, target)
        {
            return true;
        }
    }

    false
}

/// A function to get all entries from the corpus directory
pub fn get_corpus_files(corpus_path: &PathBuf) -> Result<HashSet<PathBuf>> {
    println!(
        "Got corpus files in: {:?}",
        corpus_path.canonicalize()?.to_str().unwrap()
    );
    let corpus_files = fs::read_dir(corpus_path)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect::<HashSet<PathBuf>>();

    Ok(corpus_files)
}
