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

    /// This function takes a `CoverageTrace` and remove all the coverage from the trace
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

    /// This function create an artificial coverage to convince ziggy that a message is interesting or not.
    /// TODO! Refactor the 300, it should change depending the contract
    pub fn redirect_coverage(self) {
        // Flatten the branches and collect into a Vec<u8>
        let flatten_cov: Vec<u8> = self.branches.into_iter().flatten().collect();
        // We deduplicate the coverage in case of loop in the contract that wouldn't necessarily
        // Improve the coverage better, and also to avoid duplicate call inside a call
        let coverage_str = utils::deduplicate(&String::from_utf8_lossy(&flatten_cov));

        if coverage_str.contains(&format!("COV={}", 0)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 1)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 2)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 3)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 4)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 5)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 6)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 7)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 8)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 9)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 10)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 11)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 12)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 13)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 14)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 15)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 16)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 17)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 18)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 19)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 20)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 21)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 22)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 23)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 24)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 25)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 26)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 27)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 28)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 29)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 30)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 31)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 32)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 33)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 34)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 35)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 36)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 37)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 38)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 39)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 40)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 41)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 42)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 43)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 44)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 45)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 46)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 47)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 48)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 49)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 50)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 51)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 52)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 53)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 54)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 55)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 56)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 57)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 58)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 59)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 60)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 61)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 62)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 63)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 64)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 65)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 66)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 67)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 68)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 69)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 70)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 71)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 72)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 73)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 74)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 75)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 76)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 77)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 78)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 79)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 80)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 81)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 82)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 83)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 84)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 85)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 86)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 87)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 88)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 89)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 90)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 91)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 92)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 93)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 94)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 95)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 96)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 97)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 98)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 99)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 100)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 101)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 102)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 103)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 104)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 105)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 106)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 107)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 108)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 109)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 110)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 111)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 112)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 113)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 114)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 115)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 116)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 117)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 118)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 119)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 120)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 121)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 122)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 123)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 124)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 125)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 126)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 127)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 128)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 129)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 130)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 131)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 132)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 133)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 134)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 135)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 136)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 137)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 138)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 139)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 140)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 141)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 142)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 143)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 144)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 145)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 146)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 147)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 148)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 149)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 150)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 151)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 152)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 153)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 154)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 155)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 156)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 157)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 158)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 159)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 160)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 161)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 162)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 163)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 164)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 165)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 166)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 167)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 168)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 169)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 170)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 171)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 172)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 173)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 174)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 175)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 176)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 177)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 178)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 179)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 180)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 181)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 182)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 183)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 184)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 185)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 186)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 187)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 188)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 189)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 190)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 191)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 192)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 193)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 194)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 195)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 196)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 197)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 198)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 199)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 200)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 201)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 202)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 203)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 204)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 205)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 206)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 207)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 208)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 209)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 210)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 211)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 212)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 213)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 214)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 215)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 216)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 217)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 218)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 219)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 220)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 221)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 222)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 223)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 224)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 225)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 226)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 227)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 228)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 229)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 230)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 231)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 232)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 233)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 234)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 235)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 236)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 237)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 238)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 239)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 240)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 241)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 242)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 243)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 244)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 245)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 246)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 247)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 248)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 249)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 250)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 251)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 252)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 253)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 254)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 255)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 256)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 257)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 258)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 259)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 260)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 261)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 262)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 263)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 264)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 265)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 266)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 267)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 268)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 269)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 270)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 271)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 272)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 273)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 274)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 275)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 276)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 277)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 278)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 279)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 280)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 281)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 282)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 283)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 284)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 285)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 286)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 287)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 288)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 289)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 290)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 291)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 292)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 293)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 294)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 295)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 296)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 297)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 298)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 299)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 300)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 301)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 302)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 303)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 304)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 305)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 306)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 307)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 308)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 309)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 310)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 311)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 312)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 313)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 314)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 315)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 316)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 317)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 318)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 319)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 320)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 321)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 322)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 323)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 324)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 325)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 326)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 327)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 328)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 329)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 330)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 331)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 332)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 333)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 334)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 335)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 336)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 337)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 338)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 339)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 340)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 341)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 342)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 343)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 344)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 345)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 346)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 347)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 348)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 349)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 350)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 351)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 352)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 353)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 354)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 355)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 356)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 357)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 358)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 359)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 360)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 361)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 362)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 363)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 364)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 365)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 366)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 367)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 368)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 369)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 370)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 371)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 372)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 373)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 374)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 375)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 376)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 377)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 378)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 379)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 380)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 381)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 382)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 383)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 384)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 385)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 386)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 387)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 388)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 389)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 390)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 391)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 392)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 393)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 394)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 395)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 396)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 397)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 398)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 399)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 400)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 401)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 402)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 403)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 404)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 405)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 406)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 407)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 408)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 409)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 410)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 411)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 412)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 413)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 414)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 415)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 416)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 417)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 418)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 419)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 420)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 421)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 422)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 423)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 424)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 425)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 426)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 427)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 428)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 429)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 430)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 431)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 432)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 433)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 434)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 435)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 436)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 437)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 438)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 439)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 440)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 441)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 442)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 443)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 444)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 445)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 446)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 447)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 448)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 449)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 450)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 451)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 452)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 453)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 454)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 455)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 456)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 457)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 458)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 459)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 460)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 461)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 462)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 463)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 464)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 465)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 466)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 467)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 468)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 469)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 470)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 471)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 472)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 473)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 474)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 475)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 476)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 477)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 478)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 479)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 480)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 481)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 482)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 483)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 484)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 485)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 486)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 487)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 488)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 489)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 490)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 491)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 492)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 493)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 494)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 495)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 496)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 497)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 498)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 499)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }
        if coverage_str.contains(&format!("COV={}", 500)) {
            let a = 1 + 1;
            let _b = a + 1;
            println!("toz");
        }

        // Fake code, for coverage purposes
        // seq_macro::seq!(x in 0..=500 {
        //    if coverage_str.contains(&format!("COV={}", x)) {
        //         let a = 1 + 1;
        //         let _b = a + 1;
        //         println!("toz");
        //     }
        // });
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
