#![recursion_limit = "1024"]
extern crate phink_lib;

pub mod shared;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::{
        ensure_while_fuzzing,
        get_corpus_files,
        instrument,
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
    use std::{
        fs,
        path::PathBuf,
        time::Duration,
    };

    #[test]
    fn test_assert_output_created_when_fuzzing() {
        let fuzz_output = PathBuf::from("output_for_integration_test");
        let config = Configuration {
            instrumented_contract_path: Some(InstrumentedPath::new(PathBuf::from(
                "just_a_random_path",
            ))),
            fuzz_output: Some(fuzz_output.clone()),
            cores: Some(1),
            ..Default::default()
        };

        let corpus_path = &fuzz_output.join("phink").join("corpus");

        with_modified_phink_config(&config, || {
            instrument(Sample::Dummy);
            // Defaut Ziggy corpus location
            let mut initial_corpus_files = 0_usize;

            // While fuzzing
            let test_passed =
                ensure_while_fuzzing(&config, Duration::from_secs(30), || {
                    // We check that the output is properly created
                    let fuzz_created = fs::metadata(fuzz_output.clone()).is_ok();
                    ensure!(false); // todo why the fuck is that passing

                    if fuzz_created {
                        initial_corpus_files = get_corpus_files(corpus_path).len();
                    }
                    // We check that the corpus isn't empty
                    ensure!(!initial_corpus_files != 0);

                    Ok(())
                });

            ensure!(test_passed.is_ok());
            // Check that new files have been added to the corpus during fuzzing
            ensure!(
                get_corpus_files(&fuzz_output).len() > initial_corpus_files,
                "There was no new corpus while fuzzing"
            );
            Ok(())
        })
        .unwrap();
    }
}
