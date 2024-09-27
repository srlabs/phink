use crate::{
    cli::config::{
        PFiles,
        PhinkFiles,
    },
    contract::remote::FullContractResponse,
    cover::coverage::InputCoverage,
    fuzzer::parser::OneInput,
};
use std::{
    fs::{
        File,
        OpenOptions,
    },
    io,
    io::{
        BufRead,
        Read,
        Write,
    },
    path::PathBuf,
    time::{
        Duration,
        Instant,
        SystemTime,
        UNIX_EPOCH,
    },
};

pub const LAST_SEED_FILENAME: &str = "last_seed.phink";
pub const DELIMITER: &str = "-X------X-";

pub struct SeedWriter {
    input: OneInput,
    coverage: InputCoverage,
    responses: Vec<FullContractResponse>,
}

impl SeedWriter {
    pub fn new(
        input: OneInput,
        coverage: InputCoverage,
        responses: Vec<FullContractResponse>,
    ) -> Self {
        SeedWriter {
            input,
            coverage,
            responses,
        }
    }

    pub fn should_save() -> bool {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            % 2
            == 0
    }

    pub fn save(&self, output: PathBuf) {
        let to = PhinkFiles::new(output).path(PFiles::LastSeed);
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&to)
        {
            for message in self.input.messages.iter() {
                let msg = message.to_string();
                writeln!(file, "{DELIMITER} {msg}").expect("Failed to save the fuzzed seed");
            }
        } else {
            eprintln!("Failed to open the file for writing");
        }
    }
}

pub struct SeedDisplayer {
    output: PathBuf,
}

impl SeedDisplayer {
    pub fn new(output: PathBuf) -> Self {
        Self { output }
    }
    pub fn load(&self) -> Option<Vec<String>> {
        let buf = PhinkFiles::new(self.output.clone()).path(PFiles::LastSeed);
        let maybe_file = File::open(buf.clone());
        if let Ok(file) = maybe_file {
            return Some(Self::parse(file))
        }
        None
    }

    fn parse(file: File) -> Vec<String> {
        let mut reader = io::BufReader::new(file);

        let mut contents = String::new();

        reader
            .read_to_string(&mut contents)
            .expect("Failed to read file");

        let sections: Vec<String> = contents
            .split(DELIMITER)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        sections
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_displayer_load() {
        let seed_displayer = SeedDisplayer::new(PathBuf::from("tests/fixtures"));
        let seeds = seed_displayer.load().unwrap();
        assert_eq!(seeds.len(), 2);
        assert_eq!(seeds[0], "crash_with_invariant { data: ' }");
        assert_eq!(seeds[1], "crash_with_invariant { data:  }");
    }
}
