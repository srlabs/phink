use rand::{
    distributions::Alphanumeric,
    Rng,
};
use serde::Deserialize;
use serde_derive::Serialize;
use std::{
    fs,
    io,
    io::Write,
    path::{
        Path,
        PathBuf,
    },
};

pub const DEFAULT_PATH_PATTERN_INSTRUMENTEDPATH: &str = "ink_fuzzed_";

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct InstrumentedPath {
    pub path: PathBuf,
}

impl Default for InstrumentedPath {
    /// By default, we create a random folder in /tmp/ink_fuzzed_XXXX
    fn default() -> Self {
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();

        let new_dir = Path::new("/tmp")
            .join(DEFAULT_PATH_PATTERN_INSTRUMENTEDPATH.to_string() + &random_string);

        Self { path: new_dir }
    }
}

impl InstrumentedPath {
    pub fn new(path: PathBuf) -> Self {
        InstrumentedPath { path }
    }

    pub fn clean() -> anyhow::Result<()> {
        let dirs_to_remove =
            Self::get_dirs_to_remove(Path::new("/tmp"), DEFAULT_PATH_PATTERN_INSTRUMENTEDPATH)?;

        if dirs_to_remove.is_empty() {
            println!("❌  No directories found matching the pattern '{DEFAULT_PATH_PATTERN_INSTRUMENTEDPATH}'. There's nothing to be cleaned :)" );
            return Ok(())
        }

        println!("🔍 Found the following instrumented ink! contracts:");
        for dir in &dirs_to_remove {
            println!("{}", dir.display());
        }

        if Self::prompt_user_confirmation()? {
            Self::remove_directories(dirs_to_remove)?;
        } else {
            println!("❌ Operation cancelled.");
        }

        Ok(())
    }

    fn get_dirs_to_remove(tmp_dir: &Path, pattern: &str) -> Result<Vec<PathBuf>, io::Error> {
        Ok(fs::read_dir(tmp_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_dir() && path.file_name()?.to_string_lossy().starts_with(pattern) {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>())
    }

    fn prompt_user_confirmation() -> Result<bool, io::Error> {
        print!("🗑️ Do you really want to remove these directories? (NO/yes): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().eq_ignore_ascii_case("yes"))
    }

    fn remove_directories(dirs_to_remove: Vec<PathBuf>) -> Result<(), io::Error> {
        for dir in dirs_to_remove {
            fs::remove_dir_all(&dir)?;
            println!("✅ Removed directory: {}", dir.display());
        }
        Ok(())
    }
}
