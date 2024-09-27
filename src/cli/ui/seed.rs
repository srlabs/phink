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
        Write,
    },
    path::PathBuf,
    time::{
        Duration,
        Instant,
    },
};

const EVERY_N_SECONDS: u32 = 1;
pub const LAST_SEED_FILENAME: &str = "last_seed.phink";
pub const DELIMITER: &str = "{}\n--------";

pub struct SeedWriter {
    input: OneInput,
    coverage: InputCoverage,
    responses: Vec<FullContractResponse>,
    last_save_time: Instant,
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
            last_save_time: Instant::now(),
        }
    }
    pub fn should_save(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_save_time) >= Duration::new(EVERY_N_SECONDS.into(), 0) {
            self.last_save_time = now;
            true
        } else {
            false
        }
    }

    pub fn save(&self, output: PathBuf) {
        let to = PhinkFiles::new(output).path(PFiles::LastSeed);

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&to) {
            for (message, response) in self.input.messages.iter().zip(self.responses.iter()) {
                let msg = message.display_with_reply(response.get_response());
                writeln!(file, "{DELIMITER} {msg}").expect("Failed to write to file");
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
        let maybe_file = File::open(PhinkFiles::new(self.output.clone()).path(PFiles::LastSeed));
        if let Ok(file) = maybe_file {
            return Some(Self::parse(file))
        }
        None
    }

    fn parse(file: File) -> Vec<String> {
        let reader = io::BufReader::new(file);
        let mut result = Vec::new();
        let mut current_seed = String::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line == DELIMITER {
                result.push(current_seed.clone());
                current_seed.clear();
            } else {
                if !current_seed.is_empty() {
                    current_seed.push('\n');
                }
                current_seed.push_str(&line);
            }
        }

        // Push the last seed if the file doesn't end with the delimiter
        if !current_seed.is_empty() {
            result.push(current_seed);
        }
        result
    }
}
