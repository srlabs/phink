extern crate phink_lib;
pub mod shared;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{
        instrument,
        samples::Sample,
        with_modified_phink_config,
        DEFAULT_TEST_PHINK_TOML,
    };
    use assert_cmd::Command;
    use phink_lib::{
        cli::config::Configuration,
        instrumenter::instrumented_path::InstrumentedPath,
    };
    use std::{
        fs,
        io::{
            BufRead,
            BufReader,
        },
        path::{
            Path,
            PathBuf,
        },
        process::Stdio,
        thread,
        time::{
            Duration,
            Instant,
        },
    };
    use std::io::stdout;

    #[test]
    fn test_fuzz_started() {
        let path_instrumented_contract =
            InstrumentedPath::new(PathBuf::from("just_a_random_path"));

        let config = Configuration {
            instrumented_contract_path: Some(path_instrumented_contract.clone()),
            ..Default::default()
        };

        with_modified_phink_config(config.clone(), || {
            instrument(Sample::Dummy);

            let mut cmd = Command::cargo_bin("phink").unwrap();
            cmd.args(["--config", DEFAULT_TEST_PHINK_TOML])
                .arg("fuzz")
                .arg(path_instrumented_contract.path.to_str().unwrap())
                .output().unwrap();

            let reader = BufReader::new(stdout);

            let start_time = Instant::now();
            let timeout = Duration::from_secs(300); // 5 minutes timeout

            let corpus_path = Path::new("abc"); // Adjust this path as needed

            for line in reader.lines() {
                if let Ok(line) = line {
                    println!("Output: {}", line);
                    if line.contains("ziggy rocking") {
                        // Check if corpus is non-empty
                        if corpus_path.exists()
                            && corpus_path
                                .read_dir()
                                .map(|mut d| d.next().is_some())
                                .unwrap_or(false)
                        {
                            println!("Corpus is non-empty!");
                            child.kill().expect("Command won't die");
                            return Ok(());
                        } else {
                            println!("Corpus is empty or doesn't exist");
                        }
                    }
                }

                if start_time.elapsed() > timeout {
                    println!("Test timed out after {:?}", timeout);
                    child.kill().expect("Command won't die");
                    return Err(*Box::new(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "Test timed out",
                    )));
                }

                thread::sleep(Duration::from_millis(100));
            }

            Err(*Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Test completed without finding 'ziggy rocking'",
            )))
        });
    }
}
