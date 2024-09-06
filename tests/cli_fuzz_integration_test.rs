#![recursion_limit = "1024"]
extern crate phink_lib;

pub mod shared;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{
        afl_log_didnt_fail,
        ensure_while_fuzzing,
        get_corpus_files,
        instrument,
        is_compiled,
        is_instrumented,
        samples::Sample,
        with_modified_phink_config,
    };
    use anyhow::{
        ensure,
        Result,
    };
    use phink_lib::{
        cli::config::Configuration,
        instrumenter::instrumented_path::InstrumentedPath,
    };
    use regex::Regex;
    use std::{
        fs,
        time::Duration,
    };
    use tempfile::tempdir;

    #[test]
    #[cfg_attr(target_os = "macos", ignore)]
    fn test_fuzz_find_crash_before_two_minutes() -> Result<()> {
        let fuzz_output = tempdir()?.into_path();
        let config = Configuration {
            instrumented_contract_path: Some(InstrumentedPath::from(tempdir()?.into_path())),
            fuzz_output: Some(fuzz_output.clone()),
            cores: Some(4),
            show_ui: false,
            ..Default::default()
        };

        with_modified_phink_config(&config, || {
            let _ = instrument(Sample::Dummy);

            let fuzzing = ensure_while_fuzzing(&config, Duration::from_secs(120), || {
                let fuzz_created = fs::metadata(fuzz_output.clone()).is_ok();
                ensure!(fuzz_created, "Fuzz output directory wasn't created");

                if fuzz_created {
                    // We search if a crashes is spotted
                    let afl_log = config
                        .fuzz_output
                        .clone()
                        .unwrap_or_default()
                        .join("phink")
                        .join("logs")
                        .join("afl.log");

                    let log_content = fs::read_to_string(afl_log).unwrap();

                    let saved_crashes: i32 = if let Some(captures) =
                        Regex::new(r"saved crashes : (\d+)")
                            .unwrap()
                            .captures(&log_content)
                    {
                        captures[1].parse().unwrap()
                    } else {
                        0
                    };

                    ensure!(
                        saved_crashes >= 1,
                        "No crash detected within the 120 seconds, this should crash easily"
                    );
                }
                Ok(())
            });

            ensure!(
                fuzzing.is_ok(),
                "ensure_while_fuzzing returned an error: {:?}",
                fuzzing.unwrap_err()
            );

            Ok(())
        })
    }

    #[test]
    fn test_fuzz_assert_output_created_when_fuzzing() -> Result<()> {
        let fuzz_output = tempdir()?.into_path();
        let config = Configuration {
            instrumented_contract_path: Some(InstrumentedPath::from(tempdir()?.into_path())),
            fuzz_output: Some(fuzz_output.clone()),
            cores: Some(1),
            show_ui: false,
            ..Default::default()
        };

        with_modified_phink_config(&config, || {
            let _ = instrument(Sample::Dummy);

            let mut initial_corpus_len = 0_usize;
            let phink_output = fuzz_output.join("phink");

            // While fuzzing, let's perform the tests
            let fuzzing = ensure_while_fuzzing(&config, Duration::from_secs(120), || {
                let fuzz_created = phink_output.exists();
                ensure!(fuzz_created, "Fuzz output directory wasn't created");

                if fuzz_created {
                    let corpus_res = get_corpus_files(&phink_output.join("corpus"));
                    initial_corpus_len = match corpus_res {
                        Ok(_) => corpus_res.iter().len(),
                        _ => 0,
                    };

                    let path_contract = &config
                        .clone()
                        .instrumented_contract_path
                        .unwrap_or_default()
                        .path;

                    ensure!(is_instrumented(path_contract), "Dummy wasn't instrumented ");

                    ensure!(is_compiled(path_contract), "Dummy wasn't compiled properly");

                    ensure!(
                        initial_corpus_len > 0,
                        "Corpus directory is empty after creation: {:?} files",
                        initial_corpus_len
                    );

                    let selector = phink_output.join("selectors.dict");
                    ensure!(selector.exists(), "selectors.dict doesn't exist");

                    // The content of selectors.dict should be
                    //
                    // # Dictionary file for selectors
                    // # Lines starting with '#' and empty lines are ignored.
                    // delimiter="********"
                    // "\x9B\xAE\x9D\x5E"
                    // "\xFA\x80\xC2\xF6"

                    ensure!(
                        fs::read_to_string(selector).unwrap().lines().count() == 5,
                        "There should be 5 lines in selectors, 2 for crash_with_invariant and phink_assert_dangerous_number, 1 for demimiter, and two comments"
                    );

                    ensure!(
                        afl_log_didnt_fail(&phink_output),
                        "'logs/afl.log' didn't return a successfull dashboard"
                    );

                    // We don't use allowlist for macos
                    if cfg!(not(target_os = "macos")) {
                        ensure!(
                            phink_output.join("allowlist.txt").exists(),
                            "allowlist.txt for AFL doesn't exist"
                        );
                    }
                }
                Ok(())
            });

            ensure!(
                fuzzing.is_ok(),
                "ensure_while_fuzzing returned an error: {:?}",
                fuzzing.unwrap_err()
            );

            // Check that new files have been added to the corpus during fuzzing
            let corpus_len = get_corpus_files(&phink_output.join("corpus"))
                .unwrap()
                .len();
            println!("After a bit of fuzzing, we get {corpus_len} corpus entries, when we had {initial_corpus_len} entries at the begining");
            ensure!(
                corpus_len > initial_corpus_len,
                "There was no new corpus while fuzzing"
            );

            Ok(())
        })
    }

    #[test]
    fn test_fuzz_two_cores_work() -> Result<()> {
        let fuzz_output = tempdir()?.into_path();
        let config = Configuration {
            instrumented_contract_path: Some(InstrumentedPath::from(tempdir()?.into_path())),
            fuzz_output: Some(fuzz_output.clone()),
            cores: Some(2),
            show_ui: false,
            ..Default::default()
        };

        with_modified_phink_config(&config, || {
            let _ = instrument(Sample::Dummy);

            let fuzzing = ensure_while_fuzzing(&config, Duration::from_secs(120), || {
                let fuzz_created = fs::metadata(fuzz_output.clone()).is_ok();
                ensure!(fuzz_created, "Fuzz output directory wasn't created");

                if fuzz_created {
                    let log_path = config
                        .fuzz_output
                        .clone()
                        .unwrap_or_default()
                        .join("phink")
                        .join("logs");

                    ensure!(log_path.join("afl.log").exists(), "afl.log wasn't created",);
                    ensure!(
                        log_path.join("afl_1.log").exists(),
                        "afl_1.log wasn't created, even though we have 2 cores ! ",
                    );
                }
                Ok(())
            });

            ensure!(
                fuzzing.is_ok(),
                "ensure_while_fuzzing returned an error: {:?}",
                fuzzing.unwrap_err()
            );

            Ok(())
        })
    }
}
