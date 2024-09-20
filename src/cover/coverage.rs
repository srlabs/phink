use crate::{
    cli::config::{
        PFiles::CoverageTracePath,
        PhinkFiles,
    },
    cover::trace::CoverageTrace,
};
use std::{
    collections::HashSet,
    fmt,
    fmt::{
        Debug,
        Formatter,
    },
    fs::OpenOptions,
    hint::black_box,
    io::Write,
    path::PathBuf,
};

#[derive(Clone, Default)]
pub struct InputCoverage {
    /// One input might contains multiple messages
    messages_coverage: Vec<MessageCoverage>,
    /// Simply the `Vec` of Strings, for example
    /// COV=128
    /// COV=129 ...
    raw_from_debug: Vec<CoverageTrace>,
}

/// This struct represent the coverage of one message.
#[derive(Clone, Debug)]
pub struct MessageCoverage {
    /// A map where the key is the ID of the parsed value of COV=..., and the value is
    /// the number of times this coverage point was hit.
    pub cov_ids: Vec<u64>,
}

impl Debug for InputCoverage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Coverage")
            .field("messages_coverage", &self.messages_coverage)
            .finish()
    }
}

impl InputCoverage {
    pub fn add_cov(&mut self, coverage: &CoverageTrace) {
        let parsed = coverage.parse_coverage();
        self.raw_from_debug.push(coverage.clone());
        self.messages_coverage
            .push(MessageCoverage { cov_ids: parsed });
    }

    pub fn save(&self, output: PathBuf) -> std::io::Result<()> {
        // Create a HashSet to store unique coverage IDs
        let mut unique_cov_ids = HashSet::new();

        // Collect all coverage IDs from MessageCoverage into the HashSet
        for message in &self.messages_coverage {
            for &cov_id in &message.cov_ids {
                unique_cov_ids.insert(cov_id);
            }
        }

        // Convert HashSet to a sorted Vec of Strings
        let mut trace_strings: Vec<String> = unique_cov_ids
            .into_iter()
            .map(|id| id.to_string())
            .collect();
        trace_strings.sort_unstable();

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(PhinkFiles::new(output).path(CoverageTracePath))?;

        // Write each unique ID to the file, one per line
        writeln!(file, "{}", trace_strings.join("\n"))?;

        Ok(())
    }

    pub fn flatten_cov(&self) -> Vec<u64> {
        // todo: also dedup this, we shouldn't have a messageinput that has two times the same
        // coverage, as a coverage
        self.messages_coverage
            .iter()
            .flat_map(|entry| entry.cov_ids.clone().into_iter())
            .collect()
    }

    #[allow(unused_doc_comments)]
    #[allow(clippy::identity_op)]
    pub fn redirect_coverage(&self, flat: Vec<u64>) {
        #[cfg(not(fuzzing))]
        {
            println!(
                "[🚧DEBUG TRACE] Detected {} messages traces",
                self.messages_coverage.clone().len(),
            );
            println!("[🚧DEBUG TRACE] Caught coverage identifiers {:?}\n", &flat);
        }

        #[allow(clippy::unnecessary_cast)]
        /// We assume that the instrumentation will never insert more than
        /// `2_000` artificial branches This value should be big enough
        /// to handle most of smart-contract, even the biggest
        seq_macro::seq!(x in 0..= 2_000 {
            if flat.contains(&(x)) {
                println!("{:?}", x);
                let _ = black_box(x + 1);
            }
        });
    }

    pub fn deduplicate(input: &str) -> String {
        let mut unique_lines = HashSet::new();
        input
            .lines()
            .filter(|&line| unique_lines.insert(line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
