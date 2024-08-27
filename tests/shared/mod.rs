use std::{
    fs,
    io,
};

pub const DEFAULT_TEST_PHINK_TOML: &str = "phink_temp_test.toml";
use phink_lib::cli::config::Configuration;

pub fn with_modified_phink_config<F>(config: Configuration, executed_test: F)
where
    F: FnOnce() -> std::io::Result<()>,
{
    // We ensure that the path doesn't exist yet. It doesn't matter if it `Err`
    let _ = remove_instrumented_contract_path(&config);

    config.save_as_toml(DEFAULT_TEST_PHINK_TOML);

    // Executing the actual test
    executed_test().unwrap();

    // We remove the temp config file
    fs::remove_file(DEFAULT_TEST_PHINK_TOML).unwrap();

    // We clean the instrumented path
    remove_instrumented_contract_path(&config).unwrap();
}

fn remove_instrumented_contract_path(config: &Configuration) -> Result<(), io::Error> {
    // If it's `None`, we'll just get the default path, so we don't remove it
    match &config.instrumented_contract_path {
        None => Ok(()),
        Some(path) => {
            let buf = &path.path;
            println!("Removing {:?}", buf);
            fs::remove_dir_all(buf)
        }
    }
}
