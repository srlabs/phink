extern crate phink_lib;
pub mod shared;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{
        fuzz,
        instrument,
        samples::Sample,
        with_modified_phink_config,
        DEFAULT_TEST_PHINK_TOML,
    };
    use phink_lib::{
        cli::config::Configuration,
        instrumenter::instrumented_path::InstrumentedPath,
    };
    use predicates::prelude::predicate;
    use std::{
        fs,
        io::{
            stdout,
            BufRead,
            BufReader,
        },
        path::{
            Path,
            PathBuf,
        },
        process::{
            Child,
            Command,
            Stdio,
        },
        thread,
        time::{
            Duration,
            Instant,
        },
    };

    #[test]
    fn test_fuzz_started() {
        let path_instrumented_contract =
            InstrumentedPath::new(PathBuf::from("just_a_random_path"));

        let config = Configuration {
            instrumented_contract_path: Some(path_instrumented_contract.clone()),
            fuzz_output: Some(PathBuf::from("output_for_integration_test")),
            cores: Some(1),
            ..Default::default()
        };

        let exec_test = with_modified_phink_config(config.clone(), || {
            instrument(Sample::Dummy);

            let mut child = fuzz(path_instrumented_contract);

            // Track the time to ensure it's within the first 2 minutes
            let start_time = Instant::now();
            let timeout = Duration::from_secs(20); // 2 minutes

            loop {
                println!("LINE");

                // Capture the standard output of the child process
                if let Some(stdout) = child.stdout.take() {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        let line = line.expect("Failed to read line");
                        println!("LINE IS {:?}", line);
                        if line.contains("ziggy rocking") {
                            panic!("Found 'ziggy rocks' in output, stopping the loop.");
                        }
                    }
                }

                if fs::metadata(config.fuzz_output.clone().unwrap_or_default()).is_ok() {
                    child.kill().expect("Failed to kill the process");
                    assert!(true);
                    return Ok(());
                }

                // If 2 minutes have passed, fail the test
                if start_time.elapsed() > timeout {
                    child.kill().expect("Failed to kill the process");
                    assert!(false, "Folder 'abc' was not created within 2 minutes");
                    return Ok(());
                }

                // Sleep for a short period to avoid busy-waiting
                std::thread::sleep(Duration::from_secs(1));
            }
            Ok(())
        });

        assert!(exec_test.is_ok());
    }
}
