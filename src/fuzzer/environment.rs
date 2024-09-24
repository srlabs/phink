use crate::{
    cli::config::{
        PFiles::{
            CorpusPath,
            DictPath,
        },
        PhinkFiles,
    },
    contract::selectors::{
        database::SelectorDatabase,
        selector::Selector,
    },
};
use anyhow::Context;
use std::{
    fs,
    fs::OpenOptions,
    io,
    io::Write,
    path::PathBuf,
};

pub struct Dict {
    file_path: PathBuf,
}

impl Dict {
    pub fn write_dict_entry(&self, selector: &Selector) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.file_path)
            .with_context(|| format!("Failed to open file for appending: {:?}", self.file_path))?;

        writeln!(file, "\"{}\"", selector)
            .with_context(|| format!("Couldn't write selector '{}' into the dict", selector))?;

        Ok(())
    }

    pub fn new(phink_file: PhinkFiles) -> io::Result<Dict> {
        let path_buf = phink_file.path(DictPath);
        // Create the directory structure if it doesn't exist
        if let Some(parent) = path_buf.parent() {
            fs::create_dir_all(parent)?;
        }
        // Open the file for writing (create if it doesn't exist)
        let mut dict_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path_buf.clone())?;

        writeln!(dict_file, "# Dictionary file for selectors")?;
        writeln!(
            dict_file,
            "# Lines starting with '#' and empty lines are ignored."
        )?;
        writeln!(dict_file, "delimiter=\"********\"")?;

        Ok(Self {
            file_path: path_buf,
        })
    }
}

pub struct CorpusManager {
    corpus_dir: PathBuf,
}

impl CorpusManager {
    pub fn new(phink_file: PhinkFiles) -> anyhow::Result<CorpusManager> {
        let corpus_dir = phink_file.path(CorpusPath);
        fs::create_dir_all(&corpus_dir)?;
        Ok(Self { corpus_dir })
    }

    pub fn write_corpus_file(&self, index: usize, selector: &Selector) -> io::Result<()> {
        // 00010000 01 fa80c2f6 00
        let mut data = vec![0x00, 0x00, 0x00, 0x00, 0x01];
        let file_path = self.corpus_dir.join(format!("selector_{index}.bin"));
        data.extend_from_slice(selector.0.as_ref());
        data.extend(vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]);
        fs::write(file_path, data)
    }
}

pub struct EnvironmentBuilder {
    database: SelectorDatabase,
}

impl EnvironmentBuilder {
    pub fn new(database: SelectorDatabase) -> EnvironmentBuilder {
        Self { database }
    }

    /// This function builds both the correct seeds and the dict file for AFL++
    pub fn build_env(self, fuzz_output: PathBuf) -> anyhow::Result<()> {
        let phink_file = PhinkFiles::new(fuzz_output);

        let dict = Dict::new(phink_file.clone())?;
        let corpus_manager = CorpusManager::new(phink_file)
            .with_context(|| "Couldn't create a new corpus manager")?;

        for (i, selector) in self
            .database
            .messages()
            .with_context(|| "Couldn't load messages")?
            .iter()
            .enumerate()
        {
            corpus_manager
                .write_corpus_file(i, selector)
                .with_context(|| "Couldn't write corpus file")?;
            dict.write_dict_entry(selector)
                .with_context(|| "Couldn't write the dictionnary entries")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        io,
    };
    use tempfile::tempdir;

    fn create_test_selector() -> Selector {
        Selector([0x01, 0x02, 0x03, 0x04])
    }

    fn create_temp_phink_file(file_name: &str) -> PathBuf {
        let dir = tempdir().unwrap();
        dir.path().join(file_name)
    }

    #[test]
    fn test_dict_new_creates_file() -> io::Result<()> {
        let path = create_temp_phink_file("test_dict");
        let phink_file = PhinkFiles::new(path.clone());

        let dict = Dict::new(phink_file)?;
        assert!(dict.file_path.exists());

        let contents = fs::read_to_string(dict.file_path)?;
        assert!(contents.contains("# Dictionary file for selectors"));

        Ok(())
    }

    #[test]
    fn test_write_dict_entry() -> anyhow::Result<()> {
        let path = create_temp_phink_file("test_dict");
        let phink_file = PhinkFiles::new(path.clone());
        let dict = Dict::new(phink_file)?;
        let selector = create_test_selector(); //        Selector([0x01, 0x02, 0x03, 0x04])
        dict.write_dict_entry(&selector)?;
        let contents = fs::read_to_string(dict.file_path)?;
        assert!(contents.contains("01020304"));

        Ok(())
    }

    #[test]
    fn test_corpus_manager_new_creates_dir() -> anyhow::Result<()> {
        let path = create_temp_phink_file("test_corpus");
        let phink_file = PhinkFiles::new(path.clone());

        let corpus_manager = CorpusManager::new(phink_file)?;
        assert!(corpus_manager.corpus_dir.exists());

        Ok(())
    }

    #[test]
    fn test_write_corpus_file() -> io::Result<()> {
        let path = create_temp_phink_file("test_corpus");
        let phink_file = PhinkFiles::new(path.clone());
        let corpus_manager = CorpusManager::new(phink_file).unwrap();

        let selector = create_test_selector();
        corpus_manager.write_corpus_file(0, &selector)?;

        let file_path = corpus_manager.corpus_dir.join("selector_0.bin");
        assert!(file_path.exists());

        let data = fs::read(file_path)?;
        assert_eq!(data[5..9], selector.0);

        Ok(())
    }
}
