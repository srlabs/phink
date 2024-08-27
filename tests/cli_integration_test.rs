pub mod shared;

extern crate phink_lib;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{
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
        fs,
        path::PathBuf,
    };
    use walkdir::WalkDir;

    #[test]
    fn test_instrument_dummy_command() {
        let contract_path = "sample/dummy";
        let path_instrumentated_contract =
            InstrumentedPath::new(PathBuf::from("just_a_random_path"));

        let config = Configuration {
            instrumented_contract_path: Some(path_instrumentated_contract.clone()),
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
                fs::exists(path_instrumentated_contract.path.clone()).is_ok(),
                "Instrumented contract not ofund"
            );

            // Verify that a Cargo.toml exists somewhere under
            // path_instrumentated_contract
            let cargo_toml_exists = WalkDir::new(path_instrumentated_contract.path)
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
}
