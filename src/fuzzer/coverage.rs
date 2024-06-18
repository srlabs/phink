use crate::utils;
use std::hint::black_box;

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
        let flatten_cov: Vec<u8> = self.branches.into_iter().flatten().collect();
        let coverage_str = utils::deduplicate(&String::from_utf8_lossy(&flatten_cov));
        let coverage_lines: Vec<&str> = coverage_str.split('\n').collect();

        println!("TRACE : {:?}", coverage_lines);
        // seq_macro::seq!(x in 0..=500 {
        //     let target = format!("COV={}", x);
        //     if coverage_lines.contains(&target.as_str()) {
        //         let a = black_box(1 + 1);
        //         let _b = black_box(a + 1);
        //         println!("COV={}", x);
        //     }
        // });

        let target = format!("COV={}", 0);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 0);
        }
        let target = format!("COV={}", 1);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 1);
        }
        let target = format!("COV={}", 2);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 2);
        }
        let target = format!("COV={}", 3);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 3);
        }
        let target = format!("COV={}", 4);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 4);
        }
        let target = format!("COV={}", 5);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 5);
        }
        let target = format!("COV={}", 6);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 6);
        }
        let target = format!("COV={}", 7);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 7);
        }
        let target = format!("COV={}", 8);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 8);
        }
        let target = format!("COV={}", 9);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 9);
        }
        let target = format!("COV={}", 10);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 10);
        }
        let target = format!("COV={}", 11);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 11);
        }
        let target = format!("COV={}", 12);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 12);
        }
        let target = format!("COV={}", 13);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 13);
        }
        let target = format!("COV={}", 14);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 14);
        }
        let target = format!("COV={}", 15);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 15);
        }
        let target = format!("COV={}", 16);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 16);
        }
        let target = format!("COV={}", 17);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 17);
        }
        let target = format!("COV={}", 18);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 18);
        }
        let target = format!("COV={}", 19);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 19);
        }
        let target = format!("COV={}", 20);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 20);
        }
        let target = format!("COV={}", 21);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 21);
        }
        let target = format!("COV={}", 22);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 22);
        }
        let target = format!("COV={}", 23);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 23);
        }
        let target = format!("COV={}", 24);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 24);
        }
        let target = format!("COV={}", 25);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 25);
        }
        let target = format!("COV={}", 26);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 26);
        }
        let target = format!("COV={}", 27);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 27);
        }
        let target = format!("COV={}", 28);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 28);
        }
        let target = format!("COV={}", 29);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 29);
        }
        let target = format!("COV={}", 30);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 30);
        }
        let target = format!("COV={}", 31);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 31);
        }
        let target = format!("COV={}", 32);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 32);
        }
        let target = format!("COV={}", 33);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 33);
        }
        let target = format!("COV={}", 34);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 34);
        }
        let target = format!("COV={}", 35);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 35);
        }
        let target = format!("COV={}", 36);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 36);
        }
        let target = format!("COV={}", 37);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 37);
        }
        let target = format!("COV={}", 38);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 38);
        }
        let target = format!("COV={}", 39);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 39);
        }
        let target = format!("COV={}", 40);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 40);
        }
        let target = format!("COV={}", 41);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 41);
        }
        let target = format!("COV={}", 42);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 42);
        }
        let target = format!("COV={}", 43);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 43);
        }
        let target = format!("COV={}", 44);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 44);
        }
        let target = format!("COV={}", 45);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 45);
        }
        let target = format!("COV={}", 46);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 46);
        }
        let target = format!("COV={}", 47);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 47);
        }
        let target = format!("COV={}", 48);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 48);
        }
        let target = format!("COV={}", 49);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 49);
        }
        let target = format!("COV={}", 50);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 50);
        }
        let target = format!("COV={}", 51);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 51);
        }
        let target = format!("COV={}", 52);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 52);
        }
        let target = format!("COV={}", 53);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 53);
        }
        let target = format!("COV={}", 54);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 54);
        }
        let target = format!("COV={}", 55);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 55);
        }
        let target = format!("COV={}", 56);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 56);
        }
        let target = format!("COV={}", 57);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 57);
        }
        let target = format!("COV={}", 58);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 58);
        }
        let target = format!("COV={}", 59);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 59);
        }
        let target = format!("COV={}", 60);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 60);
        }
        let target = format!("COV={}", 61);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 61);
        }
        let target = format!("COV={}", 62);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 62);
        }
        let target = format!("COV={}", 63);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 63);
        }
        let target = format!("COV={}", 64);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 64);
        }
        let target = format!("COV={}", 65);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 65);
        }
        let target = format!("COV={}", 66);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 66);
        }
        let target = format!("COV={}", 67);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 67);
        }
        let target = format!("COV={}", 68);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 68);
        }
        let target = format!("COV={}", 69);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 69);
        }
        let target = format!("COV={}", 70);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 70);
        }
        let target = format!("COV={}", 71);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 71);
        }
        let target = format!("COV={}", 72);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 72);
        }
        let target = format!("COV={}", 73);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 73);
        }
        let target = format!("COV={}", 74);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 74);
        }
        let target = format!("COV={}", 75);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 75);
        }
        let target = format!("COV={}", 76);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 76);
        }
        let target = format!("COV={}", 77);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 77);
        }
        let target = format!("COV={}", 78);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 78);
        }
        let target = format!("COV={}", 79);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 79);
        }
        let target = format!("COV={}", 80);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 80);
        }
        let target = format!("COV={}", 81);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 81);
        }
        let target = format!("COV={}", 82);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 82);
        }
        let target = format!("COV={}", 83);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 83);
        }
        let target = format!("COV={}", 84);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 84);
        }
        let target = format!("COV={}", 85);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 85);
        }
        let target = format!("COV={}", 86);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 86);
        }
        let target = format!("COV={}", 87);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 87);
        }
        let target = format!("COV={}", 88);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 88);
        }
        let target = format!("COV={}", 89);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 89);
        }
        let target = format!("COV={}", 90);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 90);
        }
        let target = format!("COV={}", 91);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 91);
        }
        let target = format!("COV={}", 92);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 92);
        }
        let target = format!("COV={}", 93);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 93);
        }
        let target = format!("COV={}", 94);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 94);
        }
        let target = format!("COV={}", 95);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 95);
        }
        let target = format!("COV={}", 96);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 96);
        }
        let target = format!("COV={}", 97);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 97);
        }
        let target = format!("COV={}", 98);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 98);
        }
        let target = format!("COV={}", 99);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 99);
        }
        let target = format!("COV={}", 100);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 100);
        }
        let target = format!("COV={}", 101);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 101);
        }
        let target = format!("COV={}", 102);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 102);
        }
        let target = format!("COV={}", 103);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 103);
        }
        let target = format!("COV={}", 104);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 104);
        }
        let target = format!("COV={}", 105);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 105);
        }
        let target = format!("COV={}", 106);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 106);
        }
        let target = format!("COV={}", 107);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 107);
        }
        let target = format!("COV={}", 108);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 108);
        }
        let target = format!("COV={}", 109);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 109);
        }
        let target = format!("COV={}", 110);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 110);
        }
        let target = format!("COV={}", 111);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 111);
        }
        let target = format!("COV={}", 112);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 112);
        }
        let target = format!("COV={}", 113);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 113);
        }
        let target = format!("COV={}", 114);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 114);
        }
        let target = format!("COV={}", 115);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 115);
        }
        let target = format!("COV={}", 116);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 116);
        }
        let target = format!("COV={}", 117);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 117);
        }
        let target = format!("COV={}", 118);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 118);
        }
        let target = format!("COV={}", 119);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 119);
        }
        let target = format!("COV={}", 120);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 120);
        }
        let target = format!("COV={}", 121);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 121);
        }
        let target = format!("COV={}", 122);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 122);
        }
        let target = format!("COV={}", 123);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 123);
        }
        let target = format!("COV={}", 124);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 124);
        }
        let target = format!("COV={}", 125);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 125);
        }
        let target = format!("COV={}", 126);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 126);
        }
        let target = format!("COV={}", 127);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 127);
        }
        let target = format!("COV={}", 128);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 128);
        }
        let target = format!("COV={}", 129);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 129);
        }
        let target = format!("COV={}", 130);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 130);
        }
        let target = format!("COV={}", 131);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 131);
        }
        let target = format!("COV={}", 132);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 132);
        }
        let target = format!("COV={}", 133);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 133);
        }
        let target = format!("COV={}", 134);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 134);
        }
        let target = format!("COV={}", 135);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 135);
        }
        let target = format!("COV={}", 136);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 136);
        }
        let target = format!("COV={}", 137);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 137);
        }
        let target = format!("COV={}", 138);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 138);
        }
        let target = format!("COV={}", 139);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 139);
        }
        let target = format!("COV={}", 140);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 140);
        }
        let target = format!("COV={}", 141);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 141);
        }
        let target = format!("COV={}", 142);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 142);
        }
        let target = format!("COV={}", 143);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 143);
        }
        let target = format!("COV={}", 144);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 144);
        }
        let target = format!("COV={}", 145);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 145);
        }
        let target = format!("COV={}", 146);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 146);
        }
        let target = format!("COV={}", 147);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 147);
        }
        let target = format!("COV={}", 148);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 148);
        }
        let target = format!("COV={}", 149);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 149);
        }
        let target = format!("COV={}", 150);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 150);
        }
        let target = format!("COV={}", 151);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 151);
        }
        let target = format!("COV={}", 152);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 152);
        }
        let target = format!("COV={}", 153);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 153);
        }
        let target = format!("COV={}", 154);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 154);
        }
        let target = format!("COV={}", 155);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 155);
        }
        let target = format!("COV={}", 156);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 156);
        }
        let target = format!("COV={}", 157);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 157);
        }
        let target = format!("COV={}", 158);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 158);
        }
        let target = format!("COV={}", 159);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 159);
        }
        let target = format!("COV={}", 160);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 160);
        }
        let target = format!("COV={}", 161);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 161);
        }
        let target = format!("COV={}", 162);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 162);
        }
        let target = format!("COV={}", 163);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 163);
        }
        let target = format!("COV={}", 164);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 164);
        }
        let target = format!("COV={}", 165);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 165);
        }
        let target = format!("COV={}", 166);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 166);
        }
        let target = format!("COV={}", 167);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 167);
        }
        let target = format!("COV={}", 168);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 168);
        }
        let target = format!("COV={}", 169);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 169);
        }
        let target = format!("COV={}", 170);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 170);
        }
        let target = format!("COV={}", 171);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 171);
        }
        let target = format!("COV={}", 172);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 172);
        }
        let target = format!("COV={}", 173);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 173);
        }
        let target = format!("COV={}", 174);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 174);
        }
        let target = format!("COV={}", 175);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 175);
        }
        let target = format!("COV={}", 176);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 176);
        }
        let target = format!("COV={}", 177);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 177);
        }
        let target = format!("COV={}", 178);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 178);
        }
        let target = format!("COV={}", 179);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 179);
        }
        let target = format!("COV={}", 180);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 180);
        }
        let target = format!("COV={}", 181);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 181);
        }
        let target = format!("COV={}", 182);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 182);
        }
        let target = format!("COV={}", 183);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 183);
        }
        let target = format!("COV={}", 184);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 184);
        }
        let target = format!("COV={}", 185);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 185);
        }
        let target = format!("COV={}", 186);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 186);
        }
        let target = format!("COV={}", 187);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 187);
        }
        let target = format!("COV={}", 188);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 188);
        }
        let target = format!("COV={}", 189);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 189);
        }
        let target = format!("COV={}", 190);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 190);
        }
        let target = format!("COV={}", 191);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 191);
        }
        let target = format!("COV={}", 192);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 192);
        }
        let target = format!("COV={}", 193);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 193);
        }
        let target = format!("COV={}", 194);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 194);
        }
        let target = format!("COV={}", 195);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 195);
        }
        let target = format!("COV={}", 196);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 196);
        }
        let target = format!("COV={}", 197);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 197);
        }
        let target = format!("COV={}", 198);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 198);
        }
        let target = format!("COV={}", 199);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 199);
        }
        let target = format!("COV={}", 200);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 200);
        }
        let target = format!("COV={}", 201);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 201);
        }
        let target = format!("COV={}", 202);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 202);
        }
        let target = format!("COV={}", 203);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 203);
        }
        let target = format!("COV={}", 204);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 204);
        }
        let target = format!("COV={}", 205);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 205);
        }
        let target = format!("COV={}", 206);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 206);
        }
        let target = format!("COV={}", 207);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 207);
        }
        let target = format!("COV={}", 208);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 208);
        }
        let target = format!("COV={}", 209);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 209);
        }
        let target = format!("COV={}", 210);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 210);
        }
        let target = format!("COV={}", 211);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 211);
        }
        let target = format!("COV={}", 212);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 212);
        }
        let target = format!("COV={}", 213);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 213);
        }
        let target = format!("COV={}", 214);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 214);
        }
        let target = format!("COV={}", 215);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 215);
        }
        let target = format!("COV={}", 216);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 216);
        }
        let target = format!("COV={}", 217);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 217);
        }
        let target = format!("COV={}", 218);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 218);
        }
        let target = format!("COV={}", 219);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 219);
        }
        let target = format!("COV={}", 220);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 220);
        }
        let target = format!("COV={}", 221);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 221);
        }
        let target = format!("COV={}", 222);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 222);
        }
        let target = format!("COV={}", 223);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 223);
        }
        let target = format!("COV={}", 224);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 224);
        }
        let target = format!("COV={}", 225);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 225);
        }
        let target = format!("COV={}", 226);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 226);
        }
        let target = format!("COV={}", 227);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 227);
        }
        let target = format!("COV={}", 228);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 228);
        }
        let target = format!("COV={}", 229);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 229);
        }
        let target = format!("COV={}", 230);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 230);
        }
        let target = format!("COV={}", 231);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 231);
        }
        let target = format!("COV={}", 232);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 232);
        }
        let target = format!("COV={}", 233);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 233);
        }
        let target = format!("COV={}", 234);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 234);
        }
        let target = format!("COV={}", 235);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 235);
        }
        let target = format!("COV={}", 236);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 236);
        }
        let target = format!("COV={}", 237);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 237);
        }
        let target = format!("COV={}", 238);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 238);
        }
        let target = format!("COV={}", 239);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 239);
        }
        let target = format!("COV={}", 240);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 240);
        }
        let target = format!("COV={}", 241);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 241);
        }
        let target = format!("COV={}", 242);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 242);
        }
        let target = format!("COV={}", 243);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 243);
        }
        let target = format!("COV={}", 244);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 244);
        }
        let target = format!("COV={}", 245);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 245);
        }
        let target = format!("COV={}", 246);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 246);
        }
        let target = format!("COV={}", 247);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 247);
        }
        let target = format!("COV={}", 248);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 248);
        }
        let target = format!("COV={}", 249);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 249);
        }
        let target = format!("COV={}", 250);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 250);
        }
        let target = format!("COV={}", 251);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 251);
        }
        let target = format!("COV={}", 252);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 252);
        }
        let target = format!("COV={}", 253);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 253);
        }
        let target = format!("COV={}", 254);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 254);
        }
        let target = format!("COV={}", 255);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 255);
        }
        let target = format!("COV={}", 256);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 256);
        }
        let target = format!("COV={}", 257);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 257);
        }
        let target = format!("COV={}", 258);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 258);
        }
        let target = format!("COV={}", 259);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 259);
        }
        let target = format!("COV={}", 260);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 260);
        }
        let target = format!("COV={}", 261);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 261);
        }
        let target = format!("COV={}", 262);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 262);
        }
        let target = format!("COV={}", 263);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 263);
        }
        let target = format!("COV={}", 264);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 264);
        }
        let target = format!("COV={}", 265);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 265);
        }
        let target = format!("COV={}", 266);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 266);
        }
        let target = format!("COV={}", 267);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 267);
        }
        let target = format!("COV={}", 268);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 268);
        }
        let target = format!("COV={}", 269);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 269);
        }
        let target = format!("COV={}", 270);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 270);
        }
        let target = format!("COV={}", 271);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 271);
        }
        let target = format!("COV={}", 272);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 272);
        }
        let target = format!("COV={}", 273);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 273);
        }
        let target = format!("COV={}", 274);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 274);
        }
        let target = format!("COV={}", 275);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 275);
        }
        let target = format!("COV={}", 276);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 276);
        }
        let target = format!("COV={}", 277);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 277);
        }
        let target = format!("COV={}", 278);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 278);
        }
        let target = format!("COV={}", 279);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 279);
        }
        let target = format!("COV={}", 280);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 280);
        }
        let target = format!("COV={}", 281);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 281);
        }
        let target = format!("COV={}", 282);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 282);
        }
        let target = format!("COV={}", 283);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 283);
        }
        let target = format!("COV={}", 284);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 284);
        }
        let target = format!("COV={}", 285);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 285);
        }
        let target = format!("COV={}", 286);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 286);
        }
        let target = format!("COV={}", 287);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 287);
        }
        let target = format!("COV={}", 288);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 288);
        }
        let target = format!("COV={}", 289);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 289);
        }
        let target = format!("COV={}", 290);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 290);
        }
        let target = format!("COV={}", 291);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 291);
        }
        let target = format!("COV={}", 292);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 292);
        }
        let target = format!("COV={}", 293);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 293);
        }
        let target = format!("COV={}", 294);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 294);
        }
        let target = format!("COV={}", 295);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 295);
        }
        let target = format!("COV={}", 296);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 296);
        }
        let target = format!("COV={}", 297);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 297);
        }
        let target = format!("COV={}", 298);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 298);
        }
        let target = format!("COV={}", 299);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 299);
        }
        let target = format!("COV={}", 300);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 300);
        }
        let target = format!("COV={}", 301);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 301);
        }
        let target = format!("COV={}", 302);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 302);
        }
        let target = format!("COV={}", 303);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 303);
        }
        let target = format!("COV={}", 304);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 304);
        }
        let target = format!("COV={}", 305);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 305);
        }
        let target = format!("COV={}", 306);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 306);
        }
        let target = format!("COV={}", 307);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 307);
        }
        let target = format!("COV={}", 308);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 308);
        }
        let target = format!("COV={}", 309);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 309);
        }
        let target = format!("COV={}", 310);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 310);
        }
        let target = format!("COV={}", 311);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 311);
        }
        let target = format!("COV={}", 312);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 312);
        }
        let target = format!("COV={}", 313);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 313);
        }
        let target = format!("COV={}", 314);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 314);
        }
        let target = format!("COV={}", 315);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 315);
        }
        let target = format!("COV={}", 316);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 316);
        }
        let target = format!("COV={}", 317);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 317);
        }
        let target = format!("COV={}", 318);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 318);
        }
        let target = format!("COV={}", 319);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 319);
        }
        let target = format!("COV={}", 320);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 320);
        }
        let target = format!("COV={}", 321);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 321);
        }
        let target = format!("COV={}", 322);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 322);
        }
        let target = format!("COV={}", 323);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 323);
        }
        let target = format!("COV={}", 324);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 324);
        }
        let target = format!("COV={}", 325);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 325);
        }
        let target = format!("COV={}", 326);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 326);
        }
        let target = format!("COV={}", 327);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 327);
        }
        let target = format!("COV={}", 328);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 328);
        }
        let target = format!("COV={}", 329);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 329);
        }
        let target = format!("COV={}", 330);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 330);
        }
        let target = format!("COV={}", 331);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 331);
        }
        let target = format!("COV={}", 332);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 332);
        }
        let target = format!("COV={}", 333);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 333);
        }
        let target = format!("COV={}", 334);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 334);
        }
        let target = format!("COV={}", 335);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 335);
        }
        let target = format!("COV={}", 336);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 336);
        }
        let target = format!("COV={}", 337);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 337);
        }
        let target = format!("COV={}", 338);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 338);
        }
        let target = format!("COV={}", 339);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 339);
        }
        let target = format!("COV={}", 340);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 340);
        }
        let target = format!("COV={}", 341);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 341);
        }
        let target = format!("COV={}", 342);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 342);
        }
        let target = format!("COV={}", 343);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 343);
        }
        let target = format!("COV={}", 344);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 344);
        }
        let target = format!("COV={}", 345);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 345);
        }
        let target = format!("COV={}", 346);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 346);
        }
        let target = format!("COV={}", 347);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 347);
        }
        let target = format!("COV={}", 348);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 348);
        }
        let target = format!("COV={}", 349);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 349);
        }
        let target = format!("COV={}", 350);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 350);
        }
        let target = format!("COV={}", 351);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 351);
        }
        let target = format!("COV={}", 352);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 352);
        }
        let target = format!("COV={}", 353);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 353);
        }
        let target = format!("COV={}", 354);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 354);
        }
        let target = format!("COV={}", 355);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 355);
        }
        let target = format!("COV={}", 356);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 356);
        }
        let target = format!("COV={}", 357);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 357);
        }
        let target = format!("COV={}", 358);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 358);
        }
        let target = format!("COV={}", 359);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 359);
        }
        let target = format!("COV={}", 360);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 360);
        }
        let target = format!("COV={}", 361);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 361);
        }
        let target = format!("COV={}", 362);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 362);
        }
        let target = format!("COV={}", 363);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 363);
        }
        let target = format!("COV={}", 364);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 364);
        }
        let target = format!("COV={}", 365);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 365);
        }
        let target = format!("COV={}", 366);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 366);
        }
        let target = format!("COV={}", 367);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 367);
        }
        let target = format!("COV={}", 368);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 368);
        }
        let target = format!("COV={}", 369);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 369);
        }
        let target = format!("COV={}", 370);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 370);
        }
        let target = format!("COV={}", 371);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 371);
        }
        let target = format!("COV={}", 372);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 372);
        }
        let target = format!("COV={}", 373);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 373);
        }
        let target = format!("COV={}", 374);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 374);
        }
        let target = format!("COV={}", 375);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 375);
        }
        let target = format!("COV={}", 376);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 376);
        }
        let target = format!("COV={}", 377);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 377);
        }
        let target = format!("COV={}", 378);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 378);
        }
        let target = format!("COV={}", 379);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 379);
        }
        let target = format!("COV={}", 380);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 380);
        }
        let target = format!("COV={}", 381);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 381);
        }
        let target = format!("COV={}", 382);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 382);
        }
        let target = format!("COV={}", 383);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 383);
        }
        let target = format!("COV={}", 384);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 384);
        }
        let target = format!("COV={}", 385);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 385);
        }
        let target = format!("COV={}", 386);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 386);
        }
        let target = format!("COV={}", 387);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 387);
        }
        let target = format!("COV={}", 388);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 388);
        }
        let target = format!("COV={}", 389);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 389);
        }
        let target = format!("COV={}", 390);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 390);
        }
        let target = format!("COV={}", 391);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 391);
        }
        let target = format!("COV={}", 392);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 392);
        }
        let target = format!("COV={}", 393);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 393);
        }
        let target = format!("COV={}", 394);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 394);
        }
        let target = format!("COV={}", 395);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 395);
        }
        let target = format!("COV={}", 396);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 396);
        }
        let target = format!("COV={}", 397);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 397);
        }
        let target = format!("COV={}", 398);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 398);
        }
        let target = format!("COV={}", 399);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 399);
        }
        let target = format!("COV={}", 400);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 400);
        }
        let target = format!("COV={}", 401);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 401);
        }
        let target = format!("COV={}", 402);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 402);
        }
        let target = format!("COV={}", 403);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 403);
        }
        let target = format!("COV={}", 404);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 404);
        }
        let target = format!("COV={}", 405);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 405);
        }
        let target = format!("COV={}", 406);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 406);
        }
        let target = format!("COV={}", 407);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 407);
        }
        let target = format!("COV={}", 408);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 408);
        }
        let target = format!("COV={}", 409);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 409);
        }
        let target = format!("COV={}", 410);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 410);
        }
        let target = format!("COV={}", 411);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 411);
        }
        let target = format!("COV={}", 412);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 412);
        }
        let target = format!("COV={}", 413);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 413);
        }
        let target = format!("COV={}", 414);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 414);
        }
        let target = format!("COV={}", 415);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 415);
        }
        let target = format!("COV={}", 416);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 416);
        }
        let target = format!("COV={}", 417);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 417);
        }
        let target = format!("COV={}", 418);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 418);
        }
        let target = format!("COV={}", 419);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 419);
        }
        let target = format!("COV={}", 420);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 420);
        }
        let target = format!("COV={}", 421);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 421);
        }
        let target = format!("COV={}", 422);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 422);
        }
        let target = format!("COV={}", 423);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 423);
        }
        let target = format!("COV={}", 424);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 424);
        }
        let target = format!("COV={}", 425);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 425);
        }
        let target = format!("COV={}", 426);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 426);
        }
        let target = format!("COV={}", 427);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 427);
        }
        let target = format!("COV={}", 428);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 428);
        }
        let target = format!("COV={}", 429);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 429);
        }
        let target = format!("COV={}", 430);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 430);
        }
        let target = format!("COV={}", 431);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 431);
        }
        let target = format!("COV={}", 432);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 432);
        }
        let target = format!("COV={}", 433);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 433);
        }
        let target = format!("COV={}", 434);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 434);
        }
        let target = format!("COV={}", 435);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 435);
        }
        let target = format!("COV={}", 436);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 436);
        }
        let target = format!("COV={}", 437);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 437);
        }
        let target = format!("COV={}", 438);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 438);
        }
        let target = format!("COV={}", 439);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 439);
        }
        let target = format!("COV={}", 440);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 440);
        }
        let target = format!("COV={}", 441);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 441);
        }
        let target = format!("COV={}", 442);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 442);
        }
        let target = format!("COV={}", 443);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 443);
        }
        let target = format!("COV={}", 444);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 444);
        }
        let target = format!("COV={}", 445);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 445);
        }
        let target = format!("COV={}", 446);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 446);
        }
        let target = format!("COV={}", 447);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 447);
        }
        let target = format!("COV={}", 448);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 448);
        }
        let target = format!("COV={}", 449);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 449);
        }
        let target = format!("COV={}", 450);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 450);
        }
        let target = format!("COV={}", 451);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 451);
        }
        let target = format!("COV={}", 452);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 452);
        }
        let target = format!("COV={}", 453);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 453);
        }
        let target = format!("COV={}", 454);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 454);
        }
        let target = format!("COV={}", 455);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 455);
        }
        let target = format!("COV={}", 456);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 456);
        }
        let target = format!("COV={}", 457);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 457);
        }
        let target = format!("COV={}", 458);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 458);
        }
        let target = format!("COV={}", 459);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 459);
        }
        let target = format!("COV={}", 460);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 460);
        }
        let target = format!("COV={}", 461);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 461);
        }
        let target = format!("COV={}", 462);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 462);
        }
        let target = format!("COV={}", 463);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 463);
        }
        let target = format!("COV={}", 464);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 464);
        }
        let target = format!("COV={}", 465);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 465);
        }
        let target = format!("COV={}", 466);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 466);
        }
        let target = format!("COV={}", 467);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 467);
        }
        let target = format!("COV={}", 468);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 468);
        }
        let target = format!("COV={}", 469);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 469);
        }
        let target = format!("COV={}", 470);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 470);
        }
        let target = format!("COV={}", 471);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 471);
        }
        let target = format!("COV={}", 472);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 472);
        }
        let target = format!("COV={}", 473);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 473);
        }
        let target = format!("COV={}", 474);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 474);
        }
        let target = format!("COV={}", 475);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 475);
        }
        let target = format!("COV={}", 476);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 476);
        }
        let target = format!("COV={}", 477);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 477);
        }
        let target = format!("COV={}", 478);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 478);
        }
        let target = format!("COV={}", 479);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 479);
        }
        let target = format!("COV={}", 480);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 480);
        }
        let target = format!("COV={}", 481);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 481);
        }
        let target = format!("COV={}", 482);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 482);
        }
        let target = format!("COV={}", 483);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 483);
        }
        let target = format!("COV={}", 484);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 484);
        }
        let target = format!("COV={}", 485);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 485);
        }
        let target = format!("COV={}", 486);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 486);
        }
        let target = format!("COV={}", 487);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 487);
        }
        let target = format!("COV={}", 488);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 488);
        }
        let target = format!("COV={}", 489);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 489);
        }
        let target = format!("COV={}", 490);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 490);
        }
        let target = format!("COV={}", 491);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 491);
        }
        let target = format!("COV={}", 492);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 492);
        }
        let target = format!("COV={}", 493);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 493);
        }
        let target = format!("COV={}", 494);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 494);
        }
        let target = format!("COV={}", 495);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 495);
        }
        let target = format!("COV={}", 496);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 496);
        }
        let target = format!("COV={}", 497);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 497);
        }
        let target = format!("COV={}", 498);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 498);
        }
        let target = format!("COV={}", 499);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 499);
        }
        let target = format!("COV={}", 500);
        if coverage_lines.contains(&target.as_str()) {
            let a = black_box(1 + 1);
            let _b = black_box(a + 1);
            println!("COV={}", 500);
        }
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
