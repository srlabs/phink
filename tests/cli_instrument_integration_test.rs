extern crate phink_lib;
pub mod shared;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{
        find_string_in_rs_files,
        instrument,
        samples::Sample,
        with_modified_phink_config,
        DEFAULT_TEST_PHINK_TOML,
    };
    use anyhow::ensure;
    use assert_cmd::Command as CommandAssertCmd;
    use phink_lib::{
        cli::config::Configuration,
        instrumenter::instrumented_path::InstrumentedPath,
    };
    use predicates::prelude::predicate;
    use std::{
        fs,
        path::PathBuf,
    };
    use walkdir::WalkDir;

    #[test]
    fn test_instrument_respects_configuration() {
        let path_instrumented_contract = InstrumentedPath::new(PathBuf::from(
            "test_instrumentation_multifile_contract_INSTRUMENTED_PATH",
        ));

        let config = Configuration {
            instrumented_contract_path: Some(path_instrumented_contract.clone()),
            fuzz_output: Some(PathBuf::from("test_instrumentation_multifile_contract")),
            ..Default::default()
        };

        let test = with_modified_phink_config(&config, || {
            instrument(Sample::MultiContractCaller);

            ensure!(
                fs::exists(path_instrumented_contract.path.clone()).is_ok(),
                "Instrumented contract not found"
            );

            // Verify that a Cargo.toml exists somewhere under
            // `path_instrumented_contract`
            let cargo_toml_exists = WalkDir::new(path_instrumented_contract.path)
                .into_iter()
                .filter_map(|e| e.ok()) // Filter out errors
                .any(|entry| {
                    entry.file_name() == "Cargo.toml"
                });

            ensure!(
                cargo_toml_exists,
                "Cargo.toml not found in the instrumented contract path"
            );

            Ok(())
        });
        assert!(test.is_ok(), "{}", test.err().unwrap().to_string());
    }

    #[test]
    fn test_instrumentation_multifile_contract() {
        let path_instrumented_contract = InstrumentedPath::new(PathBuf::from(
            "path_instrumented_contract_test_instrument_respects_configuration",
        ));

        let config = Configuration {
            instrumented_contract_path: Some(path_instrumented_contract.clone()),
            fuzz_output: Some(PathBuf::from(
                "fuzz_output_test_instrument_respects_configuration",
            )),
            ..Default::default()
        };

        let test = with_modified_phink_config(&config, || {
            instrument(Sample::MultiContractCaller);

            ensure!(
                fs::exists(path_instrumented_contract.path.clone()).is_ok(),
                "Instrumented contract not found"
            );

            let accumulator_contains_debug = find_string_in_rs_files(
                &path_instrumented_contract.path.join("accumulator"),
                "ink::env::debug_println!(\"COV={}\",",
            );
            ensure!(
                accumulator_contains_debug,
                "Expected to find a trace of instrumentation in Accumulator"
            );

            let subber_contains_debug = find_string_in_rs_files(
                &path_instrumented_contract.path.join("subber"),
                "ink::env::debug_println!(\"COV={}\",",
            );
            ensure!(
                subber_contains_debug,
                "Expected to find a trace of instrumentation in Subber"
            );

            let adder_contains_debug = find_string_in_rs_files(
                &path_instrumented_contract.path.join("adder"),
                "ink::env::debug_println!(\"COV={}\",",
            );
            ensure!(
                adder_contains_debug,
                "Expected to find a trace of instrumentation in Adder"
            );

            Ok(())
        });
        assert!(test.is_ok(), "{}", test.err().unwrap().to_string());
    }

    #[test]
    fn test_instrument_contains_instrumented_code() {
        let path_instrumented_contract = InstrumentedPath::new(PathBuf::from(
            "path_instrumented_contract_test_instrument_contains_instrumented_code",
        ));

        let config = Configuration {
            instrumented_contract_path: Some(path_instrumented_contract.clone()),
            fuzz_output: Some(PathBuf::from(
                "fuzz_output_test_instrument_contains_instrumented_code",
            )),
            ..Default::default()
        };

        let test = with_modified_phink_config(&config, || {
            instrument(Sample::Dummy);

            let contains_instrumented_code = find_string_in_rs_files(
                &path_instrumented_contract.path,
                "ink::env::debug_println!(\"COV={}\",",
            );
            ensure!(
                contains_instrumented_code,
                "Expected to find a trace of instrumentation in at least one .rs file"
            );
            Ok(())
        });

        assert!(test.is_ok(), "{}", test.err().unwrap().to_string());
    }

    #[test]
    fn test_instrument_help_terminates_correctly() {
        CommandAssertCmd::cargo_bin("phink")
            .unwrap()
            .args(["--config", DEFAULT_TEST_PHINK_TOML])
            .arg("instrument")
            .arg(Sample::Dummy.path())
            .arg("--help")
            .assert()
            .stdout(predicate::str::contains(
                "Usage: phink instrument <CONTRACT_PATH>",
            ));
    }
}
