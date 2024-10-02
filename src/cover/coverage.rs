use crate::{
    cli::config::{
        PFiles::CoverageTracePath,
        PhinkFiles,
    },
    cover::trace::CoverageTrace,
};
use serde_derive::{
    Deserialize,
    Serialize,
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

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct InputCoverage {
    /// One input might contains multiple messages
    messages_coverage: Vec<MessageCoverage>,
    /// Simply the `Vec` of Strings, for example
    /// COV=128
    /// COV=129 ...
    raw_from_debug: Vec<CoverageTrace>,
}

/// This struct represent the coverage of one message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageCoverage {
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
    pub fn new() -> InputCoverage {
        InputCoverage {
            messages_coverage: Vec::new(),
            raw_from_debug: Vec::new(),
        }
    }
    pub fn coverage_len(&self) -> usize {
        self.messages_coverage
            .iter()
            .map(|msg| msg.cov_ids.len())
            .sum()
    }

    pub fn messages_coverage(&self) -> &Vec<MessageCoverage> {
        &self.messages_coverage
    }

    pub fn add_cov(&mut self, coverage: &CoverageTrace) {
        let parsed = coverage.parse_coverage();
        self.raw_from_debug.push(coverage.clone());
        self.messages_coverage
            .push(MessageCoverage { cov_ids: parsed });
    }

    pub fn save(&self, output: PathBuf) -> std::io::Result<()> {
        let mut unique_cov_ids = HashSet::new();

        for message in &self.messages_coverage {
            for &cov_id in &message.cov_ids {
                unique_cov_ids.insert(cov_id);
            }
        }

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
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::identity_op)]
    pub fn redirect_coverage(&self, flat: Vec<u64>) {
        /// We assume that the instrumentation will never insert more than
        /// `2_000` artificial branches This value should be big enough
        /// to handle most of smart-contract, even the biggest
        seq_macro::seq!(x in 0..= 2_000 {
            if flat.contains(&(x)) {
                // println!("{:?}", x);
                let _ = black_box(x + 1);
            }
        });
    }
}
