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
    all_cov_id: Vec<u64>,
}

impl Debug for InputCoverage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Coverage")
            .field("", &self.all_cov_id)
            .finish()
    }
}

impl InputCoverage {
    pub fn new() -> InputCoverage {
        InputCoverage {
            all_cov_id: Vec::new(),
        }
    }
    pub fn coverage_len(&self) -> usize {
        self.all_cov_id.len()
    }

    pub fn messages_coverage(&self) -> &Vec<u64> {
        &self.all_cov_id
    }

    pub fn add_cov(&mut self, coverage: &CoverageTrace) {
        let parsed = coverage.parse_coverage();
        for id in parsed {
            if !self.all_cov_id.contains(&id) {
                self.all_cov_id.push(id);
            }
        }
    }

    pub fn save(&self, output: PathBuf) -> std::io::Result<()> {
        let mut trace_strings: Vec<String> = self
            .messages_coverage()
            .iter()
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
    #[allow(clippy::identity_op)]
    pub fn redirect_coverage(&self, flat: &[u64]) {
        /// We assume that the instrumentation will never insert more than
        /// `2_000` artificial branches This value should be big enough
        /// to handle most of smart-contract, even the biggest

        seq_macro::seq!(cov_id in 0_u64 .. 2_000_u64 {
            if black_box(flat.contains(&cov_id)) {
                // println!("{:?}", cov_id);
                let cov = cov_id.saturating_add(1);
                let _blackbox = black_box(cov);
            }
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_coverage_len() {
        let mut coverage = InputCoverage::new();
        coverage.all_cov_id.push(1);
        coverage.all_cov_id.push(2);
        assert_eq!(coverage.coverage_len(), 2);
    }

    #[test]
    fn test_messages_coverage() {
        let mut coverage = InputCoverage::new();
        coverage.all_cov_id.push(1);
        coverage.all_cov_id.push(2);
        let messages = coverage.messages_coverage();
        assert_eq!(messages.len(), 2);
        assert!(messages.contains(&1));
        assert!(messages.contains(&2));
    }

    #[test]
    fn test_add_cov() {
        let mut coverage = InputCoverage::new();
        let trace = CoverageTrace::from("COV=1 COV=2 COV=3".as_bytes().to_vec());
        coverage.add_cov(&trace);
        assert_eq!(coverage.coverage_len(), 3);
        assert!(coverage.messages_coverage().contains(&1));
        assert!(coverage.messages_coverage().contains(&2));
        assert!(coverage.messages_coverage().contains(&3));
    }

    #[test]
    fn test_add_cov_deduplication() {
        let mut coverage = InputCoverage::new();
        let trace1 = CoverageTrace::from("COV=1 COV=2 COV=3".as_bytes().to_vec());
        let trace2 = CoverageTrace::from("COV=2 COV=3 COV=4".as_bytes().to_vec());
        coverage.add_cov(&trace1);
        coverage.add_cov(&trace2);
        assert_eq!(coverage.coverage_len(), 4);
    }

    #[test]
    fn test_debug_format() {
        let mut coverage = InputCoverage::new();
        coverage.all_cov_id.push(1);
        coverage.all_cov_id.push(2);
        let debug_output = format!("{:?}", coverage);
        assert!(debug_output.contains("Coverage"));
        assert!(debug_output.contains("1"));
        assert!(debug_output.contains("2"));
    }

    #[test]
    fn test_save() {
        let mut coverage = InputCoverage::new();
        coverage.all_cov_id.push(1);
        coverage.all_cov_id.push(2);
        coverage.all_cov_id.push(3);

        let output = TempDir::new().unwrap().into_path();
        let cov_trace_path = PhinkFiles::new(output.clone())
            .make_all()
            .path(CoverageTracePath);

        coverage.save(output).unwrap();

        let content = std::fs::read_to_string(cov_trace_path).unwrap();
        let lines: Vec<String> = content.lines().map(String::from).collect();

        assert_eq!(
            lines,
            Vec::from_iter(vec!["1".to_string(), "2".to_string(), "3".to_string()])
        );
    }
}
