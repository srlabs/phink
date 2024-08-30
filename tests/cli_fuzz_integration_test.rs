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
        try_cleanup_fuzzoutput,
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
    fn test_fuzz_assert_output_created_when_fuzzing() {
        let fuzz_output = PathBuf::from("output_test_assert_output_created_when_fuzzing");
        let config = Configuration {
            instrumented_contract_path: Some(InstrumentedPath::new(PathBuf::from(
                "test_assert_output_created_when_fuzzing_instrumented",
            ))),
            fuzz_output: Some(fuzz_output.clone()),
            cores: Some(1),
            ..Default::default()
        };

        let corpus_path = &fuzz_output.join("phink").join("corpus"); // That's the default Ziggy corpus path

        with_modified_phink_config(&config, || {
            instrument(Sample::Dummy);
            let mut initial_corpus_files = 0_usize;

            // While fuzzing, let's perform the tests
            ensure!(
                ensure_while_fuzzing(&config, Duration::from_secs(120), || {
                    let fuzz_created = fs::metadata(fuzz_output.clone()).is_ok();
                    println!("Fuzz output created yet : {:?}", fuzz_created);
                    ensure!(fuzz_created, "Fuzz output directory wasn't created");

                    // // ensure!(false); // todo why the fuck is that passing
                    if fuzz_created {
                        initial_corpus_files = get_corpus_files(corpus_path).len();
                        println!("initial_corpus_files: {:?}", initial_corpus_files);

                        ensure!(
                            !initial_corpus_files != 0,
                            "After created, the corpus files wasn't empty{:?}",
                            initial_corpus_files
                        );

                        ensure!(
                            fs::metadata(&fuzz_output.join("phink").join("selectors.dict")).is_ok(),
                            "Selectors.dict doesn't exist"
                        );

                        ensure!(
                            fs::metadata(&fuzz_output.join("phink").join("allowlist.txt")).is_ok(),
                            "ALLOWLIST for AFL doesn't exist"
                        );
                    }

                    Ok(())
                })
                .is_ok(),
                "ensure_while_fuzzing failed to pass"
            );

            // Check that new files have been added to the corpus during fuzzing
            ensure!(
                get_corpus_files(&corpus_path).len() > initial_corpus_files,
                "There was no new corpus while fuzzing"
            );

            try_cleanup_fuzzoutput(&config);

            Ok(())
        })
        .unwrap();
    }
}
