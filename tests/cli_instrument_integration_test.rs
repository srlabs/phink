extern crate phink_lib;
pub mod shared;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{
        instrument,
        is_compiled,
        is_instrumented,
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
            let _ = instrument(Sample::MultiContractCaller);

            ensure!(
                fs::exists(path_instrumented_contract.path.clone()).is_ok(),
                "Instrumented contract not found"
            );

            // Verify that a Cargo.toml exists somewhere under
            // `path_instrumented_contract`
            let cargo_toml_exists = WalkDir::new(&path_instrumented_contract.path)
                .into_iter()
                .filter_map(Result::ok) // Filter out errors
                .any(|entry| {
                    entry.file_name() == "Cargo.toml"
                });

            ensure!(
                cargo_toml_exists,
                "Cargo.toml not found in the instrumented contract path"
            );

            ensure!(
                is_compiled(&path_instrumented_contract.path),
                "Target wasn't compiled properly"
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
            let _ = instrument(Sample::MultiContractCaller);

            ensure!(
                fs::exists(path_instrumented_contract.path.clone()).is_ok(),
                "Instrumented contract not found"
            );

            let accu = &path_instrumented_contract.path.join("accumulator");
            ensure!(
                is_instrumented(accu),
                "Expected to find a trace of instrumentation in Accumulator"
            );

            let subber = &path_instrumented_contract.path.join("subber");
            ensure!(
                is_instrumented(subber),
                "Expected to find a trace of instrumentation in Subber"
            );

            let adder = &path_instrumented_contract.path.join("adder");

            ensure!(
                is_instrumented(adder),
                "Expected to find a trace of instrumentation in Adder"
            );

            ensure!(
                is_compiled(&path_instrumented_contract.path),
                "Target wasn't compiled properly"
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
            let _ = instrument(Sample::Dummy);

            ensure!(
                is_instrumented(&path_instrumented_contract.path),
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
