use serde::Deserialize;
use serde_derive::Serialize;
use std::path::{
    Path,
    PathBuf,
};

pub const DEFAULT_PATH_PATTERN_INSTRUMENTEDPATH: &str = "ink_fuzzed_";

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct InstrumentedPath {
    pub path: PathBuf,
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
