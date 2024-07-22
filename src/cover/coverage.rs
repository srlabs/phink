use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fmt,
    fmt::{
        Debug,
        Formatter,
    },
    fs::{
        File,
        OpenOptions,
    },
    hint::black_box,
    io::{
        Read,
        Write,
    },
};

pub type CoverageTrace = Vec<u8>;
pub const COVERAGE_PATH: &str = "./output/phink/traces.cov";

#[derive(Clone)]
pub struct Coverage {
    branches: Vec<CoverageEntry>,
    raw: Vec<CoverageTrace>,
}

#[derive(Clone, Debug)]
struct CoverageEntry {
    /// A map where the key is the ID of the parsed value of COV=..., and the value is
    /// the number of times this coverage point was hit.
    pub coverage_data: HashMap<u64, u64>,
}

impl Debug for Coverage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Coverage")
            .field("branches", &self.branches)
            .finish()
    }
}

impl Coverage {
    pub fn new() -> Self {
        Coverage {
            branches: Vec::new(),
            raw: Vec::new(),
        }
    }

    pub fn add_cov(&mut self, coverage: &CoverageTrace) {
        let parsed = Self::parse_coverage(coverage);
        self.raw.push(coverage.clone());
        self.branches.push(CoverageEntry {
            coverage_data: parsed,
        });
    }

    fn parse_coverage(coverage: &CoverageTrace) -> HashMap<u64, u64> {
        let coverage_str = String::from_utf8_lossy(coverage);
        let mut parsed = HashMap::new();

        for part in coverage_str.split_whitespace() {
            if let Some(cov) = part.strip_prefix("COV=") {
                if let Ok(value) = cov.parse::<u64>() {
                    *parsed.entry(value).or_insert(0) += 1;
                }
            }
        }

        parsed
    }

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

        let trace_strings: Vec<String> = self
            .raw
            .iter()
            .map(|trace| {
                trace
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<String>()
            })
            .collect();

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(COVERAGE_PATH)?;

        writeln!(file, "{}", trace_strings.join("\n"))?;

        Ok(())
    }

    #[allow(unused_doc_comments)]
    #[allow(clippy::identity_op)]
    pub fn redirect_coverage(&self) {
        let flattened_cov: Vec<_> = self
            .branches
            .iter()
            .flat_map(|entry| entry.coverage_data.keys().cloned())
            .collect();

        #[cfg(not(fuzzing))]
        {
            println!(
                "[🚧DEBUG TRACE] Coverage size of {} {:?}",
                self.branches.clone().len(),
                self.branches
            );
        }

        /// We assume that the instrumentation will never insert more than
        /// `2_000` artificial branches This value should be big enough
        /// to handle most of smart-contract, even the biggest
        seq_macro::seq!(x in 0..= 2_000 {
            if flattened_cov.contains(&(x as u64)) {
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
