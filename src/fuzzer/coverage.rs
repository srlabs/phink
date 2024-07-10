use std::{collections::HashSet, fs::File, fs::OpenOptions, hint::black_box, io::Read, io::Write};

pub type CoverageTrace = Vec<u8>;
pub const COVERAGE_PATH: &str = "./output/phink/traces.cov";

#[derive(Clone)]
pub struct Coverage {
    branches: Vec<CoverageTrace>,
}

impl Coverage {
    pub fn new() -> Self {
        Coverage {
            branches: Vec::new(),
        }
    }

    pub fn add_cov(&mut self, coverage: &CoverageTrace) {
        self.branches.push(coverage.clone());
    }

    /// This function takes a `CoverageTrace` and removes all the coverage from the trace
    /// 'COV=153 COV=154 panicked at lib.rs:157:24: index out of bounds' =>
    /// 'panicked at lib.rs:157:24: index out of bounds'
    pub fn remove_cov_from_trace(trace: CoverageTrace) -> Vec<u8> {
        let cleaned_str = String::from_utf8_lossy(&trace)
            .split_whitespace()
            .filter(|&s| !s.starts_with("COV="))
            .collect::<Vec<&str>>()
            .join(" ");

        cleaned_str.into_bytes()
    }
    pub fn save(&self) -> std::io::Result<()> {
        let mut existing_content = String::new();
        if let Ok(mut file) = File::open(COVERAGE_PATH) {
            file.read_to_string(&mut existing_content)?;
        }

        let mut trace_strings = Vec::new();

        for trace in &self.branches {
            let x = String::from_utf8_lossy(trace);
            trace_strings.push(x);
        }

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(COVERAGE_PATH)?;

        write!(file, "{}", trace_strings.join("\n"))?;

        Ok(())
    }

    /// This function create an artificial coverage to convince Ziggy that a message is interesting or not.
    #[allow(unused_doc_comments)]
    pub fn redirect_coverage(&self) {
        let flatten_cov: Vec<u8> = self.branches.clone().into_iter().flatten().collect();
        let coverage_str = Self::deduplicate(&String::from_utf8_lossy(&flatten_cov));
        let coverage_lines: Vec<&str> = coverage_str.split('\n').collect();

        #[cfg(not(fuzzing))]
        {
            println!(
                "[🚧DEBUG TRACE] Coverage size of {} {:?}",
                coverage_lines.len(),
                coverage_lines
            );
        }

        /// We assume that the instrumentation will never insert more than `2_000` artificial branches
        /// This value should be big enough to handle most of smart-contract, even the biggest
        seq_macro::seq!(x in 0..= 2_000 {
            let target = format!("COV={}", x);
            if coverage_lines.contains(&target.as_str()) {
                let _ = black_box(x + 1);
            }
        });
    }

    /// A simple helper to remove some duplicated lines from a `&str`
    /// This is used mainly to remove coverage returns being inserted many times in the debug vector
    /// in case of any `iter()`, `for` loop and so on
    /// # Arguments
    /// * `input`: The string to deduplicate
    pub fn deduplicate(input: &str) -> String {
        let mut unique_lines = HashSet::new();
        input
            .lines()
            .filter(|&line| unique_lines.insert(line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_cov_from_trace_simple() {
        let input = b"COV=153 panicked at ..x/lib.rs:157:24: index out of bounds".to_vec();
        let expected_output = b"panicked at ..x/lib.rs:157:24: index out of bounds".to_vec();
        assert_eq!(Coverage::remove_cov_from_trace(input), expected_output);
    }

    #[test]
    fn test_remove_cov_from_trace_multiple() {
        let input = b"COV=153 COV=154 panicked at ..x/lib.rs:157:24: index out of bounds".to_vec();
        let expected_output = b"panicked at ..x/lib.rs:157:24: index out of bounds".to_vec();
        assert_eq!(Coverage::remove_cov_from_trace(input), expected_output);
    }

    #[test]
    fn test_remove_cov_from_trace_with_other_text() {
        let input =
            b"error COV=153 occurred at ..x/lib.rs:157:24: COV=154 index out of bounds".to_vec();
        let expected_output = b"error occurred at ..x/lib.rs:157:24: index out of bounds".to_vec();
        assert_eq!(Coverage::remove_cov_from_trace(input), expected_output);
    }

    #[test]
    fn test_remove_cov_from_trace_no_cov() {
        let input = b"panicked at ..x/lib.rs:157:24: index out of bounds".to_vec();
        let expected_output = b"panicked at ..x/lib.rs:157:24: index out of bounds".to_vec();
        assert_eq!(Coverage::remove_cov_from_trace(input), expected_output);
    }

    #[test]
    fn test_remove_cov_from_trace_empty() {
        let input = b"".to_vec();
        let expected_output = b"".to_vec();
        assert_eq!(Coverage::remove_cov_from_trace(input), expected_output);
    }

    #[test]
    fn test_remove_cov_from_trace_mixed() {
        let input =
            b"panicked COV=12345 at COV=6789 ..x/lib.rs:157:24: index out of COV=98765 bounds"
                .to_vec();
        let expected_output = b"panicked at ..x/lib.rs:157:24: index out of bounds".to_vec();
        assert_eq!(Coverage::remove_cov_from_trace(input), expected_output);
    }
}
