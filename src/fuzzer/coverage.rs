use crate::utils;

pub type CoverageTrace = Vec<u8>;
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

    /// This function create an artificial coverage to convince ziggy that a message is interesting or not.
    /// TODO! Refactor the 300, it should change depending the contract
    pub fn redirect_coverage(self) {
        // Flatten the branches and collect into a Vec<u8>
        let flatten_cov: Vec<u8> = self.branches.into_iter().flatten().collect();
        // We deduplicate the coverage in case of loop in the contract that wouldn't necessarily
        // Improve the coverage better, and also to avoid duplicate call inside a call
        let coverage_str = utils::deduplicate(&String::from_utf8_lossy(&flatten_cov));

        seq_macro::seq!(x in 0..=300 {
           if coverage_str.contains(&format!("COV={}", x)) {
                let _ = 1 + 1;
            }
        });
    }
}
