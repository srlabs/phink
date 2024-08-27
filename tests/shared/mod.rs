use std::{
    ffi::OsStr,
    fs,
    io,
    io::Read,
    path::Path,
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
pub fn find_string_in_rs_files(dir: &Path, target: &str) -> bool {
    fn is_rs_file(entry: &Path) -> bool {
        entry.extension() == Some(OsStr::new("rs"))
    }

    fn file_contains_string(file_path: &Path, target: &str) -> bool {
        let mut file = fs::File::open(file_path).expect("Unable to open file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Unable to read file");
        content.contains(target)
    }

    let entries = fs::read_dir(dir).expect("Unable to read directory");

    for entry in entries {
        let entry = entry.expect("Unable to get directory entry");
        let path = entry.path();

        if path.is_dir() {
            if find_string_in_rs_files(&path, target) {
                return true;
            }
        } else if is_rs_file(&path) {
            if file_contains_string(&path, target) {
                return true;
            }
        }
    }

    false
}
