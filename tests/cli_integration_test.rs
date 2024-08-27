pub mod shared;

extern crate phink_lib;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{
        find_string_in_rs_files,
        with_modified_phink_config,
        DEFAULT_TEST_PHINK_TOML,
    };
    use assert_cmd::Command;
    use phink_lib::{
        cli::config::Configuration,
        instrumenter::instrumented_path::InstrumentedPath,
    };
    use predicate::str;
    use predicates::prelude::*;
    use std::{
        ffi::OsStr,
        fs,
        path::PathBuf,
    };
    use walkdir::WalkDir;

    #[test]
    fn test_instrument_respects_configuration() {
        let contract_path = "sample/dummy";
        let path_instrumented_contract =
            InstrumentedPath::new(PathBuf::from("just_a_random_path"));

        let config = Configuration {
            instrumented_contract_path: Some(path_instrumented_contract.clone()),
            ..Default::default()
        };

        with_modified_phink_config(config, || {
            let mut cmd = Command::cargo_bin("phink").unwrap();
            let binding = cmd
                .args(["--config", DEFAULT_TEST_PHINK_TOML])
                .arg("instrument")
                .arg(contract_path)
                .assert()
                .success();

            assert!(
                fs::exists(path_instrumented_contract.path.clone()).is_ok(),
                "Instrumented contract not found"
            );

            // Verify that a Cargo.toml exists somewhere under
            // `path_instrumented_contract`
            let cargo_toml_exists = WalkDir::new(path_instrumented_contract.path)
                .into_iter()
                .filter_map(|e| e.ok()) // Filter out errors
                .any(|entry| entry.file_name() == "Cargo.toml");

            assert!(
                cargo_toml_exists,
                "Cargo.toml not found in the instrumented contract path"
            );

            Ok(())
        });
    }

    #[test]
    fn test_instrument_contains_instrumented_code() {
        let contract_path = "sample/dummy";
        let path_instrumented_contract =
            InstrumentedPath::new(PathBuf::from("just_a_random_path"));

        let config = Configuration {
            instrumented_contract_path: Some(path_instrumented_contract.clone()),
            ..Default::default()
        };

        with_modified_phink_config(config, || {
            let mut cmd = Command::cargo_bin("phink").unwrap();
            let binding = cmd
                .args(["--config", DEFAULT_TEST_PHINK_TOML])
                .arg("instrument")
                .arg(contract_path)
                .assert()
                .success();

            let contains_instrumented_code = find_string_in_rs_files(
                &path_instrumented_contract.path,
                "ink::env::debug_println!(\"COV={}\",",
            );
            assert!(
                contains_instrumented_code,
                "Expected to find 'ABCDEF' in at least one .rs file"
            );
            Ok(())
        });
    }
}
