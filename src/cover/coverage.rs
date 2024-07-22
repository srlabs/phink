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
}

#[derive(Clone, Debug)]
struct CoverageEntry {
    pub raw: CoverageTrace,
    pub parsed: HashMap<u64, u64>,
}

impl Debug for Coverage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Coverage {{ branches: [")?;
        for (i, entry) in self.branches.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "CoverageEntry {{ raw: {:?}, parsed: {{", entry.raw)?;
            for (j, (&key, _)) in entry.parsed.iter().enumerate() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", key)?;
            }
            write!(f, "}} }}")?;
        }
        write!(f, "] }}")
    }
}

impl Coverage {
    pub fn new() -> Self {
        Coverage {
            branches: Vec::new(),
        }
    }

    pub fn add_cov(&mut self, coverage: &CoverageTrace) {
        let parsed = Self::parse_coverage(coverage);
        self.branches.push(CoverageEntry {
            raw: coverage.clone(),
            parsed,
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
            .branches
            .iter()
            .map(|entry| String::from_utf8_lossy(&entry.raw).to_string())
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
        let flattened_cov: HashMap<u64, u64> = self
            .branches
            .iter()
            .flat_map(|entry| entry.parsed.clone())
            .fold(HashMap::new(), |mut acc, (k, v)| {
                *acc.entry(k).or_insert(0) += v;
                acc
            });

        #[cfg(not(fuzzing))]
        {
            println!(
                "[🚧DEBUG TRACE] Coverage size of {} {:?}",
                flattened_cov.len(),
                flattened_cov
            );
        }

        /// We assume that the instrumentation will never insert more than
        /// `2_000` artificial branches This value should be big enough
        /// to handle most of smart-contract, even the biggest
        seq_macro::seq!(x in 0..= 2_000 {
            if flattened_cov.contains_key(&(x as u64)) {
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
