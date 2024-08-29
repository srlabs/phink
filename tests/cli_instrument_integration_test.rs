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
    use phink_lib::{
        cli::config::Configuration,
        instrumenter::instrumented_path::InstrumentedPath,
    };
    use std::{
        fs,
        path::PathBuf,
    };
    use walkdir::WalkDir;

    #[test]
    fn test_instrument_respects_configuration() {
        let path_instrumented_contract =
            InstrumentedPath::new(PathBuf::from("just_a_random_path"));

        let config = Configuration {
            instrumented_contract_path: Some(path_instrumented_contract.clone()),
            ..Default::default()
        };

        let test = with_modified_phink_config(config, || {
            instrument(Sample::Dummy);

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
        assert!(test.is_ok());
    }

    #[test]
    fn test_instrument_contains_instrumented_code() {
        let path_instrumented_contract =
            InstrumentedPath::new(PathBuf::from("just_a_random_path"));

        let config = Configuration {
            instrumented_contract_path: Some(path_instrumented_contract.clone()),
            ..Default::default()
        };

        let exec_test = with_modified_phink_config(config, || {
            instrument(Sample::Dummy);

            let contains_instrumented_code = find_string_in_rs_files(
                &path_instrumented_contract.path,
                "ink::env::debug_println!(\"COV={}\",",
            );
            assert!(
                contains_instrumented_code,
                "Expected to find a trace of instrumentation in at least one .rs file"
            );
            Ok(())
        });
        assert!(exec_test.is_ok());
    }
}
