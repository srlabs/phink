pub mod samples;

use crate::shared::samples::Sample;
use assert_cmd::Command;
use phink_lib::{
    cli::config::Configuration,
    instrumenter::instrumented_path::InstrumentedPath,
};
use std::{
    ffi::OsStr,
    fs,
    io,
    io::Read,
    path::Path,
    process::{
        Child,
        Command as NativeCommand,
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
///   i.e functions containing `assert`
/// # Examples
///
/// ```
/// with_modified_phink_config(config.clone(), || {
///     instrument(Sample::Dummy);
///     Ok(())
/// });
/// ```
pub fn with_modified_phink_config<F>(
    config: Configuration,
    executed_test: F,
) -> io::Result<()>
where
    F: FnOnce() -> io::Result<()>,
{
    // We ensure that the path doesn't exist yet. It doesn't matter if it `Err`
    let _ = remove_instrumented_contract_path(&config);

    // If this isn't the default `fuzz_output`, we clean it
    &config
        .fuzz_output
        .as_ref()
        .map(|output| fs::remove_dir_all(output));

    config.save_as_toml(DEFAULT_TEST_PHINK_TOML);

    // Executing the actual test
    executed_test()?;

    // We remove the temp config file
    fs::remove_file(DEFAULT_TEST_PHINK_TOML)?;
    // We clean the instrumented path
    remove_instrumented_contract_path(&config)?;

    // If this isn't the default `fuzz_output`, we clean it after the test executed
    &config
        .fuzz_output
        .as_ref()
        .map(|output| fs::remove_dir_all(output));
    Ok(())
}

fn remove_instrumented_contract_path(config: &Configuration) -> Result<(), io::Error> {
    // If it's `None`, we'll just get the default path, so we don't remove it
    match &config.instrumented_contract_path {
        None => Ok(()),
        Some(path) => {
            let buf = &path.path;
            println!("Removing {:?}", buf);
            fs::remove_dir_all(buf)
        }
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
        .arg("fuzz")
        .arg(path_instrumented_contract.path.to_str().unwrap())
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
