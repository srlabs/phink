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
    path::PathBuf,
    time::{
        Duration,
        Instant,
    },
};

const EVERY_N_SECONDS: u32 = 1;
pub const LAST_SEED_FILENAME: &str = "last_seed.phink";

pub struct SeedDisplayer {
    input: OneInput,
    coverage: InputCoverage,
    responses: Vec<FullContractResponse>,
    last_save_time: Instant,
}

impl SeedDisplayer {
    pub fn new(
        input: OneInput,
        coverage: InputCoverage,
        responses: Vec<FullContractResponse>,
    ) -> Self {
        SeedDisplayer {
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
    }
}
