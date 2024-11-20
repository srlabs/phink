use crate::cli::{
    config::PFiles,
    ui::traits::FromPath,
};
use std::{
    fs,
    path::PathBuf,
    time::UNIX_EPOCH,
};

#[derive(Clone, Debug)]
pub struct CorpusWatcher {
    corpus_folder: PathBuf,
}

#[derive(Clone, Debug)]
pub struct PlotEntry {
    x: f64,
    y: f64,
}

impl PlotEntry {
    pub fn new(x: f64, y: f64) -> PlotEntry {
        PlotEntry { x, y }
    }
}

impl From<PlotEntry> for (f64, f64) {
    fn from(entry: PlotEntry) -> Self {
        (entry.x, entry.y)
    }
}

impl FromPath for CorpusWatcher {
    type Output = CorpusWatcher;

    fn create_instance(log_fullpath: PathBuf) -> Self::Output {
        CorpusWatcher {
            corpus_folder: log_fullpath,
        }
    }

    fn get_filetype() -> PFiles {
        PFiles::CorpusPath
    }
}

impl CorpusWatcher {
    pub fn as_tuple_slice(&mut self) -> Vec<(f64, f64)> {
        self.data().iter().map(|entry| (entry.x, entry.y)).collect()
    }

    pub fn data(&mut self) -> Vec<PlotEntry> {
        let mut data: Vec<PlotEntry> = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.corpus_folder) {
            let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();

            // Sort entries by their creation time
            entries.sort_by_key(|entry| entry.metadata().unwrap().created().unwrap());

            for (i, entry) in entries.into_iter().enumerate() {
                let x = entry
                    .metadata()
                    .unwrap()
                    .created()
                    .unwrap()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64();

                data.push(PlotEntry::new(x, i as f64));
            }
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EmptyResult;
    use std::{
        fs,
        io::Write,
        path::Path,
        thread::sleep,
        time::Duration,
    };
    use tempfile::{
        NamedTempFile,
        TempDir,
    };

    #[test]
    fn test_from_fullpath_existing_folder() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();

        let result = CorpusWatcher::from_fullpath(path.clone());
        assert!(result.is_ok());

        let mut watcher = result.unwrap();
        assert_eq!(watcher.corpus_folder, path);
        assert!(watcher.data().is_empty());
    }

    #[test]
    fn test_from_fullpath_non_existing_folder() {
        let non_existing_path = PathBuf::from("/non/existing/path");
        let result = CorpusWatcher::from_fullpath(non_existing_path);
        assert!(result.is_err());
    }
    #[test]
    fn test_corpus_watcher_data() -> EmptyResult {
        let corpus_path = Path::new("tests/fixtures/corpus").to_path_buf();

        let mut watcher = CorpusWatcher::from_fullpath(corpus_path.clone())?;

        // Initial check
        let initial_data = watcher.data();
        assert_eq!(initial_data.len(), 1215);

        // Add a file and check again
        let mut temp_file = NamedTempFile::new_in(corpus_path.clone())?;
        writeln!(temp_file, "just a random seed")?;

        sleep(Duration::from_secs(1)); // Sleep to ensure different timestamp
        let data_after_one_file = watcher.data();
        assert_eq!(data_after_one_file.get(3).unwrap().y, 3f64); // One file, so y should be 1

        // Add another file and check again
        let mut temp_file = NamedTempFile::new_in(corpus_path.clone())?;
        writeln!(temp_file, "just a random seed but again")?;

        sleep(Duration::from_secs(1)); // Sleep to ensure different timestamp
        let data_after_one_file = watcher.data();
        assert_eq!(data_after_one_file.len(), 1219);
        assert_eq!(data_after_one_file.get(2).unwrap().y, 2f64); // Two files, so y should be 2

        // Check that x values (timestamps) are increasing
        let second = data_after_one_file.get(40).unwrap().x; // we do 40 because if we take 2, it'll have the same timestamp
        let first = data_after_one_file.first().unwrap().x;
        // println!("second: {} & first: {}", second, first);
        assert!(second > first);
        Ok(())
    }

    #[test]
    fn test_from_output_non_existing_folder() {
        let non_existing_path = PathBuf::from("/non/existing/path");
        let result = CorpusWatcher::from_output(non_existing_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_data_empty_folder() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();

        let mut watcher = CorpusWatcher::from_fullpath(path).unwrap();
        let data = watcher.data();

        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_data_non_empty_folder() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_path_buf();

        // Create some files in the temporary directory
        fs::File::create(path.join("file1.txt")).unwrap();
        fs::File::create(path.join("file2.txt")).unwrap();

        let mut watcher = CorpusWatcher::from_fullpath(path).unwrap();
        let data = watcher.data();

        assert_eq!(data.len(), 2);
        assert_eq!(data[1].y, 1.0);
    }

    #[test]
    fn test_plot_entry_conversion() {
        let entry = PlotEntry { x: 1.0, y: 2.0 };
        let tuple: (f64, f64) = entry.into();
        assert_eq!(tuple, (1.0, 2.0));
    }
}
