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
    };
    use anyhow::ensure;
    use phink_lib::{
        cli::config::Configuration,
        instrumenter::instrumented_path::InstrumentedPath,
    };
    use std::{
        fs,
        path::PathBuf,
    };
    use walkdir::WalkDir;

    // TODO: Test multi instrumentatino

    #[test]
    fn test_instrument_respects_configuration() {
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
            instrument(Sample::CrossMessageBug);

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
}
