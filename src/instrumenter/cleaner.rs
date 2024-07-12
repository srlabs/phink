use crate::Instrumenter;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};
pub trait Cleaner {
    fn clean() -> Result<(), io::Error>;
}

impl Cleaner for Instrumenter {
    fn clean() -> Result<(), io::Error> {
        let pattern = "ink_fuzzed_";
        let dirs_to_remove = Self::get_dirs_to_remove(Path::new("/tmp"), pattern)?;

        if dirs_to_remove.is_empty() {
            println!("❌  No directories found matching the pattern '{}'. There's nothing to be cleaned :)", pattern);
            return Ok(());
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
}

impl Instrumenter {
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
