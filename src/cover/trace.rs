pub const COV_IDENTIFIER: &str = "COV="; // todo: apply me to everythin

#[derive(Clone, Default)]
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

    pub fn remove_cov_from_trace(self) -> Vec<u8> {
        let cleaned_str = String::from_utf8_lossy(self.as_ref())
            .split_whitespace()
            .filter(|&s| !s.starts_with(COV_IDENTIFIER))
            .collect::<Vec<&str>>()
            .join(" ");
        cleaned_str.into_bytes()
    }
}
