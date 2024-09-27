use serde::Deserialize;
use serde_derive::Serialize;
use std::{
    fmt::{
        Display,
        Formatter,
    },
    path::{
        Path,
        PathBuf,
    },
};

pub const DEFAULT_PATH_PATTERN_INSTRUMENTEDPATH: &str = "ink_fuzzed_";

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct InstrumentedPath {
    pub path: PathBuf,
}
impl Display for InstrumentedPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

impl From<PathBuf> for InstrumentedPath {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }
}
impl From<&str> for InstrumentedPath {
    fn from(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
        }
    }
}
impl Default for InstrumentedPath {
    /// By default, we create a random folder in `/tmp/ink_fuzzed_1`
    fn default() -> Self {
        Self {
            path: Path::new("/tmp").join(DEFAULT_PATH_PATTERN_INSTRUMENTEDPATH.to_string() + "1"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instrumenter::path::InstrumentedPath;

    #[test]
    fn test_display_for_default_instrumentedpath() {
        let inst = InstrumentedPath::default();
        println!("{}", inst);
        assert_eq!(inst.to_string(), "/tmp/ink_fuzzed_1");
    }
}
