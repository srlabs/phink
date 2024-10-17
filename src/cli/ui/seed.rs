use crate::{
    cli::config::{
        PFiles,
        PhinkFiles,
    },
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
        BufReader,
        BufWriter,
        Read,
        Write,
    },
    path::PathBuf,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

pub const LAST_SEED_FILENAME: &str = "last_seed.phink";

pub struct SeedWriter {
    input: OneInput,
    coverage: InputCoverage,
}

impl SeedWriter {
    pub fn new(input: OneInput, coverage: InputCoverage) -> Self {
        SeedWriter { input, coverage }
    }

    pub fn should_save() -> bool {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            % 2
            == 0
    }

    pub fn save(&self, output: PathBuf) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(PhinkFiles::new(output).path(PFiles::LastSeed))?;
        let mut writer = BufWriter::new(file);

        let input = &self.input;
        writeln!(
            writer,
            "Got {} coverage size with {} message.s {:?}\nBytes: 0x{}\n",
            self.coverage.coverage_len(),
            input.messages.len(),
            self.coverage.messages_coverage(),
            hex::encode(&input.raw_binary)
        )?;

        for message in input.messages.iter() {
            writeln!(writer, "{}", message.print())?;
        }

        writer.flush()?;
        Ok(())
    }
}

pub struct SeedDisplayer {
    output: PathBuf,
}

impl SeedDisplayer {
    pub fn new(output: PathBuf) -> Self {
        Self { output }
    }
    pub fn load(&self) -> Option<String> {
        let buf = PhinkFiles::new(self.output.clone()).path(PFiles::LastSeed);
        let maybe_file = File::open(buf.clone());
        if let Ok(file) = maybe_file {
            return Some(Self::parse(file))
        }
        None
    }

    fn parse(file: File) -> String {
        let mut contents = String::new();

        BufReader::new(file)
            .read_to_string(&mut contents)
            .expect("Failed to read file");

        contents
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_displayer_load() {
        let seed_displayer = SeedDisplayer::new(PathBuf::from("tests/fixtures"));
        let seeds = seed_displayer.load().unwrap();
        assert!(seeds.contains("crash_with_invariant"));
    }
}
