pub mod samples;

use crate::shared::samples::Sample;
use anyhow::{
    bail,
    ensure,
    Context,
    Result,
};
use assert_cmd::Command;
use phink_lib::cli::config::Configuration;

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

pub const DEFAULT_TEST_PHINK_TOML: &str = "phink_temp_test.toml";

/// # Description
/// Helper for running tests, ensuring that the instrumented
/// folder and fuzzing output are absent. It removes the temporary forked
/// Phink config file, executes the tests, and cleans everything up afterward.
///
/// # Arguments
///
/// * `config`: A `Configuration` struct, the same one used for CLI
/// * `executed_test`: The function being executed that effectively performs the tests, i.e
///   functions containing `ensure!`
///
/// # Example
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
#[allow(clippy::zombie_processes)]
pub fn ensure_while_fuzzing<F>(
    config: &Configuration,
    timeout: Duration,
    mut executed_test: F,
) -> Result<()>
where
    F: FnMut() -> Result<()>,
{
    // We start the fuzzer
    let mut child = fuzz_test(config.verbose, DEFAULT_TEST_PHINK_TOML)?;

    let start_time = Instant::now();

    // When the fuzzer is popped, we check if the test pass. If it does, we kill Ziggy and
    // we `Ok(())`, we don't need to fuzz furthermore.
    loop {
        let test_result = executed_test();
        if test_result.is_ok() {
            println!(
                "Fuzzing test passed in {} seconds",
                start_time.elapsed().as_secs()
            );
            child.kill().with_context(|| "Failed to kill Ziggy")?;
            return Ok(())
        }

        if start_time.elapsed() > timeout {
            child.kill().with_context(|| "Failed to kill Ziggy")?;
            // If we haven't return `Ok(())` early on, we `Err()` because we timeout.
            bail!(
                "Couldn't check the assert within the given timeout. Here is the latest error we've got: {:?}", test_result.unwrap_err()
            );
        }

        // We perform the tests every `1` second
        thread::sleep(Duration::from_secs(1));
    }
}

// #[must_use]
// pub fn afl_log_didnt_fail(output: &Path) -> bool {
//     let log_path = output.join("phink").join("logs").join("afl.log");
//
//     match fs::read_to_string(log_path) {
//         Ok(content) => {
//             // this is a string that is present in AFL dashboard
//             content.contains("findings in depth")
//         }
//         _ => false,
//     }
// }

/// Try to clean up the path where the instrumented contract is. If it fails, it doesn't matter
pub fn try_cleanup_instrumented(config: &Configuration) {
    let _ = fs::remove_dir_all(
        config
            .to_owned()
            .instrumented_contract_path
            .unwrap_or_default()
            .path,
    );
}

/// Try to clean up the path where the output of the fuzzing campaign is. If it fails, it doesn't
/// matter
pub fn try_cleanup_fuzzoutput(config: &Configuration) {
    let output = config.clone().fuzz_output.unwrap_or_default();
    let result = fs::remove_dir_all(&output);
    if config.verbose {
        match result {
            Ok(()) => {
                println!("Removed {}", output.display());
            }
            Err(_) => {
                println!("*DIDN'T* removed {}", output.display());
            }
        };
    }
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

/// Simple `phink` bin pop from cargo to fuzz using `DEFAULT_TEST_PHINK_TOML`
/// ## Important
/// This should only be used in test !
pub fn fuzz_test(verbose: bool, phink_toml: &str) -> Result<Child> {
    let stdio_stdout = if verbose {
        Stdio::inherit()
    } else {
        Stdio::null()
    };

    let stdio_stderr = if verbose {
        Stdio::inherit()
    } else {
        Stdio::null()
    };
    let child = NativeCommand::new("cargo")
        .arg("run")
        .arg("--")
        .args(["--config", phink_toml])
        .arg("fuzz")
        .stdout(stdio_stdout)
        .stderr(stdio_stderr)
        .spawn();

    match child {
        Ok(child) => Ok(child),
        Err(e) => {
            bail!(format!("{e:?}"))
        }
    }
}

/// Returns `true` if `matching_string` is found in any `*.rs` file of `dir`, otherwise `false`
#[must_use]
fn find_string_in_rs_files(dir: &Path, matching_string: &str) -> bool {
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
            if find_string_in_rs_files(&path, matching_string) {
                return true;
            }
        } else if path.extension() == Some(OsStr::new("rs"))
            && file_contains_string(&path, matching_string)
        {
            return true;
        }
    }

    false
}

pub fn is_instrumented(path_instrumented_contract: &Path) -> bool {
    find_string_in_rs_files(
        path_instrumented_contract,
        "ink::env::debug_println!(\"COV={}\",",
    )
}

pub fn is_compiled(path_instrumented_contract: &Path) -> bool {
    let target_dir = path_instrumented_contract.join("target");

    if !target_dir.exists() {
        return false
    }
    target_dir.join("debug").exists() || target_dir.join("release").exists()
}

/// A function to get all entries from the corpus directory
pub fn get_corpus_files(corpus_path: &PathBuf) -> Result<HashSet<PathBuf>> {
    let corpus_files = fs::read_dir(corpus_path)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect::<HashSet<PathBuf>>();

    println!(
        "Got {} corpus files in {:?}",
        corpus_files.len(),
        corpus_path.canonicalize()?.to_str().unwrap()
    );

    Ok(corpus_files)
}
