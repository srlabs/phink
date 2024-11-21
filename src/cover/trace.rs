use serde_derive::{
    Deserialize,
    Serialize,
};

pub const COV_IDENTIFIER: &str = "COV=";
#[derive(Clone, Default, Serialize, Debug, Deserialize)]
pub struct CoverageTrace(Vec<u8>);

impl From<Vec<u8>> for CoverageTrace {
    fn from(vec: Vec<u8>) -> Self {
        CoverageTrace(vec)
    }
}

impl AsRef<[u8]> for CoverageTrace {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl CoverageTrace {
    pub fn parse_coverage(&self) -> Vec<u64> {
        let coverage_str = String::from_utf8_lossy(&self.0);
        let mut parsed = Vec::new();

        for part in coverage_str.split_whitespace() {
            if let Some(cov) = part.strip_prefix(COV_IDENTIFIER) {
                if let Ok(value) = cov.parse::<u64>() {
                    parsed.push(value);
                }
            }
        }
        parsed
    }

    pub fn remove_cov_from_trace(&self) -> String {
        String::from_utf8_lossy(self.as_ref())
            .split_whitespace()
            .filter(|&s| !s.starts_with(COV_IDENTIFIER))
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coverage_empty() {
        let trace = CoverageTrace(vec![]);
        assert_eq!(trace.parse_coverage(), Vec::<u64>::new());
    }

    #[test]
    fn test_parse_coverage_with_valid_data() {
        let data = "COV=10 COV=20 other_data COV=30".as_bytes().to_vec();
        let trace = CoverageTrace(data);
        assert_eq!(trace.parse_coverage(), vec![10, 20, 30]);
    }

    #[test]
    fn test_parse_coverage_with_invalid_data() {
        let data = "COV=10 COV=invalid COV=30".as_bytes().to_vec();
        let trace = CoverageTrace(data);
        assert_eq!(trace.parse_coverage(), vec![10, 30]);
    }

    #[test]
    fn test_new_line() {
        let data = "COV=10\nCOV=invalid\nCOV=30".as_bytes().to_vec();
        let trace = CoverageTrace(data);
        assert_eq!(trace.parse_coverage(), vec![10, 30]);
    }

    #[test]
    fn test_remove_cov_from_trace() {
        let data = "COV=10 other_data COV=20 more_data".as_bytes().to_vec();
        let trace = CoverageTrace(data);
        let cleaned = trace.remove_cov_from_trace();
        assert_eq!(cleaned, "other_data more_data");
    }

    #[test]
    fn test_yet_another_invalid() {
        let data = "COV=10 other_data COV=20rzerze more_data"
            .as_bytes()
            .to_vec();
        let trace = CoverageTrace(data);
        let cleaned = trace.remove_cov_from_trace();
        assert_eq!(cleaned, "other_data more_data");
    }
}
