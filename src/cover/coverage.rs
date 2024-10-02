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
    /// All the coverage ID grabbed and deduplicated
    all_cov_id: HashSet<u64>,
    /// Simply the `Vec` of Strings, for example
    /// COV=128
    /// COV=129 ...
    raw_from_debug: Vec<CoverageTrace>,
}

impl Debug for InputCoverage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Coverage")
            .field("cov_ids", &self.all_cov_id)
            .finish()
    }
}

impl InputCoverage {
    pub fn new() -> InputCoverage {
        InputCoverage {
            all_cov_id: Vec::new(),
            raw_from_debug: Vec::new(),
        }
    }
    pub fn coverage_len(&self) -> usize {
        self.all_cov_id.len()
    }

    pub fn messages_coverage(&self) -> Vec<u64> {
        // todo: also dedup this, we shouldn't have a messageinput that has two times the same
        // coverage, as a coverage
        self.all_cov_id.clone()
    }

    pub fn add_cov(&mut self, coverage: &CoverageTrace) {
        let parsed = coverage.parse_coverage();
        self.raw_from_debug.push(coverage.clone());
        self.all_cov_id.extend(parsed); // todo: do a dedup here
    }

    pub fn save(&self, output: PathBuf) -> std::io::Result<()> {
        let mut trace_strings: Vec<String> = self
            .all_cov_id
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
