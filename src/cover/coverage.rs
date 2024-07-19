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
        // seq_macro::seq!(x in 0..= 2_000 {
        //     if flattened_cov.contains_key(&(x as u64)) {
        //         let _ = black_box(x + 1);
        //     }
        // });
        if flattened_cov.contains_key(&(0 as u64)) {
            let _ = black_box(0 + 1);
        }
        if flattened_cov.contains_key(&(1 as u64)) {
            let _ = black_box(1 + 1);
        }
        if flattened_cov.contains_key(&(2 as u64)) {
            let _ = black_box(2 + 1);
        }
        if flattened_cov.contains_key(&(3 as u64)) {
            let _ = black_box(3 + 1);
        }
        if flattened_cov.contains_key(&(4 as u64)) {
            let _ = black_box(4 + 1);
        }
        if flattened_cov.contains_key(&(5 as u64)) {
            let _ = black_box(5 + 1);
        }
        if flattened_cov.contains_key(&(6 as u64)) {
            let _ = black_box(6 + 1);
        }
        if flattened_cov.contains_key(&(7 as u64)) {
            let _ = black_box(7 + 1);
        }
        if flattened_cov.contains_key(&(8 as u64)) {
            let _ = black_box(8 + 1);
        }
        if flattened_cov.contains_key(&(9 as u64)) {
            let _ = black_box(9 + 1);
        }
        if flattened_cov.contains_key(&(10 as u64)) {
            let _ = black_box(10 + 1);
        }
        if flattened_cov.contains_key(&(11 as u64)) {
            let _ = black_box(11 + 1);
        }
        if flattened_cov.contains_key(&(12 as u64)) {
            let _ = black_box(12 + 1);
        }
        if flattened_cov.contains_key(&(13 as u64)) {
            let _ = black_box(13 + 1);
        }
        if flattened_cov.contains_key(&(14 as u64)) {
            let _ = black_box(14 + 1);
        }
        if flattened_cov.contains_key(&(15 as u64)) {
            let _ = black_box(15 + 1);
        }
        if flattened_cov.contains_key(&(16 as u64)) {
            let _ = black_box(16 + 1);
        }
        if flattened_cov.contains_key(&(17 as u64)) {
            let _ = black_box(17 + 1);
        }
        if flattened_cov.contains_key(&(18 as u64)) {
            let _ = black_box(18 + 1);
        }
        if flattened_cov.contains_key(&(19 as u64)) {
            let _ = black_box(19 + 1);
        }
        if flattened_cov.contains_key(&(20 as u64)) {
            let _ = black_box(20 + 1);
        }
        if flattened_cov.contains_key(&(21 as u64)) {
            let _ = black_box(21 + 1);
        }
        if flattened_cov.contains_key(&(22 as u64)) {
            let _ = black_box(22 + 1);
        }
        if flattened_cov.contains_key(&(23 as u64)) {
            let _ = black_box(23 + 1);
        }
        if flattened_cov.contains_key(&(24 as u64)) {
            let _ = black_box(24 + 1);
        }
        if flattened_cov.contains_key(&(25 as u64)) {
            let _ = black_box(25 + 1);
        }
        if flattened_cov.contains_key(&(26 as u64)) {
            let _ = black_box(26 + 1);
        }
        if flattened_cov.contains_key(&(27 as u64)) {
            let _ = black_box(27 + 1);
        }
        if flattened_cov.contains_key(&(28 as u64)) {
            let _ = black_box(28 + 1);
        }
        if flattened_cov.contains_key(&(29 as u64)) {
            let _ = black_box(29 + 1);
        }
        if flattened_cov.contains_key(&(30 as u64)) {
            let _ = black_box(30 + 1);
        }
        if flattened_cov.contains_key(&(31 as u64)) {
            let _ = black_box(31 + 1);
        }
        if flattened_cov.contains_key(&(32 as u64)) {
            let _ = black_box(32 + 1);
        }
        if flattened_cov.contains_key(&(33 as u64)) {
            let _ = black_box(33 + 1);
        }
        if flattened_cov.contains_key(&(34 as u64)) {
            let _ = black_box(34 + 1);
        }
        if flattened_cov.contains_key(&(35 as u64)) {
            let _ = black_box(35 + 1);
        }
        if flattened_cov.contains_key(&(36 as u64)) {
            let _ = black_box(36 + 1);
        }
        if flattened_cov.contains_key(&(37 as u64)) {
            let _ = black_box(37 + 1);
        }
        if flattened_cov.contains_key(&(38 as u64)) {
            let _ = black_box(38 + 1);
        }
        if flattened_cov.contains_key(&(39 as u64)) {
            let _ = black_box(39 + 1);
        }
        if flattened_cov.contains_key(&(40 as u64)) {
            let _ = black_box(40 + 1);
        }
        if flattened_cov.contains_key(&(41 as u64)) {
            let _ = black_box(41 + 1);
        }
        if flattened_cov.contains_key(&(42 as u64)) {
            let _ = black_box(42 + 1);
        }
        if flattened_cov.contains_key(&(43 as u64)) {
            let _ = black_box(43 + 1);
        }
        if flattened_cov.contains_key(&(44 as u64)) {
            let _ = black_box(44 + 1);
        }
        if flattened_cov.contains_key(&(45 as u64)) {
            let _ = black_box(45 + 1);
        }
        if flattened_cov.contains_key(&(46 as u64)) {
            let _ = black_box(46 + 1);
        }
        if flattened_cov.contains_key(&(47 as u64)) {
            let _ = black_box(47 + 1);
        }
        if flattened_cov.contains_key(&(48 as u64)) {
            let _ = black_box(48 + 1);
        }
        if flattened_cov.contains_key(&(49 as u64)) {
            let _ = black_box(49 + 1);
        }
        if flattened_cov.contains_key(&(50 as u64)) {
            let _ = black_box(50 + 1);
        }
        if flattened_cov.contains_key(&(51 as u64)) {
            let _ = black_box(51 + 1);
        }
        if flattened_cov.contains_key(&(52 as u64)) {
            let _ = black_box(52 + 1);
        }
        if flattened_cov.contains_key(&(53 as u64)) {
            let _ = black_box(53 + 1);
        }
        if flattened_cov.contains_key(&(54 as u64)) {
            let _ = black_box(54 + 1);
        }
        if flattened_cov.contains_key(&(55 as u64)) {
            let _ = black_box(55 + 1);
        }
        if flattened_cov.contains_key(&(56 as u64)) {
            let _ = black_box(56 + 1);
        }
        if flattened_cov.contains_key(&(57 as u64)) {
            let _ = black_box(57 + 1);
        }
        if flattened_cov.contains_key(&(58 as u64)) {
            let _ = black_box(58 + 1);
        }
        if flattened_cov.contains_key(&(59 as u64)) {
            let _ = black_box(59 + 1);
        }
        if flattened_cov.contains_key(&(60 as u64)) {
            let _ = black_box(60 + 1);
        }
        if flattened_cov.contains_key(&(61 as u64)) {
            let _ = black_box(61 + 1);
        }
        if flattened_cov.contains_key(&(62 as u64)) {
            let _ = black_box(62 + 1);
        }
        if flattened_cov.contains_key(&(63 as u64)) {
            let _ = black_box(63 + 1);
        }
        if flattened_cov.contains_key(&(64 as u64)) {
            let _ = black_box(64 + 1);
        }
        if flattened_cov.contains_key(&(65 as u64)) {
            let _ = black_box(65 + 1);
        }
        if flattened_cov.contains_key(&(66 as u64)) {
            let _ = black_box(66 + 1);
        }
        if flattened_cov.contains_key(&(67 as u64)) {
            let _ = black_box(67 + 1);
        }
        if flattened_cov.contains_key(&(68 as u64)) {
            let _ = black_box(68 + 1);
        }
        if flattened_cov.contains_key(&(69 as u64)) {
            let _ = black_box(69 + 1);
        }
        if flattened_cov.contains_key(&(70 as u64)) {
            let _ = black_box(70 + 1);
        }
        if flattened_cov.contains_key(&(71 as u64)) {
            let _ = black_box(71 + 1);
        }
        if flattened_cov.contains_key(&(72 as u64)) {
            let _ = black_box(72 + 1);
        }
        if flattened_cov.contains_key(&(73 as u64)) {
            let _ = black_box(73 + 1);
        }
        if flattened_cov.contains_key(&(74 as u64)) {
            let _ = black_box(74 + 1);
        }
        if flattened_cov.contains_key(&(75 as u64)) {
            let _ = black_box(75 + 1);
        }
        if flattened_cov.contains_key(&(76 as u64)) {
            let _ = black_box(76 + 1);
        }
        if flattened_cov.contains_key(&(77 as u64)) {
            let _ = black_box(77 + 1);
        }
        if flattened_cov.contains_key(&(78 as u64)) {
            let _ = black_box(78 + 1);
        }
        if flattened_cov.contains_key(&(79 as u64)) {
            let _ = black_box(79 + 1);
        }
        if flattened_cov.contains_key(&(80 as u64)) {
            let _ = black_box(80 + 1);
        }
        if flattened_cov.contains_key(&(81 as u64)) {
            let _ = black_box(81 + 1);
        }
        if flattened_cov.contains_key(&(82 as u64)) {
            let _ = black_box(82 + 1);
        }
        if flattened_cov.contains_key(&(83 as u64)) {
            let _ = black_box(83 + 1);
        }
        if flattened_cov.contains_key(&(84 as u64)) {
            let _ = black_box(84 + 1);
        }
        if flattened_cov.contains_key(&(85 as u64)) {
            let _ = black_box(85 + 1);
        }
        if flattened_cov.contains_key(&(86 as u64)) {
            let _ = black_box(86 + 1);
        }
        if flattened_cov.contains_key(&(87 as u64)) {
            let _ = black_box(87 + 1);
        }
        if flattened_cov.contains_key(&(88 as u64)) {
            let _ = black_box(88 + 1);
        }
        if flattened_cov.contains_key(&(89 as u64)) {
            let _ = black_box(89 + 1);
        }
        if flattened_cov.contains_key(&(90 as u64)) {
            let _ = black_box(90 + 1);
        }
        if flattened_cov.contains_key(&(91 as u64)) {
            let _ = black_box(91 + 1);
        }
        if flattened_cov.contains_key(&(92 as u64)) {
            let _ = black_box(92 + 1);
        }
        if flattened_cov.contains_key(&(93 as u64)) {
            let _ = black_box(93 + 1);
        }
        if flattened_cov.contains_key(&(94 as u64)) {
            let _ = black_box(94 + 1);
        }
        if flattened_cov.contains_key(&(95 as u64)) {
            let _ = black_box(95 + 1);
        }
        if flattened_cov.contains_key(&(96 as u64)) {
            let _ = black_box(96 + 1);
        }
        if flattened_cov.contains_key(&(97 as u64)) {
            let _ = black_box(97 + 1);
        }
        if flattened_cov.contains_key(&(98 as u64)) {
            let _ = black_box(98 + 1);
        }
        if flattened_cov.contains_key(&(99 as u64)) {
            let _ = black_box(99 + 1);
        }
        if flattened_cov.contains_key(&(100 as u64)) {
            let _ = black_box(100 + 1);
        }
        if flattened_cov.contains_key(&(101 as u64)) {
            let _ = black_box(101 + 1);
        }
        if flattened_cov.contains_key(&(102 as u64)) {
            let _ = black_box(102 + 1);
        }
        if flattened_cov.contains_key(&(103 as u64)) {
            let _ = black_box(103 + 1);
        }
        if flattened_cov.contains_key(&(104 as u64)) {
            let _ = black_box(104 + 1);
        }
        if flattened_cov.contains_key(&(105 as u64)) {
            let _ = black_box(105 + 1);
        }
        if flattened_cov.contains_key(&(106 as u64)) {
            let _ = black_box(106 + 1);
        }
        if flattened_cov.contains_key(&(107 as u64)) {
            let _ = black_box(107 + 1);
        }
        if flattened_cov.contains_key(&(108 as u64)) {
            let _ = black_box(108 + 1);
        }
        if flattened_cov.contains_key(&(109 as u64)) {
            let _ = black_box(109 + 1);
        }
        if flattened_cov.contains_key(&(110 as u64)) {
            let _ = black_box(110 + 1);
        }
        if flattened_cov.contains_key(&(111 as u64)) {
            let _ = black_box(111 + 1);
        }
        if flattened_cov.contains_key(&(112 as u64)) {
            let _ = black_box(112 + 1);
        }
        if flattened_cov.contains_key(&(113 as u64)) {
            let _ = black_box(113 + 1);
        }
        if flattened_cov.contains_key(&(114 as u64)) {
            let _ = black_box(114 + 1);
        }
        if flattened_cov.contains_key(&(115 as u64)) {
            let _ = black_box(115 + 1);
        }
        if flattened_cov.contains_key(&(116 as u64)) {
            let _ = black_box(116 + 1);
        }
        if flattened_cov.contains_key(&(117 as u64)) {
            let _ = black_box(117 + 1);
        }
        if flattened_cov.contains_key(&(118 as u64)) {
            let _ = black_box(118 + 1);
        }
        if flattened_cov.contains_key(&(119 as u64)) {
            let _ = black_box(119 + 1);
        }
        if flattened_cov.contains_key(&(120 as u64)) {
            let _ = black_box(120 + 1);
        }
        if flattened_cov.contains_key(&(121 as u64)) {
            let _ = black_box(121 + 1);
        }
        if flattened_cov.contains_key(&(122 as u64)) {
            let _ = black_box(122 + 1);
        }
        if flattened_cov.contains_key(&(123 as u64)) {
            let _ = black_box(123 + 1);
        }
        if flattened_cov.contains_key(&(124 as u64)) {
            let _ = black_box(124 + 1);
        }
        if flattened_cov.contains_key(&(125 as u64)) {
            let _ = black_box(125 + 1);
        }
        if flattened_cov.contains_key(&(126 as u64)) {
            let _ = black_box(126 + 1);
        }
        if flattened_cov.contains_key(&(127 as u64)) {
            let _ = black_box(127 + 1);
        }
        if flattened_cov.contains_key(&(128 as u64)) {
            let _ = black_box(128 + 1);
        }
        if flattened_cov.contains_key(&(129 as u64)) {
            let _ = black_box(129 + 1);
        }
        if flattened_cov.contains_key(&(130 as u64)) {
            let _ = black_box(130 + 1);
        }
        if flattened_cov.contains_key(&(131 as u64)) {
            let _ = black_box(131 + 1);
        }
        if flattened_cov.contains_key(&(132 as u64)) {
            let _ = black_box(132 + 1);
        }
        if flattened_cov.contains_key(&(133 as u64)) {
            let _ = black_box(133 + 1);
        }
        if flattened_cov.contains_key(&(134 as u64)) {
            let _ = black_box(134 + 1);
        }
        if flattened_cov.contains_key(&(135 as u64)) {
            let _ = black_box(135 + 1);
        }
        if flattened_cov.contains_key(&(136 as u64)) {
            let _ = black_box(136 + 1);
        }
        if flattened_cov.contains_key(&(137 as u64)) {
            let _ = black_box(137 + 1);
        }
        if flattened_cov.contains_key(&(138 as u64)) {
            let _ = black_box(138 + 1);
        }
        if flattened_cov.contains_key(&(139 as u64)) {
            let _ = black_box(139 + 1);
        }
        if flattened_cov.contains_key(&(140 as u64)) {
            let _ = black_box(140 + 1);
        }
        if flattened_cov.contains_key(&(141 as u64)) {
            let _ = black_box(141 + 1);
        }
        if flattened_cov.contains_key(&(142 as u64)) {
            let _ = black_box(142 + 1);
        }
        if flattened_cov.contains_key(&(143 as u64)) {
            let _ = black_box(143 + 1);
        }
        if flattened_cov.contains_key(&(144 as u64)) {
            let _ = black_box(144 + 1);
        }
        if flattened_cov.contains_key(&(145 as u64)) {
            let _ = black_box(145 + 1);
        }
        if flattened_cov.contains_key(&(146 as u64)) {
            let _ = black_box(146 + 1);
        }
        if flattened_cov.contains_key(&(147 as u64)) {
            let _ = black_box(147 + 1);
        }
        if flattened_cov.contains_key(&(148 as u64)) {
            let _ = black_box(148 + 1);
        }
        if flattened_cov.contains_key(&(149 as u64)) {
            let _ = black_box(149 + 1);
        }
        if flattened_cov.contains_key(&(150 as u64)) {
            let _ = black_box(150 + 1);
        }
        if flattened_cov.contains_key(&(151 as u64)) {
            let _ = black_box(151 + 1);
        }
        if flattened_cov.contains_key(&(152 as u64)) {
            let _ = black_box(152 + 1);
        }
        if flattened_cov.contains_key(&(153 as u64)) {
            let _ = black_box(153 + 1);
        }
        if flattened_cov.contains_key(&(154 as u64)) {
            let _ = black_box(154 + 1);
        }
        if flattened_cov.contains_key(&(155 as u64)) {
            let _ = black_box(155 + 1);
        }
        if flattened_cov.contains_key(&(156 as u64)) {
            let _ = black_box(156 + 1);
        }
        if flattened_cov.contains_key(&(157 as u64)) {
            let _ = black_box(157 + 1);
        }
        if flattened_cov.contains_key(&(158 as u64)) {
            let _ = black_box(158 + 1);
        }
        if flattened_cov.contains_key(&(159 as u64)) {
            let _ = black_box(159 + 1);
        }
        if flattened_cov.contains_key(&(160 as u64)) {
            let _ = black_box(160 + 1);
        }
        if flattened_cov.contains_key(&(161 as u64)) {
            let _ = black_box(161 + 1);
        }
        if flattened_cov.contains_key(&(162 as u64)) {
            let _ = black_box(162 + 1);
        }
        if flattened_cov.contains_key(&(163 as u64)) {
            let _ = black_box(163 + 1);
        }
        if flattened_cov.contains_key(&(164 as u64)) {
            let _ = black_box(164 + 1);
        }
        if flattened_cov.contains_key(&(165 as u64)) {
            let _ = black_box(165 + 1);
        }
        if flattened_cov.contains_key(&(166 as u64)) {
            let _ = black_box(166 + 1);
        }
        if flattened_cov.contains_key(&(167 as u64)) {
            let _ = black_box(167 + 1);
        }
        if flattened_cov.contains_key(&(168 as u64)) {
            let _ = black_box(168 + 1);
        }
        if flattened_cov.contains_key(&(169 as u64)) {
            let _ = black_box(169 + 1);
        }
        if flattened_cov.contains_key(&(170 as u64)) {
            let _ = black_box(170 + 1);
        }
        if flattened_cov.contains_key(&(171 as u64)) {
            let _ = black_box(171 + 1);
        }
        if flattened_cov.contains_key(&(172 as u64)) {
            let _ = black_box(172 + 1);
        }
        if flattened_cov.contains_key(&(173 as u64)) {
            let _ = black_box(173 + 1);
        }
        if flattened_cov.contains_key(&(174 as u64)) {
            let _ = black_box(174 + 1);
        }
        if flattened_cov.contains_key(&(175 as u64)) {
            let _ = black_box(175 + 1);
        }
        if flattened_cov.contains_key(&(176 as u64)) {
            let _ = black_box(176 + 1);
        }
        if flattened_cov.contains_key(&(177 as u64)) {
            let _ = black_box(177 + 1);
        }
        if flattened_cov.contains_key(&(178 as u64)) {
            let _ = black_box(178 + 1);
        }
        if flattened_cov.contains_key(&(179 as u64)) {
            let _ = black_box(179 + 1);
        }
        if flattened_cov.contains_key(&(180 as u64)) {
            let _ = black_box(180 + 1);
        }
        if flattened_cov.contains_key(&(181 as u64)) {
            let _ = black_box(181 + 1);
        }
        if flattened_cov.contains_key(&(182 as u64)) {
            let _ = black_box(182 + 1);
        }
        if flattened_cov.contains_key(&(183 as u64)) {
            let _ = black_box(183 + 1);
        }
        if flattened_cov.contains_key(&(184 as u64)) {
            let _ = black_box(184 + 1);
        }
        if flattened_cov.contains_key(&(185 as u64)) {
            let _ = black_box(185 + 1);
        }
        if flattened_cov.contains_key(&(186 as u64)) {
            let _ = black_box(186 + 1);
        }
        if flattened_cov.contains_key(&(187 as u64)) {
            let _ = black_box(187 + 1);
        }
        if flattened_cov.contains_key(&(188 as u64)) {
            let _ = black_box(188 + 1);
        }
        if flattened_cov.contains_key(&(189 as u64)) {
            let _ = black_box(189 + 1);
        }
        if flattened_cov.contains_key(&(190 as u64)) {
            let _ = black_box(190 + 1);
        }
        if flattened_cov.contains_key(&(191 as u64)) {
            let _ = black_box(191 + 1);
        }
        if flattened_cov.contains_key(&(192 as u64)) {
            let _ = black_box(192 + 1);
        }
        if flattened_cov.contains_key(&(193 as u64)) {
            let _ = black_box(193 + 1);
        }
        if flattened_cov.contains_key(&(194 as u64)) {
            let _ = black_box(194 + 1);
        }
        if flattened_cov.contains_key(&(195 as u64)) {
            let _ = black_box(195 + 1);
        }
        if flattened_cov.contains_key(&(196 as u64)) {
            let _ = black_box(196 + 1);
        }
        if flattened_cov.contains_key(&(197 as u64)) {
            let _ = black_box(197 + 1);
        }
        if flattened_cov.contains_key(&(198 as u64)) {
            let _ = black_box(198 + 1);
        }
        if flattened_cov.contains_key(&(199 as u64)) {
            let _ = black_box(199 + 1);
        }
        if flattened_cov.contains_key(&(200 as u64)) {
            let _ = black_box(200 + 1);
        }
        if flattened_cov.contains_key(&(201 as u64)) {
            let _ = black_box(201 + 1);
        }
        if flattened_cov.contains_key(&(202 as u64)) {
            let _ = black_box(202 + 1);
        }
        if flattened_cov.contains_key(&(203 as u64)) {
            let _ = black_box(203 + 1);
        }
        if flattened_cov.contains_key(&(204 as u64)) {
            let _ = black_box(204 + 1);
        }
        if flattened_cov.contains_key(&(205 as u64)) {
            let _ = black_box(205 + 1);
        }
        if flattened_cov.contains_key(&(206 as u64)) {
            let _ = black_box(206 + 1);
        }
        if flattened_cov.contains_key(&(207 as u64)) {
            let _ = black_box(207 + 1);
        }
        if flattened_cov.contains_key(&(208 as u64)) {
            let _ = black_box(208 + 1);
        }
        if flattened_cov.contains_key(&(209 as u64)) {
            let _ = black_box(209 + 1);
        }
        if flattened_cov.contains_key(&(210 as u64)) {
            let _ = black_box(210 + 1);
        }
        if flattened_cov.contains_key(&(211 as u64)) {
            let _ = black_box(211 + 1);
        }
        if flattened_cov.contains_key(&(212 as u64)) {
            let _ = black_box(212 + 1);
        }
        if flattened_cov.contains_key(&(213 as u64)) {
            let _ = black_box(213 + 1);
        }
        if flattened_cov.contains_key(&(214 as u64)) {
            let _ = black_box(214 + 1);
        }
        if flattened_cov.contains_key(&(215 as u64)) {
            let _ = black_box(215 + 1);
        }
        if flattened_cov.contains_key(&(216 as u64)) {
            let _ = black_box(216 + 1);
        }
        if flattened_cov.contains_key(&(217 as u64)) {
            let _ = black_box(217 + 1);
        }
        if flattened_cov.contains_key(&(218 as u64)) {
            let _ = black_box(218 + 1);
        }
        if flattened_cov.contains_key(&(219 as u64)) {
            let _ = black_box(219 + 1);
        }
        if flattened_cov.contains_key(&(220 as u64)) {
            let _ = black_box(220 + 1);
        }
        if flattened_cov.contains_key(&(221 as u64)) {
            let _ = black_box(221 + 1);
        }
        if flattened_cov.contains_key(&(222 as u64)) {
            let _ = black_box(222 + 1);
        }
        if flattened_cov.contains_key(&(223 as u64)) {
            let _ = black_box(223 + 1);
        }
        if flattened_cov.contains_key(&(224 as u64)) {
            let _ = black_box(224 + 1);
        }
        if flattened_cov.contains_key(&(225 as u64)) {
            let _ = black_box(225 + 1);
        }
        if flattened_cov.contains_key(&(226 as u64)) {
            let _ = black_box(226 + 1);
        }
        if flattened_cov.contains_key(&(227 as u64)) {
            let _ = black_box(227 + 1);
        }
        if flattened_cov.contains_key(&(228 as u64)) {
            let _ = black_box(228 + 1);
        }
        if flattened_cov.contains_key(&(229 as u64)) {
            let _ = black_box(229 + 1);
        }
        if flattened_cov.contains_key(&(230 as u64)) {
            let _ = black_box(230 + 1);
        }
        if flattened_cov.contains_key(&(231 as u64)) {
            let _ = black_box(231 + 1);
        }
        if flattened_cov.contains_key(&(232 as u64)) {
            let _ = black_box(232 + 1);
        }
        if flattened_cov.contains_key(&(233 as u64)) {
            let _ = black_box(233 + 1);
        }
        if flattened_cov.contains_key(&(234 as u64)) {
            let _ = black_box(234 + 1);
        }
        if flattened_cov.contains_key(&(235 as u64)) {
            let _ = black_box(235 + 1);
        }
        if flattened_cov.contains_key(&(236 as u64)) {
            let _ = black_box(236 + 1);
        }
        if flattened_cov.contains_key(&(237 as u64)) {
            let _ = black_box(237 + 1);
        }
        if flattened_cov.contains_key(&(238 as u64)) {
            let _ = black_box(238 + 1);
        }
        if flattened_cov.contains_key(&(239 as u64)) {
            let _ = black_box(239 + 1);
        }
        if flattened_cov.contains_key(&(240 as u64)) {
            let _ = black_box(240 + 1);
        }
        if flattened_cov.contains_key(&(241 as u64)) {
            let _ = black_box(241 + 1);
        }
        if flattened_cov.contains_key(&(242 as u64)) {
            let _ = black_box(242 + 1);
        }
        if flattened_cov.contains_key(&(243 as u64)) {
            let _ = black_box(243 + 1);
        }
        if flattened_cov.contains_key(&(244 as u64)) {
            let _ = black_box(244 + 1);
        }
        if flattened_cov.contains_key(&(245 as u64)) {
            let _ = black_box(245 + 1);
        }
        if flattened_cov.contains_key(&(246 as u64)) {
            let _ = black_box(246 + 1);
        }
        if flattened_cov.contains_key(&(247 as u64)) {
            let _ = black_box(247 + 1);
        }
        if flattened_cov.contains_key(&(248 as u64)) {
            let _ = black_box(248 + 1);
        }
        if flattened_cov.contains_key(&(249 as u64)) {
            let _ = black_box(249 + 1);
        }
        if flattened_cov.contains_key(&(250 as u64)) {
            let _ = black_box(250 + 1);
        }
        if flattened_cov.contains_key(&(251 as u64)) {
            let _ = black_box(251 + 1);
        }
        if flattened_cov.contains_key(&(252 as u64)) {
            let _ = black_box(252 + 1);
        }
        if flattened_cov.contains_key(&(253 as u64)) {
            let _ = black_box(253 + 1);
        }
        if flattened_cov.contains_key(&(254 as u64)) {
            let _ = black_box(254 + 1);
        }
        if flattened_cov.contains_key(&(255 as u64)) {
            let _ = black_box(255 + 1);
        }
        if flattened_cov.contains_key(&(256 as u64)) {
            let _ = black_box(256 + 1);
        }
        if flattened_cov.contains_key(&(257 as u64)) {
            let _ = black_box(257 + 1);
        }
        if flattened_cov.contains_key(&(258 as u64)) {
            let _ = black_box(258 + 1);
        }
        if flattened_cov.contains_key(&(259 as u64)) {
            let _ = black_box(259 + 1);
        }
        if flattened_cov.contains_key(&(260 as u64)) {
            let _ = black_box(260 + 1);
        }
        if flattened_cov.contains_key(&(261 as u64)) {
            let _ = black_box(261 + 1);
        }
        if flattened_cov.contains_key(&(262 as u64)) {
            let _ = black_box(262 + 1);
        }
        if flattened_cov.contains_key(&(263 as u64)) {
            let _ = black_box(263 + 1);
        }
        if flattened_cov.contains_key(&(264 as u64)) {
            let _ = black_box(264 + 1);
        }
        if flattened_cov.contains_key(&(265 as u64)) {
            let _ = black_box(265 + 1);
        }
        if flattened_cov.contains_key(&(266 as u64)) {
            let _ = black_box(266 + 1);
        }
        if flattened_cov.contains_key(&(267 as u64)) {
            let _ = black_box(267 + 1);
        }
        if flattened_cov.contains_key(&(268 as u64)) {
            let _ = black_box(268 + 1);
        }
        if flattened_cov.contains_key(&(269 as u64)) {
            let _ = black_box(269 + 1);
        }
        if flattened_cov.contains_key(&(270 as u64)) {
            let _ = black_box(270 + 1);
        }
        if flattened_cov.contains_key(&(271 as u64)) {
            let _ = black_box(271 + 1);
        }
        if flattened_cov.contains_key(&(272 as u64)) {
            let _ = black_box(272 + 1);
        }
        if flattened_cov.contains_key(&(273 as u64)) {
            let _ = black_box(273 + 1);
        }
        if flattened_cov.contains_key(&(274 as u64)) {
            let _ = black_box(274 + 1);
        }
        if flattened_cov.contains_key(&(275 as u64)) {
            let _ = black_box(275 + 1);
        }
        if flattened_cov.contains_key(&(276 as u64)) {
            let _ = black_box(276 + 1);
        }
        if flattened_cov.contains_key(&(277 as u64)) {
            let _ = black_box(277 + 1);
        }
        if flattened_cov.contains_key(&(278 as u64)) {
            let _ = black_box(278 + 1);
        }
        if flattened_cov.contains_key(&(279 as u64)) {
            let _ = black_box(279 + 1);
        }
        if flattened_cov.contains_key(&(280 as u64)) {
            let _ = black_box(280 + 1);
        }
        if flattened_cov.contains_key(&(281 as u64)) {
            let _ = black_box(281 + 1);
        }
        if flattened_cov.contains_key(&(282 as u64)) {
            let _ = black_box(282 + 1);
        }
        if flattened_cov.contains_key(&(283 as u64)) {
            let _ = black_box(283 + 1);
        }
        if flattened_cov.contains_key(&(284 as u64)) {
            let _ = black_box(284 + 1);
        }
        if flattened_cov.contains_key(&(285 as u64)) {
            let _ = black_box(285 + 1);
        }
        if flattened_cov.contains_key(&(286 as u64)) {
            let _ = black_box(286 + 1);
        }
        if flattened_cov.contains_key(&(287 as u64)) {
            let _ = black_box(287 + 1);
        }
        if flattened_cov.contains_key(&(288 as u64)) {
            let _ = black_box(288 + 1);
        }
        if flattened_cov.contains_key(&(289 as u64)) {
            let _ = black_box(289 + 1);
        }
        if flattened_cov.contains_key(&(290 as u64)) {
            let _ = black_box(290 + 1);
        }
        if flattened_cov.contains_key(&(291 as u64)) {
            let _ = black_box(291 + 1);
        }
        if flattened_cov.contains_key(&(292 as u64)) {
            let _ = black_box(292 + 1);
        }
        if flattened_cov.contains_key(&(293 as u64)) {
            let _ = black_box(293 + 1);
        }
        if flattened_cov.contains_key(&(294 as u64)) {
            let _ = black_box(294 + 1);
        }
        if flattened_cov.contains_key(&(295 as u64)) {
            let _ = black_box(295 + 1);
        }
        if flattened_cov.contains_key(&(296 as u64)) {
            let _ = black_box(296 + 1);
        }
        if flattened_cov.contains_key(&(297 as u64)) {
            let _ = black_box(297 + 1);
        }
        if flattened_cov.contains_key(&(298 as u64)) {
            let _ = black_box(298 + 1);
        }
        if flattened_cov.contains_key(&(299 as u64)) {
            let _ = black_box(299 + 1);
        }
        if flattened_cov.contains_key(&(300 as u64)) {
            let _ = black_box(300 + 1);
        }
        if flattened_cov.contains_key(&(301 as u64)) {
            let _ = black_box(301 + 1);
        }
        if flattened_cov.contains_key(&(302 as u64)) {
            let _ = black_box(302 + 1);
        }
        if flattened_cov.contains_key(&(303 as u64)) {
            let _ = black_box(303 + 1);
        }
        if flattened_cov.contains_key(&(304 as u64)) {
            let _ = black_box(304 + 1);
        }
        if flattened_cov.contains_key(&(305 as u64)) {
            let _ = black_box(305 + 1);
        }
        if flattened_cov.contains_key(&(306 as u64)) {
            let _ = black_box(306 + 1);
        }
        if flattened_cov.contains_key(&(307 as u64)) {
            let _ = black_box(307 + 1);
        }
        if flattened_cov.contains_key(&(308 as u64)) {
            let _ = black_box(308 + 1);
        }
        if flattened_cov.contains_key(&(309 as u64)) {
            let _ = black_box(309 + 1);
        }
        if flattened_cov.contains_key(&(310 as u64)) {
            let _ = black_box(310 + 1);
        }
        if flattened_cov.contains_key(&(311 as u64)) {
            let _ = black_box(311 + 1);
        }
        if flattened_cov.contains_key(&(312 as u64)) {
            let _ = black_box(312 + 1);
        }
        if flattened_cov.contains_key(&(313 as u64)) {
            let _ = black_box(313 + 1);
        }
        if flattened_cov.contains_key(&(314 as u64)) {
            let _ = black_box(314 + 1);
        }
        if flattened_cov.contains_key(&(315 as u64)) {
            let _ = black_box(315 + 1);
        }
        if flattened_cov.contains_key(&(316 as u64)) {
            let _ = black_box(316 + 1);
        }
        if flattened_cov.contains_key(&(317 as u64)) {
            let _ = black_box(317 + 1);
        }
        if flattened_cov.contains_key(&(318 as u64)) {
            let _ = black_box(318 + 1);
        }
        if flattened_cov.contains_key(&(319 as u64)) {
            let _ = black_box(319 + 1);
        }
        if flattened_cov.contains_key(&(320 as u64)) {
            let _ = black_box(320 + 1);
        }
        if flattened_cov.contains_key(&(321 as u64)) {
            let _ = black_box(321 + 1);
        }
        if flattened_cov.contains_key(&(322 as u64)) {
            let _ = black_box(322 + 1);
        }
        if flattened_cov.contains_key(&(323 as u64)) {
            let _ = black_box(323 + 1);
        }
        if flattened_cov.contains_key(&(324 as u64)) {
            let _ = black_box(324 + 1);
        }
        if flattened_cov.contains_key(&(325 as u64)) {
            let _ = black_box(325 + 1);
        }
        if flattened_cov.contains_key(&(326 as u64)) {
            let _ = black_box(326 + 1);
        }
        if flattened_cov.contains_key(&(327 as u64)) {
            let _ = black_box(327 + 1);
        }
        if flattened_cov.contains_key(&(328 as u64)) {
            let _ = black_box(328 + 1);
        }
        if flattened_cov.contains_key(&(329 as u64)) {
            let _ = black_box(329 + 1);
        }
        if flattened_cov.contains_key(&(330 as u64)) {
            let _ = black_box(330 + 1);
        }
        if flattened_cov.contains_key(&(331 as u64)) {
            let _ = black_box(331 + 1);
        }
        if flattened_cov.contains_key(&(332 as u64)) {
            let _ = black_box(332 + 1);
        }
        if flattened_cov.contains_key(&(333 as u64)) {
            let _ = black_box(333 + 1);
        }
        if flattened_cov.contains_key(&(334 as u64)) {
            let _ = black_box(334 + 1);
        }
        if flattened_cov.contains_key(&(335 as u64)) {
            let _ = black_box(335 + 1);
        }
        if flattened_cov.contains_key(&(336 as u64)) {
            let _ = black_box(336 + 1);
        }
        if flattened_cov.contains_key(&(337 as u64)) {
            let _ = black_box(337 + 1);
        }
        if flattened_cov.contains_key(&(338 as u64)) {
            let _ = black_box(338 + 1);
        }
        if flattened_cov.contains_key(&(339 as u64)) {
            let _ = black_box(339 + 1);
        }
        if flattened_cov.contains_key(&(340 as u64)) {
            let _ = black_box(340 + 1);
        }
        if flattened_cov.contains_key(&(341 as u64)) {
            let _ = black_box(341 + 1);
        }
        if flattened_cov.contains_key(&(342 as u64)) {
            let _ = black_box(342 + 1);
        }
        if flattened_cov.contains_key(&(343 as u64)) {
            let _ = black_box(343 + 1);
        }
        if flattened_cov.contains_key(&(344 as u64)) {
            let _ = black_box(344 + 1);
        }
        if flattened_cov.contains_key(&(345 as u64)) {
            let _ = black_box(345 + 1);
        }
        if flattened_cov.contains_key(&(346 as u64)) {
            let _ = black_box(346 + 1);
        }
        if flattened_cov.contains_key(&(347 as u64)) {
            let _ = black_box(347 + 1);
        }
        if flattened_cov.contains_key(&(348 as u64)) {
            let _ = black_box(348 + 1);
        }
        if flattened_cov.contains_key(&(349 as u64)) {
            let _ = black_box(349 + 1);
        }
        if flattened_cov.contains_key(&(350 as u64)) {
            let _ = black_box(350 + 1);
        }
        if flattened_cov.contains_key(&(351 as u64)) {
            let _ = black_box(351 + 1);
        }
        if flattened_cov.contains_key(&(352 as u64)) {
            let _ = black_box(352 + 1);
        }
        if flattened_cov.contains_key(&(353 as u64)) {
            let _ = black_box(353 + 1);
        }
        if flattened_cov.contains_key(&(354 as u64)) {
            let _ = black_box(354 + 1);
        }
        if flattened_cov.contains_key(&(355 as u64)) {
            let _ = black_box(355 + 1);
        }
        if flattened_cov.contains_key(&(356 as u64)) {
            let _ = black_box(356 + 1);
        }
        if flattened_cov.contains_key(&(357 as u64)) {
            let _ = black_box(357 + 1);
        }
        if flattened_cov.contains_key(&(358 as u64)) {
            let _ = black_box(358 + 1);
        }
        if flattened_cov.contains_key(&(359 as u64)) {
            let _ = black_box(359 + 1);
        }
        if flattened_cov.contains_key(&(360 as u64)) {
            let _ = black_box(360 + 1);
        }
        if flattened_cov.contains_key(&(361 as u64)) {
            let _ = black_box(361 + 1);
        }
        if flattened_cov.contains_key(&(362 as u64)) {
            let _ = black_box(362 + 1);
        }
        if flattened_cov.contains_key(&(363 as u64)) {
            let _ = black_box(363 + 1);
        }
        if flattened_cov.contains_key(&(364 as u64)) {
            let _ = black_box(364 + 1);
        }
        if flattened_cov.contains_key(&(365 as u64)) {
            let _ = black_box(365 + 1);
        }
        if flattened_cov.contains_key(&(366 as u64)) {
            let _ = black_box(366 + 1);
        }
        if flattened_cov.contains_key(&(367 as u64)) {
            let _ = black_box(367 + 1);
        }
        if flattened_cov.contains_key(&(368 as u64)) {
            let _ = black_box(368 + 1);
        }
        if flattened_cov.contains_key(&(369 as u64)) {
            let _ = black_box(369 + 1);
        }
        if flattened_cov.contains_key(&(370 as u64)) {
            let _ = black_box(370 + 1);
        }
        if flattened_cov.contains_key(&(371 as u64)) {
            let _ = black_box(371 + 1);
        }
        if flattened_cov.contains_key(&(372 as u64)) {
            let _ = black_box(372 + 1);
        }
        if flattened_cov.contains_key(&(373 as u64)) {
            let _ = black_box(373 + 1);
        }
        if flattened_cov.contains_key(&(374 as u64)) {
            let _ = black_box(374 + 1);
        }
        if flattened_cov.contains_key(&(375 as u64)) {
            let _ = black_box(375 + 1);
        }
        if flattened_cov.contains_key(&(376 as u64)) {
            let _ = black_box(376 + 1);
        }
        if flattened_cov.contains_key(&(377 as u64)) {
            let _ = black_box(377 + 1);
        }
        if flattened_cov.contains_key(&(378 as u64)) {
            let _ = black_box(378 + 1);
        }
        if flattened_cov.contains_key(&(379 as u64)) {
            let _ = black_box(379 + 1);
        }
        if flattened_cov.contains_key(&(380 as u64)) {
            let _ = black_box(380 + 1);
        }
        if flattened_cov.contains_key(&(381 as u64)) {
            let _ = black_box(381 + 1);
        }
        if flattened_cov.contains_key(&(382 as u64)) {
            let _ = black_box(382 + 1);
        }
        if flattened_cov.contains_key(&(383 as u64)) {
            let _ = black_box(383 + 1);
        }
        if flattened_cov.contains_key(&(384 as u64)) {
            let _ = black_box(384 + 1);
        }
        if flattened_cov.contains_key(&(385 as u64)) {
            let _ = black_box(385 + 1);
        }
        if flattened_cov.contains_key(&(386 as u64)) {
            let _ = black_box(386 + 1);
        }
        if flattened_cov.contains_key(&(387 as u64)) {
            let _ = black_box(387 + 1);
        }
        if flattened_cov.contains_key(&(388 as u64)) {
            let _ = black_box(388 + 1);
        }
        if flattened_cov.contains_key(&(389 as u64)) {
            let _ = black_box(389 + 1);
        }
        if flattened_cov.contains_key(&(390 as u64)) {
            let _ = black_box(390 + 1);
        }
        if flattened_cov.contains_key(&(391 as u64)) {
            let _ = black_box(391 + 1);
        }
        if flattened_cov.contains_key(&(392 as u64)) {
            let _ = black_box(392 + 1);
        }
        if flattened_cov.contains_key(&(393 as u64)) {
            let _ = black_box(393 + 1);
        }
        if flattened_cov.contains_key(&(394 as u64)) {
            let _ = black_box(394 + 1);
        }
        if flattened_cov.contains_key(&(395 as u64)) {
            let _ = black_box(395 + 1);
        }
        if flattened_cov.contains_key(&(396 as u64)) {
            let _ = black_box(396 + 1);
        }
        if flattened_cov.contains_key(&(397 as u64)) {
            let _ = black_box(397 + 1);
        }
        if flattened_cov.contains_key(&(398 as u64)) {
            let _ = black_box(398 + 1);
        }
        if flattened_cov.contains_key(&(399 as u64)) {
            let _ = black_box(399 + 1);
        }
        if flattened_cov.contains_key(&(400 as u64)) {
            let _ = black_box(400 + 1);
        }
        if flattened_cov.contains_key(&(401 as u64)) {
            let _ = black_box(401 + 1);
        }
        if flattened_cov.contains_key(&(402 as u64)) {
            let _ = black_box(402 + 1);
        }
        if flattened_cov.contains_key(&(403 as u64)) {
            let _ = black_box(403 + 1);
        }
        if flattened_cov.contains_key(&(404 as u64)) {
            let _ = black_box(404 + 1);
        }
        if flattened_cov.contains_key(&(405 as u64)) {
            let _ = black_box(405 + 1);
        }
        if flattened_cov.contains_key(&(406 as u64)) {
            let _ = black_box(406 + 1);
        }
        if flattened_cov.contains_key(&(407 as u64)) {
            let _ = black_box(407 + 1);
        }
        if flattened_cov.contains_key(&(408 as u64)) {
            let _ = black_box(408 + 1);
        }
        if flattened_cov.contains_key(&(409 as u64)) {
            let _ = black_box(409 + 1);
        }
        if flattened_cov.contains_key(&(410 as u64)) {
            let _ = black_box(410 + 1);
        }
        if flattened_cov.contains_key(&(411 as u64)) {
            let _ = black_box(411 + 1);
        }
        if flattened_cov.contains_key(&(412 as u64)) {
            let _ = black_box(412 + 1);
        }
        if flattened_cov.contains_key(&(413 as u64)) {
            let _ = black_box(413 + 1);
        }
        if flattened_cov.contains_key(&(414 as u64)) {
            let _ = black_box(414 + 1);
        }
        if flattened_cov.contains_key(&(415 as u64)) {
            let _ = black_box(415 + 1);
        }
        if flattened_cov.contains_key(&(416 as u64)) {
            let _ = black_box(416 + 1);
        }
        if flattened_cov.contains_key(&(417 as u64)) {
            let _ = black_box(417 + 1);
        }
        if flattened_cov.contains_key(&(418 as u64)) {
            let _ = black_box(418 + 1);
        }
        if flattened_cov.contains_key(&(419 as u64)) {
            let _ = black_box(419 + 1);
        }
        if flattened_cov.contains_key(&(420 as u64)) {
            let _ = black_box(420 + 1);
        }
        if flattened_cov.contains_key(&(421 as u64)) {
            let _ = black_box(421 + 1);
        }
        if flattened_cov.contains_key(&(422 as u64)) {
            let _ = black_box(422 + 1);
        }
        if flattened_cov.contains_key(&(423 as u64)) {
            let _ = black_box(423 + 1);
        }
        if flattened_cov.contains_key(&(424 as u64)) {
            let _ = black_box(424 + 1);
        }
        if flattened_cov.contains_key(&(425 as u64)) {
            let _ = black_box(425 + 1);
        }
        if flattened_cov.contains_key(&(426 as u64)) {
            let _ = black_box(426 + 1);
        }
        if flattened_cov.contains_key(&(427 as u64)) {
            let _ = black_box(427 + 1);
        }
        if flattened_cov.contains_key(&(428 as u64)) {
            let _ = black_box(428 + 1);
        }
        if flattened_cov.contains_key(&(429 as u64)) {
            let _ = black_box(429 + 1);
        }
        if flattened_cov.contains_key(&(430 as u64)) {
            let _ = black_box(430 + 1);
        }
        if flattened_cov.contains_key(&(431 as u64)) {
            let _ = black_box(431 + 1);
        }
        if flattened_cov.contains_key(&(432 as u64)) {
            let _ = black_box(432 + 1);
        }
        if flattened_cov.contains_key(&(433 as u64)) {
            let _ = black_box(433 + 1);
        }
        if flattened_cov.contains_key(&(434 as u64)) {
            let _ = black_box(434 + 1);
        }
        if flattened_cov.contains_key(&(435 as u64)) {
            let _ = black_box(435 + 1);
        }
        if flattened_cov.contains_key(&(436 as u64)) {
            let _ = black_box(436 + 1);
        }
        if flattened_cov.contains_key(&(437 as u64)) {
            let _ = black_box(437 + 1);
        }
        if flattened_cov.contains_key(&(438 as u64)) {
            let _ = black_box(438 + 1);
        }
        if flattened_cov.contains_key(&(439 as u64)) {
            let _ = black_box(439 + 1);
        }
        if flattened_cov.contains_key(&(440 as u64)) {
            let _ = black_box(440 + 1);
        }
        if flattened_cov.contains_key(&(441 as u64)) {
            let _ = black_box(441 + 1);
        }
        if flattened_cov.contains_key(&(442 as u64)) {
            let _ = black_box(442 + 1);
        }
        if flattened_cov.contains_key(&(443 as u64)) {
            let _ = black_box(443 + 1);
        }
        if flattened_cov.contains_key(&(444 as u64)) {
            let _ = black_box(444 + 1);
        }
        if flattened_cov.contains_key(&(445 as u64)) {
            let _ = black_box(445 + 1);
        }
        if flattened_cov.contains_key(&(446 as u64)) {
            let _ = black_box(446 + 1);
        }
        if flattened_cov.contains_key(&(447 as u64)) {
            let _ = black_box(447 + 1);
        }
        if flattened_cov.contains_key(&(448 as u64)) {
            let _ = black_box(448 + 1);
        }
        if flattened_cov.contains_key(&(449 as u64)) {
            let _ = black_box(449 + 1);
        }
        if flattened_cov.contains_key(&(450 as u64)) {
            let _ = black_box(450 + 1);
        }
        if flattened_cov.contains_key(&(451 as u64)) {
            let _ = black_box(451 + 1);
        }
        if flattened_cov.contains_key(&(452 as u64)) {
            let _ = black_box(452 + 1);
        }
        if flattened_cov.contains_key(&(453 as u64)) {
            let _ = black_box(453 + 1);
        }
        if flattened_cov.contains_key(&(454 as u64)) {
            let _ = black_box(454 + 1);
        }
        if flattened_cov.contains_key(&(455 as u64)) {
            let _ = black_box(455 + 1);
        }
        if flattened_cov.contains_key(&(456 as u64)) {
            let _ = black_box(456 + 1);
        }
        if flattened_cov.contains_key(&(457 as u64)) {
            let _ = black_box(457 + 1);
        }
        if flattened_cov.contains_key(&(458 as u64)) {
            let _ = black_box(458 + 1);
        }
        if flattened_cov.contains_key(&(459 as u64)) {
            let _ = black_box(459 + 1);
        }
        if flattened_cov.contains_key(&(460 as u64)) {
            let _ = black_box(460 + 1);
        }
        if flattened_cov.contains_key(&(461 as u64)) {
            let _ = black_box(461 + 1);
        }
        if flattened_cov.contains_key(&(462 as u64)) {
            let _ = black_box(462 + 1);
        }
        if flattened_cov.contains_key(&(463 as u64)) {
            let _ = black_box(463 + 1);
        }
        if flattened_cov.contains_key(&(464 as u64)) {
            let _ = black_box(464 + 1);
        }
        if flattened_cov.contains_key(&(465 as u64)) {
            let _ = black_box(465 + 1);
        }
        if flattened_cov.contains_key(&(466 as u64)) {
            let _ = black_box(466 + 1);
        }
        if flattened_cov.contains_key(&(467 as u64)) {
            let _ = black_box(467 + 1);
        }
        if flattened_cov.contains_key(&(468 as u64)) {
            let _ = black_box(468 + 1);
        }
        if flattened_cov.contains_key(&(469 as u64)) {
            let _ = black_box(469 + 1);
        }
        if flattened_cov.contains_key(&(470 as u64)) {
            let _ = black_box(470 + 1);
        }
        if flattened_cov.contains_key(&(471 as u64)) {
            let _ = black_box(471 + 1);
        }
        if flattened_cov.contains_key(&(472 as u64)) {
            let _ = black_box(472 + 1);
        }
        if flattened_cov.contains_key(&(473 as u64)) {
            let _ = black_box(473 + 1);
        }
        if flattened_cov.contains_key(&(474 as u64)) {
            let _ = black_box(474 + 1);
        }
        if flattened_cov.contains_key(&(475 as u64)) {
            let _ = black_box(475 + 1);
        }
        if flattened_cov.contains_key(&(476 as u64)) {
            let _ = black_box(476 + 1);
        }
        if flattened_cov.contains_key(&(477 as u64)) {
            let _ = black_box(477 + 1);
        }
        if flattened_cov.contains_key(&(478 as u64)) {
            let _ = black_box(478 + 1);
        }
        if flattened_cov.contains_key(&(479 as u64)) {
            let _ = black_box(479 + 1);
        }
        if flattened_cov.contains_key(&(480 as u64)) {
            let _ = black_box(480 + 1);
        }
        if flattened_cov.contains_key(&(481 as u64)) {
            let _ = black_box(481 + 1);
        }
        if flattened_cov.contains_key(&(482 as u64)) {
            let _ = black_box(482 + 1);
        }
        if flattened_cov.contains_key(&(483 as u64)) {
            let _ = black_box(483 + 1);
        }
        if flattened_cov.contains_key(&(484 as u64)) {
            let _ = black_box(484 + 1);
        }
        if flattened_cov.contains_key(&(485 as u64)) {
            let _ = black_box(485 + 1);
        }
        if flattened_cov.contains_key(&(486 as u64)) {
            let _ = black_box(486 + 1);
        }
        if flattened_cov.contains_key(&(487 as u64)) {
            let _ = black_box(487 + 1);
        }
        if flattened_cov.contains_key(&(488 as u64)) {
            let _ = black_box(488 + 1);
        }
        if flattened_cov.contains_key(&(489 as u64)) {
            let _ = black_box(489 + 1);
        }
        if flattened_cov.contains_key(&(490 as u64)) {
            let _ = black_box(490 + 1);
        }
        if flattened_cov.contains_key(&(491 as u64)) {
            let _ = black_box(491 + 1);
        }
        if flattened_cov.contains_key(&(492 as u64)) {
            let _ = black_box(492 + 1);
        }
        if flattened_cov.contains_key(&(493 as u64)) {
            let _ = black_box(493 + 1);
        }
        if flattened_cov.contains_key(&(494 as u64)) {
            let _ = black_box(494 + 1);
        }
        if flattened_cov.contains_key(&(495 as u64)) {
            let _ = black_box(495 + 1);
        }
        if flattened_cov.contains_key(&(496 as u64)) {
            let _ = black_box(496 + 1);
        }
        if flattened_cov.contains_key(&(497 as u64)) {
            let _ = black_box(497 + 1);
        }
        if flattened_cov.contains_key(&(498 as u64)) {
            let _ = black_box(498 + 1);
        }
        if flattened_cov.contains_key(&(499 as u64)) {
            let _ = black_box(499 + 1);
        }
        if flattened_cov.contains_key(&(500 as u64)) {
            let _ = black_box(500 + 1);
        }
        if flattened_cov.contains_key(&(501 as u64)) {
            let _ = black_box(501 + 1);
        }
        if flattened_cov.contains_key(&(502 as u64)) {
            let _ = black_box(502 + 1);
        }
        if flattened_cov.contains_key(&(503 as u64)) {
            let _ = black_box(503 + 1);
        }
        if flattened_cov.contains_key(&(504 as u64)) {
            let _ = black_box(504 + 1);
        }
        if flattened_cov.contains_key(&(505 as u64)) {
            let _ = black_box(505 + 1);
        }
        if flattened_cov.contains_key(&(506 as u64)) {
            let _ = black_box(506 + 1);
        }
        if flattened_cov.contains_key(&(507 as u64)) {
            let _ = black_box(507 + 1);
        }
        if flattened_cov.contains_key(&(508 as u64)) {
            let _ = black_box(508 + 1);
        }
        if flattened_cov.contains_key(&(509 as u64)) {
            let _ = black_box(509 + 1);
        }
        if flattened_cov.contains_key(&(510 as u64)) {
            let _ = black_box(510 + 1);
        }
        if flattened_cov.contains_key(&(511 as u64)) {
            let _ = black_box(511 + 1);
        }
        if flattened_cov.contains_key(&(512 as u64)) {
            let _ = black_box(512 + 1);
        }
        if flattened_cov.contains_key(&(513 as u64)) {
            let _ = black_box(513 + 1);
        }
        if flattened_cov.contains_key(&(514 as u64)) {
            let _ = black_box(514 + 1);
        }
        if flattened_cov.contains_key(&(515 as u64)) {
            let _ = black_box(515 + 1);
        }
        if flattened_cov.contains_key(&(516 as u64)) {
            let _ = black_box(516 + 1);
        }
        if flattened_cov.contains_key(&(517 as u64)) {
            let _ = black_box(517 + 1);
        }
        if flattened_cov.contains_key(&(518 as u64)) {
            let _ = black_box(518 + 1);
        }
        if flattened_cov.contains_key(&(519 as u64)) {
            let _ = black_box(519 + 1);
        }
        if flattened_cov.contains_key(&(520 as u64)) {
            let _ = black_box(520 + 1);
        }
        if flattened_cov.contains_key(&(521 as u64)) {
            let _ = black_box(521 + 1);
        }
        if flattened_cov.contains_key(&(522 as u64)) {
            let _ = black_box(522 + 1);
        }
        if flattened_cov.contains_key(&(523 as u64)) {
            let _ = black_box(523 + 1);
        }
        if flattened_cov.contains_key(&(524 as u64)) {
            let _ = black_box(524 + 1);
        }
        if flattened_cov.contains_key(&(525 as u64)) {
            let _ = black_box(525 + 1);
        }
        if flattened_cov.contains_key(&(526 as u64)) {
            let _ = black_box(526 + 1);
        }
        if flattened_cov.contains_key(&(527 as u64)) {
            let _ = black_box(527 + 1);
        }
        if flattened_cov.contains_key(&(528 as u64)) {
            let _ = black_box(528 + 1);
        }
        if flattened_cov.contains_key(&(529 as u64)) {
            let _ = black_box(529 + 1);
        }
        if flattened_cov.contains_key(&(530 as u64)) {
            let _ = black_box(530 + 1);
        }
        if flattened_cov.contains_key(&(531 as u64)) {
            let _ = black_box(531 + 1);
        }
        if flattened_cov.contains_key(&(532 as u64)) {
            let _ = black_box(532 + 1);
        }
        if flattened_cov.contains_key(&(533 as u64)) {
            let _ = black_box(533 + 1);
        }
        if flattened_cov.contains_key(&(534 as u64)) {
            let _ = black_box(534 + 1);
        }
        if flattened_cov.contains_key(&(535 as u64)) {
            let _ = black_box(535 + 1);
        }
        if flattened_cov.contains_key(&(536 as u64)) {
            let _ = black_box(536 + 1);
        }
        if flattened_cov.contains_key(&(537 as u64)) {
            let _ = black_box(537 + 1);
        }
        if flattened_cov.contains_key(&(538 as u64)) {
            let _ = black_box(538 + 1);
        }
        if flattened_cov.contains_key(&(539 as u64)) {
            let _ = black_box(539 + 1);
        }
        if flattened_cov.contains_key(&(540 as u64)) {
            let _ = black_box(540 + 1);
        }
        if flattened_cov.contains_key(&(541 as u64)) {
            let _ = black_box(541 + 1);
        }
        if flattened_cov.contains_key(&(542 as u64)) {
            let _ = black_box(542 + 1);
        }
        if flattened_cov.contains_key(&(543 as u64)) {
            let _ = black_box(543 + 1);
        }
        if flattened_cov.contains_key(&(544 as u64)) {
            let _ = black_box(544 + 1);
        }
        if flattened_cov.contains_key(&(545 as u64)) {
            let _ = black_box(545 + 1);
        }
        if flattened_cov.contains_key(&(546 as u64)) {
            let _ = black_box(546 + 1);
        }
        if flattened_cov.contains_key(&(547 as u64)) {
            let _ = black_box(547 + 1);
        }
        if flattened_cov.contains_key(&(548 as u64)) {
            let _ = black_box(548 + 1);
        }
        if flattened_cov.contains_key(&(549 as u64)) {
            let _ = black_box(549 + 1);
        }
        if flattened_cov.contains_key(&(550 as u64)) {
            let _ = black_box(550 + 1);
        }
        if flattened_cov.contains_key(&(551 as u64)) {
            let _ = black_box(551 + 1);
        }
        if flattened_cov.contains_key(&(552 as u64)) {
            let _ = black_box(552 + 1);
        }
        if flattened_cov.contains_key(&(553 as u64)) {
            let _ = black_box(553 + 1);
        }
        if flattened_cov.contains_key(&(554 as u64)) {
            let _ = black_box(554 + 1);
        }
        if flattened_cov.contains_key(&(555 as u64)) {
            let _ = black_box(555 + 1);
        }
        if flattened_cov.contains_key(&(556 as u64)) {
            let _ = black_box(556 + 1);
        }
        if flattened_cov.contains_key(&(557 as u64)) {
            let _ = black_box(557 + 1);
        }
        if flattened_cov.contains_key(&(558 as u64)) {
            let _ = black_box(558 + 1);
        }
        if flattened_cov.contains_key(&(559 as u64)) {
            let _ = black_box(559 + 1);
        }
        if flattened_cov.contains_key(&(560 as u64)) {
            let _ = black_box(560 + 1);
        }
        if flattened_cov.contains_key(&(561 as u64)) {
            let _ = black_box(561 + 1);
        }
        if flattened_cov.contains_key(&(562 as u64)) {
            let _ = black_box(562 + 1);
        }
        if flattened_cov.contains_key(&(563 as u64)) {
            let _ = black_box(563 + 1);
        }
        if flattened_cov.contains_key(&(564 as u64)) {
            let _ = black_box(564 + 1);
        }
        if flattened_cov.contains_key(&(565 as u64)) {
            let _ = black_box(565 + 1);
        }
        if flattened_cov.contains_key(&(566 as u64)) {
            let _ = black_box(566 + 1);
        }
        if flattened_cov.contains_key(&(567 as u64)) {
            let _ = black_box(567 + 1);
        }
        if flattened_cov.contains_key(&(568 as u64)) {
            let _ = black_box(568 + 1);
        }
        if flattened_cov.contains_key(&(569 as u64)) {
            let _ = black_box(569 + 1);
        }
        if flattened_cov.contains_key(&(570 as u64)) {
            let _ = black_box(570 + 1);
        }
        if flattened_cov.contains_key(&(571 as u64)) {
            let _ = black_box(571 + 1);
        }
        if flattened_cov.contains_key(&(572 as u64)) {
            let _ = black_box(572 + 1);
        }
        if flattened_cov.contains_key(&(573 as u64)) {
            let _ = black_box(573 + 1);
        }
        if flattened_cov.contains_key(&(574 as u64)) {
            let _ = black_box(574 + 1);
        }
        if flattened_cov.contains_key(&(575 as u64)) {
            let _ = black_box(575 + 1);
        }
        if flattened_cov.contains_key(&(576 as u64)) {
            let _ = black_box(576 + 1);
        }
        if flattened_cov.contains_key(&(577 as u64)) {
            let _ = black_box(577 + 1);
        }
        if flattened_cov.contains_key(&(578 as u64)) {
            let _ = black_box(578 + 1);
        }
        if flattened_cov.contains_key(&(579 as u64)) {
            let _ = black_box(579 + 1);
        }
        if flattened_cov.contains_key(&(580 as u64)) {
            let _ = black_box(580 + 1);
        }
        if flattened_cov.contains_key(&(581 as u64)) {
            let _ = black_box(581 + 1);
        }
        if flattened_cov.contains_key(&(582 as u64)) {
            let _ = black_box(582 + 1);
        }
        if flattened_cov.contains_key(&(583 as u64)) {
            let _ = black_box(583 + 1);
        }
        if flattened_cov.contains_key(&(584 as u64)) {
            let _ = black_box(584 + 1);
        }
        if flattened_cov.contains_key(&(585 as u64)) {
            let _ = black_box(585 + 1);
        }
        if flattened_cov.contains_key(&(586 as u64)) {
            let _ = black_box(586 + 1);
        }
        if flattened_cov.contains_key(&(587 as u64)) {
            let _ = black_box(587 + 1);
        }
        if flattened_cov.contains_key(&(588 as u64)) {
            let _ = black_box(588 + 1);
        }
        if flattened_cov.contains_key(&(589 as u64)) {
            let _ = black_box(589 + 1);
        }
        if flattened_cov.contains_key(&(590 as u64)) {
            let _ = black_box(590 + 1);
        }
        if flattened_cov.contains_key(&(591 as u64)) {
            let _ = black_box(591 + 1);
        }
        if flattened_cov.contains_key(&(592 as u64)) {
            let _ = black_box(592 + 1);
        }
        if flattened_cov.contains_key(&(593 as u64)) {
            let _ = black_box(593 + 1);
        }
        if flattened_cov.contains_key(&(594 as u64)) {
            let _ = black_box(594 + 1);
        }
        if flattened_cov.contains_key(&(595 as u64)) {
            let _ = black_box(595 + 1);
        }
        if flattened_cov.contains_key(&(596 as u64)) {
            let _ = black_box(596 + 1);
        }
        if flattened_cov.contains_key(&(597 as u64)) {
            let _ = black_box(597 + 1);
        }
        if flattened_cov.contains_key(&(598 as u64)) {
            let _ = black_box(598 + 1);
        }
        if flattened_cov.contains_key(&(599 as u64)) {
            let _ = black_box(599 + 1);
        }
        if flattened_cov.contains_key(&(600 as u64)) {
            let _ = black_box(600 + 1);
        }
        if flattened_cov.contains_key(&(601 as u64)) {
            let _ = black_box(601 + 1);
        }
        if flattened_cov.contains_key(&(602 as u64)) {
            let _ = black_box(602 + 1);
        }
        if flattened_cov.contains_key(&(603 as u64)) {
            let _ = black_box(603 + 1);
        }
        if flattened_cov.contains_key(&(604 as u64)) {
            let _ = black_box(604 + 1);
        }
        if flattened_cov.contains_key(&(605 as u64)) {
            let _ = black_box(605 + 1);
        }
        if flattened_cov.contains_key(&(606 as u64)) {
            let _ = black_box(606 + 1);
        }
        if flattened_cov.contains_key(&(607 as u64)) {
            let _ = black_box(607 + 1);
        }
        if flattened_cov.contains_key(&(608 as u64)) {
            let _ = black_box(608 + 1);
        }
        if flattened_cov.contains_key(&(609 as u64)) {
            let _ = black_box(609 + 1);
        }
        if flattened_cov.contains_key(&(610 as u64)) {
            let _ = black_box(610 + 1);
        }
        if flattened_cov.contains_key(&(611 as u64)) {
            let _ = black_box(611 + 1);
        }
        if flattened_cov.contains_key(&(612 as u64)) {
            let _ = black_box(612 + 1);
        }
        if flattened_cov.contains_key(&(613 as u64)) {
            let _ = black_box(613 + 1);
        }
        if flattened_cov.contains_key(&(614 as u64)) {
            let _ = black_box(614 + 1);
        }
        if flattened_cov.contains_key(&(615 as u64)) {
            let _ = black_box(615 + 1);
        }
        if flattened_cov.contains_key(&(616 as u64)) {
            let _ = black_box(616 + 1);
        }
        if flattened_cov.contains_key(&(617 as u64)) {
            let _ = black_box(617 + 1);
        }
        if flattened_cov.contains_key(&(618 as u64)) {
            let _ = black_box(618 + 1);
        }
        if flattened_cov.contains_key(&(619 as u64)) {
            let _ = black_box(619 + 1);
        }
        if flattened_cov.contains_key(&(620 as u64)) {
            let _ = black_box(620 + 1);
        }
        if flattened_cov.contains_key(&(621 as u64)) {
            let _ = black_box(621 + 1);
        }
        if flattened_cov.contains_key(&(622 as u64)) {
            let _ = black_box(622 + 1);
        }
        if flattened_cov.contains_key(&(623 as u64)) {
            let _ = black_box(623 + 1);
        }
        if flattened_cov.contains_key(&(624 as u64)) {
            let _ = black_box(624 + 1);
        }
        if flattened_cov.contains_key(&(625 as u64)) {
            let _ = black_box(625 + 1);
        }
        if flattened_cov.contains_key(&(626 as u64)) {
            let _ = black_box(626 + 1);
        }
        if flattened_cov.contains_key(&(627 as u64)) {
            let _ = black_box(627 + 1);
        }
        if flattened_cov.contains_key(&(628 as u64)) {
            let _ = black_box(628 + 1);
        }
        if flattened_cov.contains_key(&(629 as u64)) {
            let _ = black_box(629 + 1);
        }
        if flattened_cov.contains_key(&(630 as u64)) {
            let _ = black_box(630 + 1);
        }
        if flattened_cov.contains_key(&(631 as u64)) {
            let _ = black_box(631 + 1);
        }
        if flattened_cov.contains_key(&(632 as u64)) {
            let _ = black_box(632 + 1);
        }
        if flattened_cov.contains_key(&(633 as u64)) {
            let _ = black_box(633 + 1);
        }
        if flattened_cov.contains_key(&(634 as u64)) {
            let _ = black_box(634 + 1);
        }
        if flattened_cov.contains_key(&(635 as u64)) {
            let _ = black_box(635 + 1);
        }
        if flattened_cov.contains_key(&(636 as u64)) {
            let _ = black_box(636 + 1);
        }
        if flattened_cov.contains_key(&(637 as u64)) {
            let _ = black_box(637 + 1);
        }
        if flattened_cov.contains_key(&(638 as u64)) {
            let _ = black_box(638 + 1);
        }
        if flattened_cov.contains_key(&(639 as u64)) {
            let _ = black_box(639 + 1);
        }
        if flattened_cov.contains_key(&(640 as u64)) {
            let _ = black_box(640 + 1);
        }
        if flattened_cov.contains_key(&(641 as u64)) {
            let _ = black_box(641 + 1);
        }
        if flattened_cov.contains_key(&(642 as u64)) {
            let _ = black_box(642 + 1);
        }
        if flattened_cov.contains_key(&(643 as u64)) {
            let _ = black_box(643 + 1);
        }
        if flattened_cov.contains_key(&(644 as u64)) {
            let _ = black_box(644 + 1);
        }
        if flattened_cov.contains_key(&(645 as u64)) {
            let _ = black_box(645 + 1);
        }
        if flattened_cov.contains_key(&(646 as u64)) {
            let _ = black_box(646 + 1);
        }
        if flattened_cov.contains_key(&(647 as u64)) {
            let _ = black_box(647 + 1);
        }
        if flattened_cov.contains_key(&(648 as u64)) {
            let _ = black_box(648 + 1);
        }
        if flattened_cov.contains_key(&(649 as u64)) {
            let _ = black_box(649 + 1);
        }
        if flattened_cov.contains_key(&(650 as u64)) {
            let _ = black_box(650 + 1);
        }
        if flattened_cov.contains_key(&(651 as u64)) {
            let _ = black_box(651 + 1);
        }
        if flattened_cov.contains_key(&(652 as u64)) {
            let _ = black_box(652 + 1);
        }
        if flattened_cov.contains_key(&(653 as u64)) {
            let _ = black_box(653 + 1);
        }
        if flattened_cov.contains_key(&(654 as u64)) {
            let _ = black_box(654 + 1);
        }
        if flattened_cov.contains_key(&(655 as u64)) {
            let _ = black_box(655 + 1);
        }
        if flattened_cov.contains_key(&(656 as u64)) {
            let _ = black_box(656 + 1);
        }
        if flattened_cov.contains_key(&(657 as u64)) {
            let _ = black_box(657 + 1);
        }
        if flattened_cov.contains_key(&(658 as u64)) {
            let _ = black_box(658 + 1);
        }
        if flattened_cov.contains_key(&(659 as u64)) {
            let _ = black_box(659 + 1);
        }
        if flattened_cov.contains_key(&(660 as u64)) {
            let _ = black_box(660 + 1);
        }
        if flattened_cov.contains_key(&(661 as u64)) {
            let _ = black_box(661 + 1);
        }
        if flattened_cov.contains_key(&(662 as u64)) {
            let _ = black_box(662 + 1);
        }
        if flattened_cov.contains_key(&(663 as u64)) {
            let _ = black_box(663 + 1);
        }
        if flattened_cov.contains_key(&(664 as u64)) {
            let _ = black_box(664 + 1);
        }
        if flattened_cov.contains_key(&(665 as u64)) {
            let _ = black_box(665 + 1);
        }
        if flattened_cov.contains_key(&(666 as u64)) {
            let _ = black_box(666 + 1);
        }
        if flattened_cov.contains_key(&(667 as u64)) {
            let _ = black_box(667 + 1);
        }
        if flattened_cov.contains_key(&(668 as u64)) {
            let _ = black_box(668 + 1);
        }
        if flattened_cov.contains_key(&(669 as u64)) {
            let _ = black_box(669 + 1);
        }
        if flattened_cov.contains_key(&(670 as u64)) {
            let _ = black_box(670 + 1);
        }
        if flattened_cov.contains_key(&(671 as u64)) {
            let _ = black_box(671 + 1);
        }
        if flattened_cov.contains_key(&(672 as u64)) {
            let _ = black_box(672 + 1);
        }
        if flattened_cov.contains_key(&(673 as u64)) {
            let _ = black_box(673 + 1);
        }
        if flattened_cov.contains_key(&(674 as u64)) {
            let _ = black_box(674 + 1);
        }
        if flattened_cov.contains_key(&(675 as u64)) {
            let _ = black_box(675 + 1);
        }
        if flattened_cov.contains_key(&(676 as u64)) {
            let _ = black_box(676 + 1);
        }
        if flattened_cov.contains_key(&(677 as u64)) {
            let _ = black_box(677 + 1);
        }
        if flattened_cov.contains_key(&(678 as u64)) {
            let _ = black_box(678 + 1);
        }
        if flattened_cov.contains_key(&(679 as u64)) {
            let _ = black_box(679 + 1);
        }
        if flattened_cov.contains_key(&(680 as u64)) {
            let _ = black_box(680 + 1);
        }
        if flattened_cov.contains_key(&(681 as u64)) {
            let _ = black_box(681 + 1);
        }
        if flattened_cov.contains_key(&(682 as u64)) {
            let _ = black_box(682 + 1);
        }
        if flattened_cov.contains_key(&(683 as u64)) {
            let _ = black_box(683 + 1);
        }
        if flattened_cov.contains_key(&(684 as u64)) {
            let _ = black_box(684 + 1);
        }
        if flattened_cov.contains_key(&(685 as u64)) {
            let _ = black_box(685 + 1);
        }
        if flattened_cov.contains_key(&(686 as u64)) {
            let _ = black_box(686 + 1);
        }
        if flattened_cov.contains_key(&(687 as u64)) {
            let _ = black_box(687 + 1);
        }
        if flattened_cov.contains_key(&(688 as u64)) {
            let _ = black_box(688 + 1);
        }
        if flattened_cov.contains_key(&(689 as u64)) {
            let _ = black_box(689 + 1);
        }
        if flattened_cov.contains_key(&(690 as u64)) {
            let _ = black_box(690 + 1);
        }
        if flattened_cov.contains_key(&(691 as u64)) {
            let _ = black_box(691 + 1);
        }
        if flattened_cov.contains_key(&(692 as u64)) {
            let _ = black_box(692 + 1);
        }
        if flattened_cov.contains_key(&(693 as u64)) {
            let _ = black_box(693 + 1);
        }
        if flattened_cov.contains_key(&(694 as u64)) {
            let _ = black_box(694 + 1);
        }
        if flattened_cov.contains_key(&(695 as u64)) {
            let _ = black_box(695 + 1);
        }
        if flattened_cov.contains_key(&(696 as u64)) {
            let _ = black_box(696 + 1);
        }
        if flattened_cov.contains_key(&(697 as u64)) {
            let _ = black_box(697 + 1);
        }
        if flattened_cov.contains_key(&(698 as u64)) {
            let _ = black_box(698 + 1);
        }
        if flattened_cov.contains_key(&(699 as u64)) {
            let _ = black_box(699 + 1);
        }
        if flattened_cov.contains_key(&(700 as u64)) {
            let _ = black_box(700 + 1);
        }
        if flattened_cov.contains_key(&(701 as u64)) {
            let _ = black_box(701 + 1);
        }
        if flattened_cov.contains_key(&(702 as u64)) {
            let _ = black_box(702 + 1);
        }
        if flattened_cov.contains_key(&(703 as u64)) {
            let _ = black_box(703 + 1);
        }
        if flattened_cov.contains_key(&(704 as u64)) {
            let _ = black_box(704 + 1);
        }
        if flattened_cov.contains_key(&(705 as u64)) {
            let _ = black_box(705 + 1);
        }
        if flattened_cov.contains_key(&(706 as u64)) {
            let _ = black_box(706 + 1);
        }
        if flattened_cov.contains_key(&(707 as u64)) {
            let _ = black_box(707 + 1);
        }
        if flattened_cov.contains_key(&(708 as u64)) {
            let _ = black_box(708 + 1);
        }
        if flattened_cov.contains_key(&(709 as u64)) {
            let _ = black_box(709 + 1);
        }
        if flattened_cov.contains_key(&(710 as u64)) {
            let _ = black_box(710 + 1);
        }
        if flattened_cov.contains_key(&(711 as u64)) {
            let _ = black_box(711 + 1);
        }
        if flattened_cov.contains_key(&(712 as u64)) {
            let _ = black_box(712 + 1);
        }
        if flattened_cov.contains_key(&(713 as u64)) {
            let _ = black_box(713 + 1);
        }
        if flattened_cov.contains_key(&(714 as u64)) {
            let _ = black_box(714 + 1);
        }
        if flattened_cov.contains_key(&(715 as u64)) {
            let _ = black_box(715 + 1);
        }
        if flattened_cov.contains_key(&(716 as u64)) {
            let _ = black_box(716 + 1);
        }
        if flattened_cov.contains_key(&(717 as u64)) {
            let _ = black_box(717 + 1);
        }
        if flattened_cov.contains_key(&(718 as u64)) {
            let _ = black_box(718 + 1);
        }
        if flattened_cov.contains_key(&(719 as u64)) {
            let _ = black_box(719 + 1);
        }
        if flattened_cov.contains_key(&(720 as u64)) {
            let _ = black_box(720 + 1);
        }
        if flattened_cov.contains_key(&(721 as u64)) {
            let _ = black_box(721 + 1);
        }
        if flattened_cov.contains_key(&(722 as u64)) {
            let _ = black_box(722 + 1);
        }
        if flattened_cov.contains_key(&(723 as u64)) {
            let _ = black_box(723 + 1);
        }
        if flattened_cov.contains_key(&(724 as u64)) {
            let _ = black_box(724 + 1);
        }
        if flattened_cov.contains_key(&(725 as u64)) {
            let _ = black_box(725 + 1);
        }
        if flattened_cov.contains_key(&(726 as u64)) {
            let _ = black_box(726 + 1);
        }
        if flattened_cov.contains_key(&(727 as u64)) {
            let _ = black_box(727 + 1);
        }
        if flattened_cov.contains_key(&(728 as u64)) {
            let _ = black_box(728 + 1);
        }
        if flattened_cov.contains_key(&(729 as u64)) {
            let _ = black_box(729 + 1);
        }
        if flattened_cov.contains_key(&(730 as u64)) {
            let _ = black_box(730 + 1);
        }
        if flattened_cov.contains_key(&(731 as u64)) {
            let _ = black_box(731 + 1);
        }
        if flattened_cov.contains_key(&(732 as u64)) {
            let _ = black_box(732 + 1);
        }
        if flattened_cov.contains_key(&(733 as u64)) {
            let _ = black_box(733 + 1);
        }
        if flattened_cov.contains_key(&(734 as u64)) {
            let _ = black_box(734 + 1);
        }
        if flattened_cov.contains_key(&(735 as u64)) {
            let _ = black_box(735 + 1);
        }
        if flattened_cov.contains_key(&(736 as u64)) {
            let _ = black_box(736 + 1);
        }
        if flattened_cov.contains_key(&(737 as u64)) {
            let _ = black_box(737 + 1);
        }
        if flattened_cov.contains_key(&(738 as u64)) {
            let _ = black_box(738 + 1);
        }
        if flattened_cov.contains_key(&(739 as u64)) {
            let _ = black_box(739 + 1);
        }
        if flattened_cov.contains_key(&(740 as u64)) {
            let _ = black_box(740 + 1);
        }
        if flattened_cov.contains_key(&(741 as u64)) {
            let _ = black_box(741 + 1);
        }
        if flattened_cov.contains_key(&(742 as u64)) {
            let _ = black_box(742 + 1);
        }
        if flattened_cov.contains_key(&(743 as u64)) {
            let _ = black_box(743 + 1);
        }
        if flattened_cov.contains_key(&(744 as u64)) {
            let _ = black_box(744 + 1);
        }
        if flattened_cov.contains_key(&(745 as u64)) {
            let _ = black_box(745 + 1);
        }
        if flattened_cov.contains_key(&(746 as u64)) {
            let _ = black_box(746 + 1);
        }
        if flattened_cov.contains_key(&(747 as u64)) {
            let _ = black_box(747 + 1);
        }
        if flattened_cov.contains_key(&(748 as u64)) {
            let _ = black_box(748 + 1);
        }
        if flattened_cov.contains_key(&(749 as u64)) {
            let _ = black_box(749 + 1);
        }
        if flattened_cov.contains_key(&(750 as u64)) {
            let _ = black_box(750 + 1);
        }
        if flattened_cov.contains_key(&(751 as u64)) {
            let _ = black_box(751 + 1);
        }
        if flattened_cov.contains_key(&(752 as u64)) {
            let _ = black_box(752 + 1);
        }
        if flattened_cov.contains_key(&(753 as u64)) {
            let _ = black_box(753 + 1);
        }
        if flattened_cov.contains_key(&(754 as u64)) {
            let _ = black_box(754 + 1);
        }
        if flattened_cov.contains_key(&(755 as u64)) {
            let _ = black_box(755 + 1);
        }
        if flattened_cov.contains_key(&(756 as u64)) {
            let _ = black_box(756 + 1);
        }
        if flattened_cov.contains_key(&(757 as u64)) {
            let _ = black_box(757 + 1);
        }
        if flattened_cov.contains_key(&(758 as u64)) {
            let _ = black_box(758 + 1);
        }
        if flattened_cov.contains_key(&(759 as u64)) {
            let _ = black_box(759 + 1);
        }
        if flattened_cov.contains_key(&(760 as u64)) {
            let _ = black_box(760 + 1);
        }
        if flattened_cov.contains_key(&(761 as u64)) {
            let _ = black_box(761 + 1);
        }
        if flattened_cov.contains_key(&(762 as u64)) {
            let _ = black_box(762 + 1);
        }
        if flattened_cov.contains_key(&(763 as u64)) {
            let _ = black_box(763 + 1);
        }
        if flattened_cov.contains_key(&(764 as u64)) {
            let _ = black_box(764 + 1);
        }
        if flattened_cov.contains_key(&(765 as u64)) {
            let _ = black_box(765 + 1);
        }
        if flattened_cov.contains_key(&(766 as u64)) {
            let _ = black_box(766 + 1);
        }
        if flattened_cov.contains_key(&(767 as u64)) {
            let _ = black_box(767 + 1);
        }
        if flattened_cov.contains_key(&(768 as u64)) {
            let _ = black_box(768 + 1);
        }
        if flattened_cov.contains_key(&(769 as u64)) {
            let _ = black_box(769 + 1);
        }
        if flattened_cov.contains_key(&(770 as u64)) {
            let _ = black_box(770 + 1);
        }
        if flattened_cov.contains_key(&(771 as u64)) {
            let _ = black_box(771 + 1);
        }
        if flattened_cov.contains_key(&(772 as u64)) {
            let _ = black_box(772 + 1);
        }
        if flattened_cov.contains_key(&(773 as u64)) {
            let _ = black_box(773 + 1);
        }
        if flattened_cov.contains_key(&(774 as u64)) {
            let _ = black_box(774 + 1);
        }
        if flattened_cov.contains_key(&(775 as u64)) {
            let _ = black_box(775 + 1);
        }
        if flattened_cov.contains_key(&(776 as u64)) {
            let _ = black_box(776 + 1);
        }
        if flattened_cov.contains_key(&(777 as u64)) {
            let _ = black_box(777 + 1);
        }
        if flattened_cov.contains_key(&(778 as u64)) {
            let _ = black_box(778 + 1);
        }
        if flattened_cov.contains_key(&(779 as u64)) {
            let _ = black_box(779 + 1);
        }
        if flattened_cov.contains_key(&(780 as u64)) {
            let _ = black_box(780 + 1);
        }
        if flattened_cov.contains_key(&(781 as u64)) {
            let _ = black_box(781 + 1);
        }
        if flattened_cov.contains_key(&(782 as u64)) {
            let _ = black_box(782 + 1);
        }
        if flattened_cov.contains_key(&(783 as u64)) {
            let _ = black_box(783 + 1);
        }
        if flattened_cov.contains_key(&(784 as u64)) {
            let _ = black_box(784 + 1);
        }
        if flattened_cov.contains_key(&(785 as u64)) {
            let _ = black_box(785 + 1);
        }
        if flattened_cov.contains_key(&(786 as u64)) {
            let _ = black_box(786 + 1);
        }
        if flattened_cov.contains_key(&(787 as u64)) {
            let _ = black_box(787 + 1);
        }
        if flattened_cov.contains_key(&(788 as u64)) {
            let _ = black_box(788 + 1);
        }
        if flattened_cov.contains_key(&(789 as u64)) {
            let _ = black_box(789 + 1);
        }
        if flattened_cov.contains_key(&(790 as u64)) {
            let _ = black_box(790 + 1);
        }
        if flattened_cov.contains_key(&(791 as u64)) {
            let _ = black_box(791 + 1);
        }
        if flattened_cov.contains_key(&(792 as u64)) {
            let _ = black_box(792 + 1);
        }
        if flattened_cov.contains_key(&(793 as u64)) {
            let _ = black_box(793 + 1);
        }
        if flattened_cov.contains_key(&(794 as u64)) {
            let _ = black_box(794 + 1);
        }
        if flattened_cov.contains_key(&(795 as u64)) {
            let _ = black_box(795 + 1);
        }
        if flattened_cov.contains_key(&(796 as u64)) {
            let _ = black_box(796 + 1);
        }
        if flattened_cov.contains_key(&(797 as u64)) {
            let _ = black_box(797 + 1);
        }
        if flattened_cov.contains_key(&(798 as u64)) {
            let _ = black_box(798 + 1);
        }
        if flattened_cov.contains_key(&(799 as u64)) {
            let _ = black_box(799 + 1);
        }
        if flattened_cov.contains_key(&(800 as u64)) {
            let _ = black_box(800 + 1);
        }
        if flattened_cov.contains_key(&(801 as u64)) {
            let _ = black_box(801 + 1);
        }
        if flattened_cov.contains_key(&(802 as u64)) {
            let _ = black_box(802 + 1);
        }
        if flattened_cov.contains_key(&(803 as u64)) {
            let _ = black_box(803 + 1);
        }
        if flattened_cov.contains_key(&(804 as u64)) {
            let _ = black_box(804 + 1);
        }
        if flattened_cov.contains_key(&(805 as u64)) {
            let _ = black_box(805 + 1);
        }
        if flattened_cov.contains_key(&(806 as u64)) {
            let _ = black_box(806 + 1);
        }
        if flattened_cov.contains_key(&(807 as u64)) {
            let _ = black_box(807 + 1);
        }
        if flattened_cov.contains_key(&(808 as u64)) {
            let _ = black_box(808 + 1);
        }
        if flattened_cov.contains_key(&(809 as u64)) {
            let _ = black_box(809 + 1);
        }
        if flattened_cov.contains_key(&(810 as u64)) {
            let _ = black_box(810 + 1);
        }
        if flattened_cov.contains_key(&(811 as u64)) {
            let _ = black_box(811 + 1);
        }
        if flattened_cov.contains_key(&(812 as u64)) {
            let _ = black_box(812 + 1);
        }
        if flattened_cov.contains_key(&(813 as u64)) {
            let _ = black_box(813 + 1);
        }
        if flattened_cov.contains_key(&(814 as u64)) {
            let _ = black_box(814 + 1);
        }
        if flattened_cov.contains_key(&(815 as u64)) {
            let _ = black_box(815 + 1);
        }
        if flattened_cov.contains_key(&(816 as u64)) {
            let _ = black_box(816 + 1);
        }
        if flattened_cov.contains_key(&(817 as u64)) {
            let _ = black_box(817 + 1);
        }
        if flattened_cov.contains_key(&(818 as u64)) {
            let _ = black_box(818 + 1);
        }
        if flattened_cov.contains_key(&(819 as u64)) {
            let _ = black_box(819 + 1);
        }
        if flattened_cov.contains_key(&(820 as u64)) {
            let _ = black_box(820 + 1);
        }
        if flattened_cov.contains_key(&(821 as u64)) {
            let _ = black_box(821 + 1);
        }
        if flattened_cov.contains_key(&(822 as u64)) {
            let _ = black_box(822 + 1);
        }
        if flattened_cov.contains_key(&(823 as u64)) {
            let _ = black_box(823 + 1);
        }
        if flattened_cov.contains_key(&(824 as u64)) {
            let _ = black_box(824 + 1);
        }
        if flattened_cov.contains_key(&(825 as u64)) {
            let _ = black_box(825 + 1);
        }
        if flattened_cov.contains_key(&(826 as u64)) {
            let _ = black_box(826 + 1);
        }
        if flattened_cov.contains_key(&(827 as u64)) {
            let _ = black_box(827 + 1);
        }
        if flattened_cov.contains_key(&(828 as u64)) {
            let _ = black_box(828 + 1);
        }
        if flattened_cov.contains_key(&(829 as u64)) {
            let _ = black_box(829 + 1);
        }
        if flattened_cov.contains_key(&(830 as u64)) {
            let _ = black_box(830 + 1);
        }
        if flattened_cov.contains_key(&(831 as u64)) {
            let _ = black_box(831 + 1);
        }
        if flattened_cov.contains_key(&(832 as u64)) {
            let _ = black_box(832 + 1);
        }
        if flattened_cov.contains_key(&(833 as u64)) {
            let _ = black_box(833 + 1);
        }
        if flattened_cov.contains_key(&(834 as u64)) {
            let _ = black_box(834 + 1);
        }
        if flattened_cov.contains_key(&(835 as u64)) {
            let _ = black_box(835 + 1);
        }
        if flattened_cov.contains_key(&(836 as u64)) {
            let _ = black_box(836 + 1);
        }
        if flattened_cov.contains_key(&(837 as u64)) {
            let _ = black_box(837 + 1);
        }
        if flattened_cov.contains_key(&(838 as u64)) {
            let _ = black_box(838 + 1);
        }
        if flattened_cov.contains_key(&(839 as u64)) {
            let _ = black_box(839 + 1);
        }
        if flattened_cov.contains_key(&(840 as u64)) {
            let _ = black_box(840 + 1);
        }
        if flattened_cov.contains_key(&(841 as u64)) {
            let _ = black_box(841 + 1);
        }
        if flattened_cov.contains_key(&(842 as u64)) {
            let _ = black_box(842 + 1);
        }
        if flattened_cov.contains_key(&(843 as u64)) {
            let _ = black_box(843 + 1);
        }
        if flattened_cov.contains_key(&(844 as u64)) {
            let _ = black_box(844 + 1);
        }
        if flattened_cov.contains_key(&(845 as u64)) {
            let _ = black_box(845 + 1);
        }
        if flattened_cov.contains_key(&(846 as u64)) {
            let _ = black_box(846 + 1);
        }
        if flattened_cov.contains_key(&(847 as u64)) {
            let _ = black_box(847 + 1);
        }
        if flattened_cov.contains_key(&(848 as u64)) {
            let _ = black_box(848 + 1);
        }
        if flattened_cov.contains_key(&(849 as u64)) {
            let _ = black_box(849 + 1);
        }
        if flattened_cov.contains_key(&(850 as u64)) {
            let _ = black_box(850 + 1);
        }
        if flattened_cov.contains_key(&(851 as u64)) {
            let _ = black_box(851 + 1);
        }
        if flattened_cov.contains_key(&(852 as u64)) {
            let _ = black_box(852 + 1);
        }
        if flattened_cov.contains_key(&(853 as u64)) {
            let _ = black_box(853 + 1);
        }
        if flattened_cov.contains_key(&(854 as u64)) {
            let _ = black_box(854 + 1);
        }
        if flattened_cov.contains_key(&(855 as u64)) {
            let _ = black_box(855 + 1);
        }
        if flattened_cov.contains_key(&(856 as u64)) {
            let _ = black_box(856 + 1);
        }
        if flattened_cov.contains_key(&(857 as u64)) {
            let _ = black_box(857 + 1);
        }
        if flattened_cov.contains_key(&(858 as u64)) {
            let _ = black_box(858 + 1);
        }
        if flattened_cov.contains_key(&(859 as u64)) {
            let _ = black_box(859 + 1);
        }
        if flattened_cov.contains_key(&(860 as u64)) {
            let _ = black_box(860 + 1);
        }
        if flattened_cov.contains_key(&(861 as u64)) {
            let _ = black_box(861 + 1);
        }
        if flattened_cov.contains_key(&(862 as u64)) {
            let _ = black_box(862 + 1);
        }
        if flattened_cov.contains_key(&(863 as u64)) {
            let _ = black_box(863 + 1);
        }
        if flattened_cov.contains_key(&(864 as u64)) {
            let _ = black_box(864 + 1);
        }
        if flattened_cov.contains_key(&(865 as u64)) {
            let _ = black_box(865 + 1);
        }
        if flattened_cov.contains_key(&(866 as u64)) {
            let _ = black_box(866 + 1);
        }
        if flattened_cov.contains_key(&(867 as u64)) {
            let _ = black_box(867 + 1);
        }
        if flattened_cov.contains_key(&(868 as u64)) {
            let _ = black_box(868 + 1);
        }
        if flattened_cov.contains_key(&(869 as u64)) {
            let _ = black_box(869 + 1);
        }
        if flattened_cov.contains_key(&(870 as u64)) {
            let _ = black_box(870 + 1);
        }
        if flattened_cov.contains_key(&(871 as u64)) {
            let _ = black_box(871 + 1);
        }
        if flattened_cov.contains_key(&(872 as u64)) {
            let _ = black_box(872 + 1);
        }
        if flattened_cov.contains_key(&(873 as u64)) {
            let _ = black_box(873 + 1);
        }
        if flattened_cov.contains_key(&(874 as u64)) {
            let _ = black_box(874 + 1);
        }
        if flattened_cov.contains_key(&(875 as u64)) {
            let _ = black_box(875 + 1);
        }
        if flattened_cov.contains_key(&(876 as u64)) {
            let _ = black_box(876 + 1);
        }
        if flattened_cov.contains_key(&(877 as u64)) {
            let _ = black_box(877 + 1);
        }
        if flattened_cov.contains_key(&(878 as u64)) {
            let _ = black_box(878 + 1);
        }
        if flattened_cov.contains_key(&(879 as u64)) {
            let _ = black_box(879 + 1);
        }
        if flattened_cov.contains_key(&(880 as u64)) {
            let _ = black_box(880 + 1);
        }
        if flattened_cov.contains_key(&(881 as u64)) {
            let _ = black_box(881 + 1);
        }
        if flattened_cov.contains_key(&(882 as u64)) {
            let _ = black_box(882 + 1);
        }
        if flattened_cov.contains_key(&(883 as u64)) {
            let _ = black_box(883 + 1);
        }
        if flattened_cov.contains_key(&(884 as u64)) {
            let _ = black_box(884 + 1);
        }
        if flattened_cov.contains_key(&(885 as u64)) {
            let _ = black_box(885 + 1);
        }
        if flattened_cov.contains_key(&(886 as u64)) {
            let _ = black_box(886 + 1);
        }
        if flattened_cov.contains_key(&(887 as u64)) {
            let _ = black_box(887 + 1);
        }
        if flattened_cov.contains_key(&(888 as u64)) {
            let _ = black_box(888 + 1);
        }
        if flattened_cov.contains_key(&(889 as u64)) {
            let _ = black_box(889 + 1);
        }
        if flattened_cov.contains_key(&(890 as u64)) {
            let _ = black_box(890 + 1);
        }
        if flattened_cov.contains_key(&(891 as u64)) {
            let _ = black_box(891 + 1);
        }
        if flattened_cov.contains_key(&(892 as u64)) {
            let _ = black_box(892 + 1);
        }
        if flattened_cov.contains_key(&(893 as u64)) {
            let _ = black_box(893 + 1);
        }
        if flattened_cov.contains_key(&(894 as u64)) {
            let _ = black_box(894 + 1);
        }
        if flattened_cov.contains_key(&(895 as u64)) {
            let _ = black_box(895 + 1);
        }
        if flattened_cov.contains_key(&(896 as u64)) {
            let _ = black_box(896 + 1);
        }
        if flattened_cov.contains_key(&(897 as u64)) {
            let _ = black_box(897 + 1);
        }
        if flattened_cov.contains_key(&(898 as u64)) {
            let _ = black_box(898 + 1);
        }
        if flattened_cov.contains_key(&(899 as u64)) {
            let _ = black_box(899 + 1);
        }
        if flattened_cov.contains_key(&(900 as u64)) {
            let _ = black_box(900 + 1);
        }
        if flattened_cov.contains_key(&(901 as u64)) {
            let _ = black_box(901 + 1);
        }
        if flattened_cov.contains_key(&(902 as u64)) {
            let _ = black_box(902 + 1);
        }
        if flattened_cov.contains_key(&(903 as u64)) {
            let _ = black_box(903 + 1);
        }
        if flattened_cov.contains_key(&(904 as u64)) {
            let _ = black_box(904 + 1);
        }
        if flattened_cov.contains_key(&(905 as u64)) {
            let _ = black_box(905 + 1);
        }
        if flattened_cov.contains_key(&(906 as u64)) {
            let _ = black_box(906 + 1);
        }
        if flattened_cov.contains_key(&(907 as u64)) {
            let _ = black_box(907 + 1);
        }
        if flattened_cov.contains_key(&(908 as u64)) {
            let _ = black_box(908 + 1);
        }
        if flattened_cov.contains_key(&(909 as u64)) {
            let _ = black_box(909 + 1);
        }
        if flattened_cov.contains_key(&(910 as u64)) {
            let _ = black_box(910 + 1);
        }
        if flattened_cov.contains_key(&(911 as u64)) {
            let _ = black_box(911 + 1);
        }
        if flattened_cov.contains_key(&(912 as u64)) {
            let _ = black_box(912 + 1);
        }
        if flattened_cov.contains_key(&(913 as u64)) {
            let _ = black_box(913 + 1);
        }
        if flattened_cov.contains_key(&(914 as u64)) {
            let _ = black_box(914 + 1);
        }
        if flattened_cov.contains_key(&(915 as u64)) {
            let _ = black_box(915 + 1);
        }
        if flattened_cov.contains_key(&(916 as u64)) {
            let _ = black_box(916 + 1);
        }
        if flattened_cov.contains_key(&(917 as u64)) {
            let _ = black_box(917 + 1);
        }
        if flattened_cov.contains_key(&(918 as u64)) {
            let _ = black_box(918 + 1);
        }
        if flattened_cov.contains_key(&(919 as u64)) {
            let _ = black_box(919 + 1);
        }
        if flattened_cov.contains_key(&(920 as u64)) {
            let _ = black_box(920 + 1);
        }
        if flattened_cov.contains_key(&(921 as u64)) {
            let _ = black_box(921 + 1);
        }
        if flattened_cov.contains_key(&(922 as u64)) {
            let _ = black_box(922 + 1);
        }
        if flattened_cov.contains_key(&(923 as u64)) {
            let _ = black_box(923 + 1);
        }
        if flattened_cov.contains_key(&(924 as u64)) {
            let _ = black_box(924 + 1);
        }
        if flattened_cov.contains_key(&(925 as u64)) {
            let _ = black_box(925 + 1);
        }
        if flattened_cov.contains_key(&(926 as u64)) {
            let _ = black_box(926 + 1);
        }
        if flattened_cov.contains_key(&(927 as u64)) {
            let _ = black_box(927 + 1);
        }
        if flattened_cov.contains_key(&(928 as u64)) {
            let _ = black_box(928 + 1);
        }
        if flattened_cov.contains_key(&(929 as u64)) {
            let _ = black_box(929 + 1);
        }
        if flattened_cov.contains_key(&(930 as u64)) {
            let _ = black_box(930 + 1);
        }
        if flattened_cov.contains_key(&(931 as u64)) {
            let _ = black_box(931 + 1);
        }
        if flattened_cov.contains_key(&(932 as u64)) {
            let _ = black_box(932 + 1);
        }
        if flattened_cov.contains_key(&(933 as u64)) {
            let _ = black_box(933 + 1);
        }
        if flattened_cov.contains_key(&(934 as u64)) {
            let _ = black_box(934 + 1);
        }
        if flattened_cov.contains_key(&(935 as u64)) {
            let _ = black_box(935 + 1);
        }
        if flattened_cov.contains_key(&(936 as u64)) {
            let _ = black_box(936 + 1);
        }
        if flattened_cov.contains_key(&(937 as u64)) {
            let _ = black_box(937 + 1);
        }
        if flattened_cov.contains_key(&(938 as u64)) {
            let _ = black_box(938 + 1);
        }
        if flattened_cov.contains_key(&(939 as u64)) {
            let _ = black_box(939 + 1);
        }
        if flattened_cov.contains_key(&(940 as u64)) {
            let _ = black_box(940 + 1);
        }
        if flattened_cov.contains_key(&(941 as u64)) {
            let _ = black_box(941 + 1);
        }
        if flattened_cov.contains_key(&(942 as u64)) {
            let _ = black_box(942 + 1);
        }
        if flattened_cov.contains_key(&(943 as u64)) {
            let _ = black_box(943 + 1);
        }
        if flattened_cov.contains_key(&(944 as u64)) {
            let _ = black_box(944 + 1);
        }
        if flattened_cov.contains_key(&(945 as u64)) {
            let _ = black_box(945 + 1);
        }
        if flattened_cov.contains_key(&(946 as u64)) {
            let _ = black_box(946 + 1);
        }
        if flattened_cov.contains_key(&(947 as u64)) {
            let _ = black_box(947 + 1);
        }
        if flattened_cov.contains_key(&(948 as u64)) {
            let _ = black_box(948 + 1);
        }
        if flattened_cov.contains_key(&(949 as u64)) {
            let _ = black_box(949 + 1);
        }
        if flattened_cov.contains_key(&(950 as u64)) {
            let _ = black_box(950 + 1);
        }
        if flattened_cov.contains_key(&(951 as u64)) {
            let _ = black_box(951 + 1);
        }
        if flattened_cov.contains_key(&(952 as u64)) {
            let _ = black_box(952 + 1);
        }
        if flattened_cov.contains_key(&(953 as u64)) {
            let _ = black_box(953 + 1);
        }
        if flattened_cov.contains_key(&(954 as u64)) {
            let _ = black_box(954 + 1);
        }
        if flattened_cov.contains_key(&(955 as u64)) {
            let _ = black_box(955 + 1);
        }
        if flattened_cov.contains_key(&(956 as u64)) {
            let _ = black_box(956 + 1);
        }
        if flattened_cov.contains_key(&(957 as u64)) {
            let _ = black_box(957 + 1);
        }
        if flattened_cov.contains_key(&(958 as u64)) {
            let _ = black_box(958 + 1);
        }
        if flattened_cov.contains_key(&(959 as u64)) {
            let _ = black_box(959 + 1);
        }
        if flattened_cov.contains_key(&(960 as u64)) {
            let _ = black_box(960 + 1);
        }
        if flattened_cov.contains_key(&(961 as u64)) {
            let _ = black_box(961 + 1);
        }
        if flattened_cov.contains_key(&(962 as u64)) {
            let _ = black_box(962 + 1);
        }
        if flattened_cov.contains_key(&(963 as u64)) {
            let _ = black_box(963 + 1);
        }
        if flattened_cov.contains_key(&(964 as u64)) {
            let _ = black_box(964 + 1);
        }
        if flattened_cov.contains_key(&(965 as u64)) {
            let _ = black_box(965 + 1);
        }
        if flattened_cov.contains_key(&(966 as u64)) {
            let _ = black_box(966 + 1);
        }
        if flattened_cov.contains_key(&(967 as u64)) {
            let _ = black_box(967 + 1);
        }
        if flattened_cov.contains_key(&(968 as u64)) {
            let _ = black_box(968 + 1);
        }
        if flattened_cov.contains_key(&(969 as u64)) {
            let _ = black_box(969 + 1);
        }
        if flattened_cov.contains_key(&(970 as u64)) {
            let _ = black_box(970 + 1);
        }
        if flattened_cov.contains_key(&(971 as u64)) {
            let _ = black_box(971 + 1);
        }
        if flattened_cov.contains_key(&(972 as u64)) {
            let _ = black_box(972 + 1);
        }
        if flattened_cov.contains_key(&(973 as u64)) {
            let _ = black_box(973 + 1);
        }
        if flattened_cov.contains_key(&(974 as u64)) {
            let _ = black_box(974 + 1);
        }
        if flattened_cov.contains_key(&(975 as u64)) {
            let _ = black_box(975 + 1);
        }
        if flattened_cov.contains_key(&(976 as u64)) {
            let _ = black_box(976 + 1);
        }
        if flattened_cov.contains_key(&(977 as u64)) {
            let _ = black_box(977 + 1);
        }
        if flattened_cov.contains_key(&(978 as u64)) {
            let _ = black_box(978 + 1);
        }
        if flattened_cov.contains_key(&(979 as u64)) {
            let _ = black_box(979 + 1);
        }
        if flattened_cov.contains_key(&(980 as u64)) {
            let _ = black_box(980 + 1);
        }
        if flattened_cov.contains_key(&(981 as u64)) {
            let _ = black_box(981 + 1);
        }
        if flattened_cov.contains_key(&(982 as u64)) {
            let _ = black_box(982 + 1);
        }
        if flattened_cov.contains_key(&(983 as u64)) {
            let _ = black_box(983 + 1);
        }
        if flattened_cov.contains_key(&(984 as u64)) {
            let _ = black_box(984 + 1);
        }
        if flattened_cov.contains_key(&(985 as u64)) {
            let _ = black_box(985 + 1);
        }
        if flattened_cov.contains_key(&(986 as u64)) {
            let _ = black_box(986 + 1);
        }
        if flattened_cov.contains_key(&(987 as u64)) {
            let _ = black_box(987 + 1);
        }
        if flattened_cov.contains_key(&(988 as u64)) {
            let _ = black_box(988 + 1);
        }
        if flattened_cov.contains_key(&(989 as u64)) {
            let _ = black_box(989 + 1);
        }
        if flattened_cov.contains_key(&(990 as u64)) {
            let _ = black_box(990 + 1);
        }
        if flattened_cov.contains_key(&(991 as u64)) {
            let _ = black_box(991 + 1);
        }
        if flattened_cov.contains_key(&(992 as u64)) {
            let _ = black_box(992 + 1);
        }
        if flattened_cov.contains_key(&(993 as u64)) {
            let _ = black_box(993 + 1);
        }
        if flattened_cov.contains_key(&(994 as u64)) {
            let _ = black_box(994 + 1);
        }
        if flattened_cov.contains_key(&(995 as u64)) {
            let _ = black_box(995 + 1);
        }
        if flattened_cov.contains_key(&(996 as u64)) {
            let _ = black_box(996 + 1);
        }
        if flattened_cov.contains_key(&(997 as u64)) {
            let _ = black_box(997 + 1);
        }
        if flattened_cov.contains_key(&(998 as u64)) {
            let _ = black_box(998 + 1);
        }
        if flattened_cov.contains_key(&(999 as u64)) {
            let _ = black_box(999 + 1);
        }
        if flattened_cov.contains_key(&(1000 as u64)) {
            let _ = black_box(1000 + 1);
        }
        if flattened_cov.contains_key(&(1001 as u64)) {
            let _ = black_box(1001 + 1);
        }
        if flattened_cov.contains_key(&(1002 as u64)) {
            let _ = black_box(1002 + 1);
        }
        if flattened_cov.contains_key(&(1003 as u64)) {
            let _ = black_box(1003 + 1);
        }
        if flattened_cov.contains_key(&(1004 as u64)) {
            let _ = black_box(1004 + 1);
        }
        if flattened_cov.contains_key(&(1005 as u64)) {
            let _ = black_box(1005 + 1);
        }
        if flattened_cov.contains_key(&(1006 as u64)) {
            let _ = black_box(1006 + 1);
        }
        if flattened_cov.contains_key(&(1007 as u64)) {
            let _ = black_box(1007 + 1);
        }
        if flattened_cov.contains_key(&(1008 as u64)) {
            let _ = black_box(1008 + 1);
        }
        if flattened_cov.contains_key(&(1009 as u64)) {
            let _ = black_box(1009 + 1);
        }
        if flattened_cov.contains_key(&(1010 as u64)) {
            let _ = black_box(1010 + 1);
        }
        if flattened_cov.contains_key(&(1011 as u64)) {
            let _ = black_box(1011 + 1);
        }
        if flattened_cov.contains_key(&(1012 as u64)) {
            let _ = black_box(1012 + 1);
        }
        if flattened_cov.contains_key(&(1013 as u64)) {
            let _ = black_box(1013 + 1);
        }
        if flattened_cov.contains_key(&(1014 as u64)) {
            let _ = black_box(1014 + 1);
        }
        if flattened_cov.contains_key(&(1015 as u64)) {
            let _ = black_box(1015 + 1);
        }
        if flattened_cov.contains_key(&(1016 as u64)) {
            let _ = black_box(1016 + 1);
        }
        if flattened_cov.contains_key(&(1017 as u64)) {
            let _ = black_box(1017 + 1);
        }
        if flattened_cov.contains_key(&(1018 as u64)) {
            let _ = black_box(1018 + 1);
        }
        if flattened_cov.contains_key(&(1019 as u64)) {
            let _ = black_box(1019 + 1);
        }
        if flattened_cov.contains_key(&(1020 as u64)) {
            let _ = black_box(1020 + 1);
        }
        if flattened_cov.contains_key(&(1021 as u64)) {
            let _ = black_box(1021 + 1);
        }
        if flattened_cov.contains_key(&(1022 as u64)) {
            let _ = black_box(1022 + 1);
        }
        if flattened_cov.contains_key(&(1023 as u64)) {
            let _ = black_box(1023 + 1);
        }
        if flattened_cov.contains_key(&(1024 as u64)) {
            let _ = black_box(1024 + 1);
        }
        if flattened_cov.contains_key(&(1025 as u64)) {
            let _ = black_box(1025 + 1);
        }
        if flattened_cov.contains_key(&(1026 as u64)) {
            let _ = black_box(1026 + 1);
        }
        if flattened_cov.contains_key(&(1027 as u64)) {
            let _ = black_box(1027 + 1);
        }
        if flattened_cov.contains_key(&(1028 as u64)) {
            let _ = black_box(1028 + 1);
        }
        if flattened_cov.contains_key(&(1029 as u64)) {
            let _ = black_box(1029 + 1);
        }
        if flattened_cov.contains_key(&(1030 as u64)) {
            let _ = black_box(1030 + 1);
        }
        if flattened_cov.contains_key(&(1031 as u64)) {
            let _ = black_box(1031 + 1);
        }
        if flattened_cov.contains_key(&(1032 as u64)) {
            let _ = black_box(1032 + 1);
        }
        if flattened_cov.contains_key(&(1033 as u64)) {
            let _ = black_box(1033 + 1);
        }
        if flattened_cov.contains_key(&(1034 as u64)) {
            let _ = black_box(1034 + 1);
        }
        if flattened_cov.contains_key(&(1035 as u64)) {
            let _ = black_box(1035 + 1);
        }
        if flattened_cov.contains_key(&(1036 as u64)) {
            let _ = black_box(1036 + 1);
        }
        if flattened_cov.contains_key(&(1037 as u64)) {
            let _ = black_box(1037 + 1);
        }
        if flattened_cov.contains_key(&(1038 as u64)) {
            let _ = black_box(1038 + 1);
        }
        if flattened_cov.contains_key(&(1039 as u64)) {
            let _ = black_box(1039 + 1);
        }
        if flattened_cov.contains_key(&(1040 as u64)) {
            let _ = black_box(1040 + 1);
        }
        if flattened_cov.contains_key(&(1041 as u64)) {
            let _ = black_box(1041 + 1);
        }
        if flattened_cov.contains_key(&(1042 as u64)) {
            let _ = black_box(1042 + 1);
        }
        if flattened_cov.contains_key(&(1043 as u64)) {
            let _ = black_box(1043 + 1);
        }
        if flattened_cov.contains_key(&(1044 as u64)) {
            let _ = black_box(1044 + 1);
        }
        if flattened_cov.contains_key(&(1045 as u64)) {
            let _ = black_box(1045 + 1);
        }
        if flattened_cov.contains_key(&(1046 as u64)) {
            let _ = black_box(1046 + 1);
        }
        if flattened_cov.contains_key(&(1047 as u64)) {
            let _ = black_box(1047 + 1);
        }
        if flattened_cov.contains_key(&(1048 as u64)) {
            let _ = black_box(1048 + 1);
        }
        if flattened_cov.contains_key(&(1049 as u64)) {
            let _ = black_box(1049 + 1);
        }
        if flattened_cov.contains_key(&(1050 as u64)) {
            let _ = black_box(1050 + 1);
        }
        if flattened_cov.contains_key(&(1051 as u64)) {
            let _ = black_box(1051 + 1);
        }
        if flattened_cov.contains_key(&(1052 as u64)) {
            let _ = black_box(1052 + 1);
        }
        if flattened_cov.contains_key(&(1053 as u64)) {
            let _ = black_box(1053 + 1);
        }
        if flattened_cov.contains_key(&(1054 as u64)) {
            let _ = black_box(1054 + 1);
        }
        if flattened_cov.contains_key(&(1055 as u64)) {
            let _ = black_box(1055 + 1);
        }
        if flattened_cov.contains_key(&(1056 as u64)) {
            let _ = black_box(1056 + 1);
        }
        if flattened_cov.contains_key(&(1057 as u64)) {
            let _ = black_box(1057 + 1);
        }
        if flattened_cov.contains_key(&(1058 as u64)) {
            let _ = black_box(1058 + 1);
        }
        if flattened_cov.contains_key(&(1059 as u64)) {
            let _ = black_box(1059 + 1);
        }
        if flattened_cov.contains_key(&(1060 as u64)) {
            let _ = black_box(1060 + 1);
        }
        if flattened_cov.contains_key(&(1061 as u64)) {
            let _ = black_box(1061 + 1);
        }
        if flattened_cov.contains_key(&(1062 as u64)) {
            let _ = black_box(1062 + 1);
        }
        if flattened_cov.contains_key(&(1063 as u64)) {
            let _ = black_box(1063 + 1);
        }
        if flattened_cov.contains_key(&(1064 as u64)) {
            let _ = black_box(1064 + 1);
        }
        if flattened_cov.contains_key(&(1065 as u64)) {
            let _ = black_box(1065 + 1);
        }
        if flattened_cov.contains_key(&(1066 as u64)) {
            let _ = black_box(1066 + 1);
        }
        if flattened_cov.contains_key(&(1067 as u64)) {
            let _ = black_box(1067 + 1);
        }
        if flattened_cov.contains_key(&(1068 as u64)) {
            let _ = black_box(1068 + 1);
        }
        if flattened_cov.contains_key(&(1069 as u64)) {
            let _ = black_box(1069 + 1);
        }
        if flattened_cov.contains_key(&(1070 as u64)) {
            let _ = black_box(1070 + 1);
        }
        if flattened_cov.contains_key(&(1071 as u64)) {
            let _ = black_box(1071 + 1);
        }
        if flattened_cov.contains_key(&(1072 as u64)) {
            let _ = black_box(1072 + 1);
        }
        if flattened_cov.contains_key(&(1073 as u64)) {
            let _ = black_box(1073 + 1);
        }
        if flattened_cov.contains_key(&(1074 as u64)) {
            let _ = black_box(1074 + 1);
        }
        if flattened_cov.contains_key(&(1075 as u64)) {
            let _ = black_box(1075 + 1);
        }
        if flattened_cov.contains_key(&(1076 as u64)) {
            let _ = black_box(1076 + 1);
        }
        if flattened_cov.contains_key(&(1077 as u64)) {
            let _ = black_box(1077 + 1);
        }
        if flattened_cov.contains_key(&(1078 as u64)) {
            let _ = black_box(1078 + 1);
        }
        if flattened_cov.contains_key(&(1079 as u64)) {
            let _ = black_box(1079 + 1);
        }
        if flattened_cov.contains_key(&(1080 as u64)) {
            let _ = black_box(1080 + 1);
        }
        if flattened_cov.contains_key(&(1081 as u64)) {
            let _ = black_box(1081 + 1);
        }
        if flattened_cov.contains_key(&(1082 as u64)) {
            let _ = black_box(1082 + 1);
        }
        if flattened_cov.contains_key(&(1083 as u64)) {
            let _ = black_box(1083 + 1);
        }
        if flattened_cov.contains_key(&(1084 as u64)) {
            let _ = black_box(1084 + 1);
        }
        if flattened_cov.contains_key(&(1085 as u64)) {
            let _ = black_box(1085 + 1);
        }
        if flattened_cov.contains_key(&(1086 as u64)) {
            let _ = black_box(1086 + 1);
        }
        if flattened_cov.contains_key(&(1087 as u64)) {
            let _ = black_box(1087 + 1);
        }
        if flattened_cov.contains_key(&(1088 as u64)) {
            let _ = black_box(1088 + 1);
        }
        if flattened_cov.contains_key(&(1089 as u64)) {
            let _ = black_box(1089 + 1);
        }
        if flattened_cov.contains_key(&(1090 as u64)) {
            let _ = black_box(1090 + 1);
        }
        if flattened_cov.contains_key(&(1091 as u64)) {
            let _ = black_box(1091 + 1);
        }
        if flattened_cov.contains_key(&(1092 as u64)) {
            let _ = black_box(1092 + 1);
        }
        if flattened_cov.contains_key(&(1093 as u64)) {
            let _ = black_box(1093 + 1);
        }
        if flattened_cov.contains_key(&(1094 as u64)) {
            let _ = black_box(1094 + 1);
        }
        if flattened_cov.contains_key(&(1095 as u64)) {
            let _ = black_box(1095 + 1);
        }
        if flattened_cov.contains_key(&(1096 as u64)) {
            let _ = black_box(1096 + 1);
        }
        if flattened_cov.contains_key(&(1097 as u64)) {
            let _ = black_box(1097 + 1);
        }
        if flattened_cov.contains_key(&(1098 as u64)) {
            let _ = black_box(1098 + 1);
        }
        if flattened_cov.contains_key(&(1099 as u64)) {
            let _ = black_box(1099 + 1);
        }
        if flattened_cov.contains_key(&(1100 as u64)) {
            let _ = black_box(1100 + 1);
        }
        if flattened_cov.contains_key(&(1101 as u64)) {
            let _ = black_box(1101 + 1);
        }
        if flattened_cov.contains_key(&(1102 as u64)) {
            let _ = black_box(1102 + 1);
        }
        if flattened_cov.contains_key(&(1103 as u64)) {
            let _ = black_box(1103 + 1);
        }
        if flattened_cov.contains_key(&(1104 as u64)) {
            let _ = black_box(1104 + 1);
        }
        if flattened_cov.contains_key(&(1105 as u64)) {
            let _ = black_box(1105 + 1);
        }
        if flattened_cov.contains_key(&(1106 as u64)) {
            let _ = black_box(1106 + 1);
        }
        if flattened_cov.contains_key(&(1107 as u64)) {
            let _ = black_box(1107 + 1);
        }
        if flattened_cov.contains_key(&(1108 as u64)) {
            let _ = black_box(1108 + 1);
        }
        if flattened_cov.contains_key(&(1109 as u64)) {
            let _ = black_box(1109 + 1);
        }
        if flattened_cov.contains_key(&(1110 as u64)) {
            let _ = black_box(1110 + 1);
        }
        if flattened_cov.contains_key(&(1111 as u64)) {
            let _ = black_box(1111 + 1);
        }
        if flattened_cov.contains_key(&(1112 as u64)) {
            let _ = black_box(1112 + 1);
        }
        if flattened_cov.contains_key(&(1113 as u64)) {
            let _ = black_box(1113 + 1);
        }
        if flattened_cov.contains_key(&(1114 as u64)) {
            let _ = black_box(1114 + 1);
        }
        if flattened_cov.contains_key(&(1115 as u64)) {
            let _ = black_box(1115 + 1);
        }
        if flattened_cov.contains_key(&(1116 as u64)) {
            let _ = black_box(1116 + 1);
        }
        if flattened_cov.contains_key(&(1117 as u64)) {
            let _ = black_box(1117 + 1);
        }
        if flattened_cov.contains_key(&(1118 as u64)) {
            let _ = black_box(1118 + 1);
        }
        if flattened_cov.contains_key(&(1119 as u64)) {
            let _ = black_box(1119 + 1);
        }
        if flattened_cov.contains_key(&(1120 as u64)) {
            let _ = black_box(1120 + 1);
        }
        if flattened_cov.contains_key(&(1121 as u64)) {
            let _ = black_box(1121 + 1);
        }
        if flattened_cov.contains_key(&(1122 as u64)) {
            let _ = black_box(1122 + 1);
        }
        if flattened_cov.contains_key(&(1123 as u64)) {
            let _ = black_box(1123 + 1);
        }
        if flattened_cov.contains_key(&(1124 as u64)) {
            let _ = black_box(1124 + 1);
        }
        if flattened_cov.contains_key(&(1125 as u64)) {
            let _ = black_box(1125 + 1);
        }
        if flattened_cov.contains_key(&(1126 as u64)) {
            let _ = black_box(1126 + 1);
        }
        if flattened_cov.contains_key(&(1127 as u64)) {
            let _ = black_box(1127 + 1);
        }
        if flattened_cov.contains_key(&(1128 as u64)) {
            let _ = black_box(1128 + 1);
        }
        if flattened_cov.contains_key(&(1129 as u64)) {
            let _ = black_box(1129 + 1);
        }
        if flattened_cov.contains_key(&(1130 as u64)) {
            let _ = black_box(1130 + 1);
        }
        if flattened_cov.contains_key(&(1131 as u64)) {
            let _ = black_box(1131 + 1);
        }
        if flattened_cov.contains_key(&(1132 as u64)) {
            let _ = black_box(1132 + 1);
        }
        if flattened_cov.contains_key(&(1133 as u64)) {
            let _ = black_box(1133 + 1);
        }
        if flattened_cov.contains_key(&(1134 as u64)) {
            let _ = black_box(1134 + 1);
        }
        if flattened_cov.contains_key(&(1135 as u64)) {
            let _ = black_box(1135 + 1);
        }
        if flattened_cov.contains_key(&(1136 as u64)) {
            let _ = black_box(1136 + 1);
        }
        if flattened_cov.contains_key(&(1137 as u64)) {
            let _ = black_box(1137 + 1);
        }
        if flattened_cov.contains_key(&(1138 as u64)) {
            let _ = black_box(1138 + 1);
        }
        if flattened_cov.contains_key(&(1139 as u64)) {
            let _ = black_box(1139 + 1);
        }
        if flattened_cov.contains_key(&(1140 as u64)) {
            let _ = black_box(1140 + 1);
        }
        if flattened_cov.contains_key(&(1141 as u64)) {
            let _ = black_box(1141 + 1);
        }
        if flattened_cov.contains_key(&(1142 as u64)) {
            let _ = black_box(1142 + 1);
        }
        if flattened_cov.contains_key(&(1143 as u64)) {
            let _ = black_box(1143 + 1);
        }
        if flattened_cov.contains_key(&(1144 as u64)) {
            let _ = black_box(1144 + 1);
        }
        if flattened_cov.contains_key(&(1145 as u64)) {
            let _ = black_box(1145 + 1);
        }
        if flattened_cov.contains_key(&(1146 as u64)) {
            let _ = black_box(1146 + 1);
        }
        if flattened_cov.contains_key(&(1147 as u64)) {
            let _ = black_box(1147 + 1);
        }
        if flattened_cov.contains_key(&(1148 as u64)) {
            let _ = black_box(1148 + 1);
        }
        if flattened_cov.contains_key(&(1149 as u64)) {
            let _ = black_box(1149 + 1);
        }
        if flattened_cov.contains_key(&(1150 as u64)) {
            let _ = black_box(1150 + 1);
        }
        if flattened_cov.contains_key(&(1151 as u64)) {
            let _ = black_box(1151 + 1);
        }
        if flattened_cov.contains_key(&(1152 as u64)) {
            let _ = black_box(1152 + 1);
        }
        if flattened_cov.contains_key(&(1153 as u64)) {
            let _ = black_box(1153 + 1);
        }
        if flattened_cov.contains_key(&(1154 as u64)) {
            let _ = black_box(1154 + 1);
        }
        if flattened_cov.contains_key(&(1155 as u64)) {
            let _ = black_box(1155 + 1);
        }
        if flattened_cov.contains_key(&(1156 as u64)) {
            let _ = black_box(1156 + 1);
        }
        if flattened_cov.contains_key(&(1157 as u64)) {
            let _ = black_box(1157 + 1);
        }
        if flattened_cov.contains_key(&(1158 as u64)) {
            let _ = black_box(1158 + 1);
        }
        if flattened_cov.contains_key(&(1159 as u64)) {
            let _ = black_box(1159 + 1);
        }
        if flattened_cov.contains_key(&(1160 as u64)) {
            let _ = black_box(1160 + 1);
        }
        if flattened_cov.contains_key(&(1161 as u64)) {
            let _ = black_box(1161 + 1);
        }
        if flattened_cov.contains_key(&(1162 as u64)) {
            let _ = black_box(1162 + 1);
        }
        if flattened_cov.contains_key(&(1163 as u64)) {
            let _ = black_box(1163 + 1);
        }
        if flattened_cov.contains_key(&(1164 as u64)) {
            let _ = black_box(1164 + 1);
        }
        if flattened_cov.contains_key(&(1165 as u64)) {
            let _ = black_box(1165 + 1);
        }
        if flattened_cov.contains_key(&(1166 as u64)) {
            let _ = black_box(1166 + 1);
        }
        if flattened_cov.contains_key(&(1167 as u64)) {
            let _ = black_box(1167 + 1);
        }
        if flattened_cov.contains_key(&(1168 as u64)) {
            let _ = black_box(1168 + 1);
        }
        if flattened_cov.contains_key(&(1169 as u64)) {
            let _ = black_box(1169 + 1);
        }
        if flattened_cov.contains_key(&(1170 as u64)) {
            let _ = black_box(1170 + 1);
        }
        if flattened_cov.contains_key(&(1171 as u64)) {
            let _ = black_box(1171 + 1);
        }
        if flattened_cov.contains_key(&(1172 as u64)) {
            let _ = black_box(1172 + 1);
        }
        if flattened_cov.contains_key(&(1173 as u64)) {
            let _ = black_box(1173 + 1);
        }
        if flattened_cov.contains_key(&(1174 as u64)) {
            let _ = black_box(1174 + 1);
        }
        if flattened_cov.contains_key(&(1175 as u64)) {
            let _ = black_box(1175 + 1);
        }
        if flattened_cov.contains_key(&(1176 as u64)) {
            let _ = black_box(1176 + 1);
        }
        if flattened_cov.contains_key(&(1177 as u64)) {
            let _ = black_box(1177 + 1);
        }
        if flattened_cov.contains_key(&(1178 as u64)) {
            let _ = black_box(1178 + 1);
        }
        if flattened_cov.contains_key(&(1179 as u64)) {
            let _ = black_box(1179 + 1);
        }
        if flattened_cov.contains_key(&(1180 as u64)) {
            let _ = black_box(1180 + 1);
        }
        if flattened_cov.contains_key(&(1181 as u64)) {
            let _ = black_box(1181 + 1);
        }
        if flattened_cov.contains_key(&(1182 as u64)) {
            let _ = black_box(1182 + 1);
        }
        if flattened_cov.contains_key(&(1183 as u64)) {
            let _ = black_box(1183 + 1);
        }
        if flattened_cov.contains_key(&(1184 as u64)) {
            let _ = black_box(1184 + 1);
        }
        if flattened_cov.contains_key(&(1185 as u64)) {
            let _ = black_box(1185 + 1);
        }
        if flattened_cov.contains_key(&(1186 as u64)) {
            let _ = black_box(1186 + 1);
        }
        if flattened_cov.contains_key(&(1187 as u64)) {
            let _ = black_box(1187 + 1);
        }
        if flattened_cov.contains_key(&(1188 as u64)) {
            let _ = black_box(1188 + 1);
        }
        if flattened_cov.contains_key(&(1189 as u64)) {
            let _ = black_box(1189 + 1);
        }
        if flattened_cov.contains_key(&(1190 as u64)) {
            let _ = black_box(1190 + 1);
        }
        if flattened_cov.contains_key(&(1191 as u64)) {
            let _ = black_box(1191 + 1);
        }
        if flattened_cov.contains_key(&(1192 as u64)) {
            let _ = black_box(1192 + 1);
        }
        if flattened_cov.contains_key(&(1193 as u64)) {
            let _ = black_box(1193 + 1);
        }
        if flattened_cov.contains_key(&(1194 as u64)) {
            let _ = black_box(1194 + 1);
        }
        if flattened_cov.contains_key(&(1195 as u64)) {
            let _ = black_box(1195 + 1);
        }
        if flattened_cov.contains_key(&(1196 as u64)) {
            let _ = black_box(1196 + 1);
        }
        if flattened_cov.contains_key(&(1197 as u64)) {
            let _ = black_box(1197 + 1);
        }
        if flattened_cov.contains_key(&(1198 as u64)) {
            let _ = black_box(1198 + 1);
        }
        if flattened_cov.contains_key(&(1199 as u64)) {
            let _ = black_box(1199 + 1);
        }
        if flattened_cov.contains_key(&(1200 as u64)) {
            let _ = black_box(1200 + 1);
        }
        if flattened_cov.contains_key(&(1201 as u64)) {
            let _ = black_box(1201 + 1);
        }
        if flattened_cov.contains_key(&(1202 as u64)) {
            let _ = black_box(1202 + 1);
        }
        if flattened_cov.contains_key(&(1203 as u64)) {
            let _ = black_box(1203 + 1);
        }
        if flattened_cov.contains_key(&(1204 as u64)) {
            let _ = black_box(1204 + 1);
        }
        if flattened_cov.contains_key(&(1205 as u64)) {
            let _ = black_box(1205 + 1);
        }
        if flattened_cov.contains_key(&(1206 as u64)) {
            let _ = black_box(1206 + 1);
        }
        if flattened_cov.contains_key(&(1207 as u64)) {
            let _ = black_box(1207 + 1);
        }
        if flattened_cov.contains_key(&(1208 as u64)) {
            let _ = black_box(1208 + 1);
        }
        if flattened_cov.contains_key(&(1209 as u64)) {
            let _ = black_box(1209 + 1);
        }
        if flattened_cov.contains_key(&(1210 as u64)) {
            let _ = black_box(1210 + 1);
        }
        if flattened_cov.contains_key(&(1211 as u64)) {
            let _ = black_box(1211 + 1);
        }
        if flattened_cov.contains_key(&(1212 as u64)) {
            let _ = black_box(1212 + 1);
        }
        if flattened_cov.contains_key(&(1213 as u64)) {
            let _ = black_box(1213 + 1);
        }
        if flattened_cov.contains_key(&(1214 as u64)) {
            let _ = black_box(1214 + 1);
        }
        if flattened_cov.contains_key(&(1215 as u64)) {
            let _ = black_box(1215 + 1);
        }
        if flattened_cov.contains_key(&(1216 as u64)) {
            let _ = black_box(1216 + 1);
        }
        if flattened_cov.contains_key(&(1217 as u64)) {
            let _ = black_box(1217 + 1);
        }
        if flattened_cov.contains_key(&(1218 as u64)) {
            let _ = black_box(1218 + 1);
        }
        if flattened_cov.contains_key(&(1219 as u64)) {
            let _ = black_box(1219 + 1);
        }
        if flattened_cov.contains_key(&(1220 as u64)) {
            let _ = black_box(1220 + 1);
        }
        if flattened_cov.contains_key(&(1221 as u64)) {
            let _ = black_box(1221 + 1);
        }
        if flattened_cov.contains_key(&(1222 as u64)) {
            let _ = black_box(1222 + 1);
        }
        if flattened_cov.contains_key(&(1223 as u64)) {
            let _ = black_box(1223 + 1);
        }
        if flattened_cov.contains_key(&(1224 as u64)) {
            let _ = black_box(1224 + 1);
        }
        if flattened_cov.contains_key(&(1225 as u64)) {
            let _ = black_box(1225 + 1);
        }
        if flattened_cov.contains_key(&(1226 as u64)) {
            let _ = black_box(1226 + 1);
        }
        if flattened_cov.contains_key(&(1227 as u64)) {
            let _ = black_box(1227 + 1);
        }
        if flattened_cov.contains_key(&(1228 as u64)) {
            let _ = black_box(1228 + 1);
        }
        if flattened_cov.contains_key(&(1229 as u64)) {
            let _ = black_box(1229 + 1);
        }
        if flattened_cov.contains_key(&(1230 as u64)) {
            let _ = black_box(1230 + 1);
        }
        if flattened_cov.contains_key(&(1231 as u64)) {
            let _ = black_box(1231 + 1);
        }
        if flattened_cov.contains_key(&(1232 as u64)) {
            let _ = black_box(1232 + 1);
        }
        if flattened_cov.contains_key(&(1233 as u64)) {
            let _ = black_box(1233 + 1);
        }
        if flattened_cov.contains_key(&(1234 as u64)) {
            let _ = black_box(1234 + 1);
        }
        if flattened_cov.contains_key(&(1235 as u64)) {
            let _ = black_box(1235 + 1);
        }
        if flattened_cov.contains_key(&(1236 as u64)) {
            let _ = black_box(1236 + 1);
        }
        if flattened_cov.contains_key(&(1237 as u64)) {
            let _ = black_box(1237 + 1);
        }
        if flattened_cov.contains_key(&(1238 as u64)) {
            let _ = black_box(1238 + 1);
        }
        if flattened_cov.contains_key(&(1239 as u64)) {
            let _ = black_box(1239 + 1);
        }
        if flattened_cov.contains_key(&(1240 as u64)) {
            let _ = black_box(1240 + 1);
        }
        if flattened_cov.contains_key(&(1241 as u64)) {
            let _ = black_box(1241 + 1);
        }
        if flattened_cov.contains_key(&(1242 as u64)) {
            let _ = black_box(1242 + 1);
        }
        if flattened_cov.contains_key(&(1243 as u64)) {
            let _ = black_box(1243 + 1);
        }
        if flattened_cov.contains_key(&(1244 as u64)) {
            let _ = black_box(1244 + 1);
        }
        if flattened_cov.contains_key(&(1245 as u64)) {
            let _ = black_box(1245 + 1);
        }
        if flattened_cov.contains_key(&(1246 as u64)) {
            let _ = black_box(1246 + 1);
        }
        if flattened_cov.contains_key(&(1247 as u64)) {
            let _ = black_box(1247 + 1);
        }
        if flattened_cov.contains_key(&(1248 as u64)) {
            let _ = black_box(1248 + 1);
        }
        if flattened_cov.contains_key(&(1249 as u64)) {
            let _ = black_box(1249 + 1);
        }
        if flattened_cov.contains_key(&(1250 as u64)) {
            let _ = black_box(1250 + 1);
        }
        if flattened_cov.contains_key(&(1251 as u64)) {
            let _ = black_box(1251 + 1);
        }
        if flattened_cov.contains_key(&(1252 as u64)) {
            let _ = black_box(1252 + 1);
        }
        if flattened_cov.contains_key(&(1253 as u64)) {
            let _ = black_box(1253 + 1);
        }
        if flattened_cov.contains_key(&(1254 as u64)) {
            let _ = black_box(1254 + 1);
        }
        if flattened_cov.contains_key(&(1255 as u64)) {
            let _ = black_box(1255 + 1);
        }
        if flattened_cov.contains_key(&(1256 as u64)) {
            let _ = black_box(1256 + 1);
        }
        if flattened_cov.contains_key(&(1257 as u64)) {
            let _ = black_box(1257 + 1);
        }
        if flattened_cov.contains_key(&(1258 as u64)) {
            let _ = black_box(1258 + 1);
        }
        if flattened_cov.contains_key(&(1259 as u64)) {
            let _ = black_box(1259 + 1);
        }
        if flattened_cov.contains_key(&(1260 as u64)) {
            let _ = black_box(1260 + 1);
        }
        if flattened_cov.contains_key(&(1261 as u64)) {
            let _ = black_box(1261 + 1);
        }
        if flattened_cov.contains_key(&(1262 as u64)) {
            let _ = black_box(1262 + 1);
        }
        if flattened_cov.contains_key(&(1263 as u64)) {
            let _ = black_box(1263 + 1);
        }
        if flattened_cov.contains_key(&(1264 as u64)) {
            let _ = black_box(1264 + 1);
        }
        if flattened_cov.contains_key(&(1265 as u64)) {
            let _ = black_box(1265 + 1);
        }
        if flattened_cov.contains_key(&(1266 as u64)) {
            let _ = black_box(1266 + 1);
        }
        if flattened_cov.contains_key(&(1267 as u64)) {
            let _ = black_box(1267 + 1);
        }
        if flattened_cov.contains_key(&(1268 as u64)) {
            let _ = black_box(1268 + 1);
        }
        if flattened_cov.contains_key(&(1269 as u64)) {
            let _ = black_box(1269 + 1);
        }
        if flattened_cov.contains_key(&(1270 as u64)) {
            let _ = black_box(1270 + 1);
        }
        if flattened_cov.contains_key(&(1271 as u64)) {
            let _ = black_box(1271 + 1);
        }
        if flattened_cov.contains_key(&(1272 as u64)) {
            let _ = black_box(1272 + 1);
        }
        if flattened_cov.contains_key(&(1273 as u64)) {
            let _ = black_box(1273 + 1);
        }
        if flattened_cov.contains_key(&(1274 as u64)) {
            let _ = black_box(1274 + 1);
        }
        if flattened_cov.contains_key(&(1275 as u64)) {
            let _ = black_box(1275 + 1);
        }
        if flattened_cov.contains_key(&(1276 as u64)) {
            let _ = black_box(1276 + 1);
        }
        if flattened_cov.contains_key(&(1277 as u64)) {
            let _ = black_box(1277 + 1);
        }
        if flattened_cov.contains_key(&(1278 as u64)) {
            let _ = black_box(1278 + 1);
        }
        if flattened_cov.contains_key(&(1279 as u64)) {
            let _ = black_box(1279 + 1);
        }
        if flattened_cov.contains_key(&(1280 as u64)) {
            let _ = black_box(1280 + 1);
        }
        if flattened_cov.contains_key(&(1281 as u64)) {
            let _ = black_box(1281 + 1);
        }
        if flattened_cov.contains_key(&(1282 as u64)) {
            let _ = black_box(1282 + 1);
        }
        if flattened_cov.contains_key(&(1283 as u64)) {
            let _ = black_box(1283 + 1);
        }
        if flattened_cov.contains_key(&(1284 as u64)) {
            let _ = black_box(1284 + 1);
        }
        if flattened_cov.contains_key(&(1285 as u64)) {
            let _ = black_box(1285 + 1);
        }
        if flattened_cov.contains_key(&(1286 as u64)) {
            let _ = black_box(1286 + 1);
        }
        if flattened_cov.contains_key(&(1287 as u64)) {
            let _ = black_box(1287 + 1);
        }
        if flattened_cov.contains_key(&(1288 as u64)) {
            let _ = black_box(1288 + 1);
        }
        if flattened_cov.contains_key(&(1289 as u64)) {
            let _ = black_box(1289 + 1);
        }
        if flattened_cov.contains_key(&(1290 as u64)) {
            let _ = black_box(1290 + 1);
        }
        if flattened_cov.contains_key(&(1291 as u64)) {
            let _ = black_box(1291 + 1);
        }
        if flattened_cov.contains_key(&(1292 as u64)) {
            let _ = black_box(1292 + 1);
        }
        if flattened_cov.contains_key(&(1293 as u64)) {
            let _ = black_box(1293 + 1);
        }
        if flattened_cov.contains_key(&(1294 as u64)) {
            let _ = black_box(1294 + 1);
        }
        if flattened_cov.contains_key(&(1295 as u64)) {
            let _ = black_box(1295 + 1);
        }
        if flattened_cov.contains_key(&(1296 as u64)) {
            let _ = black_box(1296 + 1);
        }
        if flattened_cov.contains_key(&(1297 as u64)) {
            let _ = black_box(1297 + 1);
        }
        if flattened_cov.contains_key(&(1298 as u64)) {
            let _ = black_box(1298 + 1);
        }
        if flattened_cov.contains_key(&(1299 as u64)) {
            let _ = black_box(1299 + 1);
        }
        if flattened_cov.contains_key(&(1300 as u64)) {
            let _ = black_box(1300 + 1);
        }
        if flattened_cov.contains_key(&(1301 as u64)) {
            let _ = black_box(1301 + 1);
        }
        if flattened_cov.contains_key(&(1302 as u64)) {
            let _ = black_box(1302 + 1);
        }
        if flattened_cov.contains_key(&(1303 as u64)) {
            let _ = black_box(1303 + 1);
        }
        if flattened_cov.contains_key(&(1304 as u64)) {
            let _ = black_box(1304 + 1);
        }
        if flattened_cov.contains_key(&(1305 as u64)) {
            let _ = black_box(1305 + 1);
        }
        if flattened_cov.contains_key(&(1306 as u64)) {
            let _ = black_box(1306 + 1);
        }
        if flattened_cov.contains_key(&(1307 as u64)) {
            let _ = black_box(1307 + 1);
        }
        if flattened_cov.contains_key(&(1308 as u64)) {
            let _ = black_box(1308 + 1);
        }
        if flattened_cov.contains_key(&(1309 as u64)) {
            let _ = black_box(1309 + 1);
        }
        if flattened_cov.contains_key(&(1310 as u64)) {
            let _ = black_box(1310 + 1);
        }
        if flattened_cov.contains_key(&(1311 as u64)) {
            let _ = black_box(1311 + 1);
        }
        if flattened_cov.contains_key(&(1312 as u64)) {
            let _ = black_box(1312 + 1);
        }
        if flattened_cov.contains_key(&(1313 as u64)) {
            let _ = black_box(1313 + 1);
        }
        if flattened_cov.contains_key(&(1314 as u64)) {
            let _ = black_box(1314 + 1);
        }
        if flattened_cov.contains_key(&(1315 as u64)) {
            let _ = black_box(1315 + 1);
        }
        if flattened_cov.contains_key(&(1316 as u64)) {
            let _ = black_box(1316 + 1);
        }
        if flattened_cov.contains_key(&(1317 as u64)) {
            let _ = black_box(1317 + 1);
        }
        if flattened_cov.contains_key(&(1318 as u64)) {
            let _ = black_box(1318 + 1);
        }
        if flattened_cov.contains_key(&(1319 as u64)) {
            let _ = black_box(1319 + 1);
        }
        if flattened_cov.contains_key(&(1320 as u64)) {
            let _ = black_box(1320 + 1);
        }
        if flattened_cov.contains_key(&(1321 as u64)) {
            let _ = black_box(1321 + 1);
        }
        if flattened_cov.contains_key(&(1322 as u64)) {
            let _ = black_box(1322 + 1);
        }
        if flattened_cov.contains_key(&(1323 as u64)) {
            let _ = black_box(1323 + 1);
        }
        if flattened_cov.contains_key(&(1324 as u64)) {
            let _ = black_box(1324 + 1);
        }
        if flattened_cov.contains_key(&(1325 as u64)) {
            let _ = black_box(1325 + 1);
        }
        if flattened_cov.contains_key(&(1326 as u64)) {
            let _ = black_box(1326 + 1);
        }
        if flattened_cov.contains_key(&(1327 as u64)) {
            let _ = black_box(1327 + 1);
        }
        if flattened_cov.contains_key(&(1328 as u64)) {
            let _ = black_box(1328 + 1);
        }
        if flattened_cov.contains_key(&(1329 as u64)) {
            let _ = black_box(1329 + 1);
        }
        if flattened_cov.contains_key(&(1330 as u64)) {
            let _ = black_box(1330 + 1);
        }
        if flattened_cov.contains_key(&(1331 as u64)) {
            let _ = black_box(1331 + 1);
        }
        if flattened_cov.contains_key(&(1332 as u64)) {
            let _ = black_box(1332 + 1);
        }
        if flattened_cov.contains_key(&(1333 as u64)) {
            let _ = black_box(1333 + 1);
        }
        if flattened_cov.contains_key(&(1334 as u64)) {
            let _ = black_box(1334 + 1);
        }
        if flattened_cov.contains_key(&(1335 as u64)) {
            let _ = black_box(1335 + 1);
        }
        if flattened_cov.contains_key(&(1336 as u64)) {
            let _ = black_box(1336 + 1);
        }
        if flattened_cov.contains_key(&(1337 as u64)) {
            let _ = black_box(1337 + 1);
        }
        if flattened_cov.contains_key(&(1338 as u64)) {
            let _ = black_box(1338 + 1);
        }
        if flattened_cov.contains_key(&(1339 as u64)) {
            let _ = black_box(1339 + 1);
        }
        if flattened_cov.contains_key(&(1340 as u64)) {
            let _ = black_box(1340 + 1);
        }
        if flattened_cov.contains_key(&(1341 as u64)) {
            let _ = black_box(1341 + 1);
        }
        if flattened_cov.contains_key(&(1342 as u64)) {
            let _ = black_box(1342 + 1);
        }
        if flattened_cov.contains_key(&(1343 as u64)) {
            let _ = black_box(1343 + 1);
        }
        if flattened_cov.contains_key(&(1344 as u64)) {
            let _ = black_box(1344 + 1);
        }
        if flattened_cov.contains_key(&(1345 as u64)) {
            let _ = black_box(1345 + 1);
        }
        if flattened_cov.contains_key(&(1346 as u64)) {
            let _ = black_box(1346 + 1);
        }
        if flattened_cov.contains_key(&(1347 as u64)) {
            let _ = black_box(1347 + 1);
        }
        if flattened_cov.contains_key(&(1348 as u64)) {
            let _ = black_box(1348 + 1);
        }
        if flattened_cov.contains_key(&(1349 as u64)) {
            let _ = black_box(1349 + 1);
        }
        if flattened_cov.contains_key(&(1350 as u64)) {
            let _ = black_box(1350 + 1);
        }
        if flattened_cov.contains_key(&(1351 as u64)) {
            let _ = black_box(1351 + 1);
        }
        if flattened_cov.contains_key(&(1352 as u64)) {
            let _ = black_box(1352 + 1);
        }
        if flattened_cov.contains_key(&(1353 as u64)) {
            let _ = black_box(1353 + 1);
        }
        if flattened_cov.contains_key(&(1354 as u64)) {
            let _ = black_box(1354 + 1);
        }
        if flattened_cov.contains_key(&(1355 as u64)) {
            let _ = black_box(1355 + 1);
        }
        if flattened_cov.contains_key(&(1356 as u64)) {
            let _ = black_box(1356 + 1);
        }
        if flattened_cov.contains_key(&(1357 as u64)) {
            let _ = black_box(1357 + 1);
        }
        if flattened_cov.contains_key(&(1358 as u64)) {
            let _ = black_box(1358 + 1);
        }
        if flattened_cov.contains_key(&(1359 as u64)) {
            let _ = black_box(1359 + 1);
        }
        if flattened_cov.contains_key(&(1360 as u64)) {
            let _ = black_box(1360 + 1);
        }
        if flattened_cov.contains_key(&(1361 as u64)) {
            let _ = black_box(1361 + 1);
        }
        if flattened_cov.contains_key(&(1362 as u64)) {
            let _ = black_box(1362 + 1);
        }
        if flattened_cov.contains_key(&(1363 as u64)) {
            let _ = black_box(1363 + 1);
        }
        if flattened_cov.contains_key(&(1364 as u64)) {
            let _ = black_box(1364 + 1);
        }
        if flattened_cov.contains_key(&(1365 as u64)) {
            let _ = black_box(1365 + 1);
        }
        if flattened_cov.contains_key(&(1366 as u64)) {
            let _ = black_box(1366 + 1);
        }
        if flattened_cov.contains_key(&(1367 as u64)) {
            let _ = black_box(1367 + 1);
        }
        if flattened_cov.contains_key(&(1368 as u64)) {
            let _ = black_box(1368 + 1);
        }
        if flattened_cov.contains_key(&(1369 as u64)) {
            let _ = black_box(1369 + 1);
        }
        if flattened_cov.contains_key(&(1370 as u64)) {
            let _ = black_box(1370 + 1);
        }
        if flattened_cov.contains_key(&(1371 as u64)) {
            let _ = black_box(1371 + 1);
        }
        if flattened_cov.contains_key(&(1372 as u64)) {
            let _ = black_box(1372 + 1);
        }
        if flattened_cov.contains_key(&(1373 as u64)) {
            let _ = black_box(1373 + 1);
        }
        if flattened_cov.contains_key(&(1374 as u64)) {
            let _ = black_box(1374 + 1);
        }
        if flattened_cov.contains_key(&(1375 as u64)) {
            let _ = black_box(1375 + 1);
        }
        if flattened_cov.contains_key(&(1376 as u64)) {
            let _ = black_box(1376 + 1);
        }
        if flattened_cov.contains_key(&(1377 as u64)) {
            let _ = black_box(1377 + 1);
        }
        if flattened_cov.contains_key(&(1378 as u64)) {
            let _ = black_box(1378 + 1);
        }
        if flattened_cov.contains_key(&(1379 as u64)) {
            let _ = black_box(1379 + 1);
        }
        if flattened_cov.contains_key(&(1380 as u64)) {
            let _ = black_box(1380 + 1);
        }
        if flattened_cov.contains_key(&(1381 as u64)) {
            let _ = black_box(1381 + 1);
        }
        if flattened_cov.contains_key(&(1382 as u64)) {
            let _ = black_box(1382 + 1);
        }
        if flattened_cov.contains_key(&(1383 as u64)) {
            let _ = black_box(1383 + 1);
        }
        if flattened_cov.contains_key(&(1384 as u64)) {
            let _ = black_box(1384 + 1);
        }
        if flattened_cov.contains_key(&(1385 as u64)) {
            let _ = black_box(1385 + 1);
        }
        if flattened_cov.contains_key(&(1386 as u64)) {
            let _ = black_box(1386 + 1);
        }
        if flattened_cov.contains_key(&(1387 as u64)) {
            let _ = black_box(1387 + 1);
        }
        if flattened_cov.contains_key(&(1388 as u64)) {
            let _ = black_box(1388 + 1);
        }
        if flattened_cov.contains_key(&(1389 as u64)) {
            let _ = black_box(1389 + 1);
        }
        if flattened_cov.contains_key(&(1390 as u64)) {
            let _ = black_box(1390 + 1);
        }
        if flattened_cov.contains_key(&(1391 as u64)) {
            let _ = black_box(1391 + 1);
        }
        if flattened_cov.contains_key(&(1392 as u64)) {
            let _ = black_box(1392 + 1);
        }
        if flattened_cov.contains_key(&(1393 as u64)) {
            let _ = black_box(1393 + 1);
        }
        if flattened_cov.contains_key(&(1394 as u64)) {
            let _ = black_box(1394 + 1);
        }
        if flattened_cov.contains_key(&(1395 as u64)) {
            let _ = black_box(1395 + 1);
        }
        if flattened_cov.contains_key(&(1396 as u64)) {
            let _ = black_box(1396 + 1);
        }
        if flattened_cov.contains_key(&(1397 as u64)) {
            let _ = black_box(1397 + 1);
        }
        if flattened_cov.contains_key(&(1398 as u64)) {
            let _ = black_box(1398 + 1);
        }
        if flattened_cov.contains_key(&(1399 as u64)) {
            let _ = black_box(1399 + 1);
        }
        if flattened_cov.contains_key(&(1400 as u64)) {
            let _ = black_box(1400 + 1);
        }
        if flattened_cov.contains_key(&(1401 as u64)) {
            let _ = black_box(1401 + 1);
        }
        if flattened_cov.contains_key(&(1402 as u64)) {
            let _ = black_box(1402 + 1);
        }
        if flattened_cov.contains_key(&(1403 as u64)) {
            let _ = black_box(1403 + 1);
        }
        if flattened_cov.contains_key(&(1404 as u64)) {
            let _ = black_box(1404 + 1);
        }
        if flattened_cov.contains_key(&(1405 as u64)) {
            let _ = black_box(1405 + 1);
        }
        if flattened_cov.contains_key(&(1406 as u64)) {
            let _ = black_box(1406 + 1);
        }
        if flattened_cov.contains_key(&(1407 as u64)) {
            let _ = black_box(1407 + 1);
        }
        if flattened_cov.contains_key(&(1408 as u64)) {
            let _ = black_box(1408 + 1);
        }
        if flattened_cov.contains_key(&(1409 as u64)) {
            let _ = black_box(1409 + 1);
        }
        if flattened_cov.contains_key(&(1410 as u64)) {
            let _ = black_box(1410 + 1);
        }
        if flattened_cov.contains_key(&(1411 as u64)) {
            let _ = black_box(1411 + 1);
        }
        if flattened_cov.contains_key(&(1412 as u64)) {
            let _ = black_box(1412 + 1);
        }
        if flattened_cov.contains_key(&(1413 as u64)) {
            let _ = black_box(1413 + 1);
        }
        if flattened_cov.contains_key(&(1414 as u64)) {
            let _ = black_box(1414 + 1);
        }
        if flattened_cov.contains_key(&(1415 as u64)) {
            let _ = black_box(1415 + 1);
        }
        if flattened_cov.contains_key(&(1416 as u64)) {
            let _ = black_box(1416 + 1);
        }
        if flattened_cov.contains_key(&(1417 as u64)) {
            let _ = black_box(1417 + 1);
        }
        if flattened_cov.contains_key(&(1418 as u64)) {
            let _ = black_box(1418 + 1);
        }
        if flattened_cov.contains_key(&(1419 as u64)) {
            let _ = black_box(1419 + 1);
        }
        if flattened_cov.contains_key(&(1420 as u64)) {
            let _ = black_box(1420 + 1);
        }
        if flattened_cov.contains_key(&(1421 as u64)) {
            let _ = black_box(1421 + 1);
        }
        if flattened_cov.contains_key(&(1422 as u64)) {
            let _ = black_box(1422 + 1);
        }
        if flattened_cov.contains_key(&(1423 as u64)) {
            let _ = black_box(1423 + 1);
        }
        if flattened_cov.contains_key(&(1424 as u64)) {
            let _ = black_box(1424 + 1);
        }
        if flattened_cov.contains_key(&(1425 as u64)) {
            let _ = black_box(1425 + 1);
        }
        if flattened_cov.contains_key(&(1426 as u64)) {
            let _ = black_box(1426 + 1);
        }
        if flattened_cov.contains_key(&(1427 as u64)) {
            let _ = black_box(1427 + 1);
        }
        if flattened_cov.contains_key(&(1428 as u64)) {
            let _ = black_box(1428 + 1);
        }
        if flattened_cov.contains_key(&(1429 as u64)) {
            let _ = black_box(1429 + 1);
        }
        if flattened_cov.contains_key(&(1430 as u64)) {
            let _ = black_box(1430 + 1);
        }
        if flattened_cov.contains_key(&(1431 as u64)) {
            let _ = black_box(1431 + 1);
        }
        if flattened_cov.contains_key(&(1432 as u64)) {
            let _ = black_box(1432 + 1);
        }
        if flattened_cov.contains_key(&(1433 as u64)) {
            let _ = black_box(1433 + 1);
        }
        if flattened_cov.contains_key(&(1434 as u64)) {
            let _ = black_box(1434 + 1);
        }
        if flattened_cov.contains_key(&(1435 as u64)) {
            let _ = black_box(1435 + 1);
        }
        if flattened_cov.contains_key(&(1436 as u64)) {
            let _ = black_box(1436 + 1);
        }
        if flattened_cov.contains_key(&(1437 as u64)) {
            let _ = black_box(1437 + 1);
        }
        if flattened_cov.contains_key(&(1438 as u64)) {
            let _ = black_box(1438 + 1);
        }
        if flattened_cov.contains_key(&(1439 as u64)) {
            let _ = black_box(1439 + 1);
        }
        if flattened_cov.contains_key(&(1440 as u64)) {
            let _ = black_box(1440 + 1);
        }
        if flattened_cov.contains_key(&(1441 as u64)) {
            let _ = black_box(1441 + 1);
        }
        if flattened_cov.contains_key(&(1442 as u64)) {
            let _ = black_box(1442 + 1);
        }
        if flattened_cov.contains_key(&(1443 as u64)) {
            let _ = black_box(1443 + 1);
        }
        if flattened_cov.contains_key(&(1444 as u64)) {
            let _ = black_box(1444 + 1);
        }
        if flattened_cov.contains_key(&(1445 as u64)) {
            let _ = black_box(1445 + 1);
        }
        if flattened_cov.contains_key(&(1446 as u64)) {
            let _ = black_box(1446 + 1);
        }
        if flattened_cov.contains_key(&(1447 as u64)) {
            let _ = black_box(1447 + 1);
        }
        if flattened_cov.contains_key(&(1448 as u64)) {
            let _ = black_box(1448 + 1);
        }
        if flattened_cov.contains_key(&(1449 as u64)) {
            let _ = black_box(1449 + 1);
        }
        if flattened_cov.contains_key(&(1450 as u64)) {
            let _ = black_box(1450 + 1);
        }
        if flattened_cov.contains_key(&(1451 as u64)) {
            let _ = black_box(1451 + 1);
        }
        if flattened_cov.contains_key(&(1452 as u64)) {
            let _ = black_box(1452 + 1);
        }
        if flattened_cov.contains_key(&(1453 as u64)) {
            let _ = black_box(1453 + 1);
        }
        if flattened_cov.contains_key(&(1454 as u64)) {
            let _ = black_box(1454 + 1);
        }
        if flattened_cov.contains_key(&(1455 as u64)) {
            let _ = black_box(1455 + 1);
        }
        if flattened_cov.contains_key(&(1456 as u64)) {
            let _ = black_box(1456 + 1);
        }
        if flattened_cov.contains_key(&(1457 as u64)) {
            let _ = black_box(1457 + 1);
        }
        if flattened_cov.contains_key(&(1458 as u64)) {
            let _ = black_box(1458 + 1);
        }
        if flattened_cov.contains_key(&(1459 as u64)) {
            let _ = black_box(1459 + 1);
        }
        if flattened_cov.contains_key(&(1460 as u64)) {
            let _ = black_box(1460 + 1);
        }
        if flattened_cov.contains_key(&(1461 as u64)) {
            let _ = black_box(1461 + 1);
        }
        if flattened_cov.contains_key(&(1462 as u64)) {
            let _ = black_box(1462 + 1);
        }
        if flattened_cov.contains_key(&(1463 as u64)) {
            let _ = black_box(1463 + 1);
        }
        if flattened_cov.contains_key(&(1464 as u64)) {
            let _ = black_box(1464 + 1);
        }
        if flattened_cov.contains_key(&(1465 as u64)) {
            let _ = black_box(1465 + 1);
        }
        if flattened_cov.contains_key(&(1466 as u64)) {
            let _ = black_box(1466 + 1);
        }
        if flattened_cov.contains_key(&(1467 as u64)) {
            let _ = black_box(1467 + 1);
        }
        if flattened_cov.contains_key(&(1468 as u64)) {
            let _ = black_box(1468 + 1);
        }
        if flattened_cov.contains_key(&(1469 as u64)) {
            let _ = black_box(1469 + 1);
        }
        if flattened_cov.contains_key(&(1470 as u64)) {
            let _ = black_box(1470 + 1);
        }
        if flattened_cov.contains_key(&(1471 as u64)) {
            let _ = black_box(1471 + 1);
        }
        if flattened_cov.contains_key(&(1472 as u64)) {
            let _ = black_box(1472 + 1);
        }
        if flattened_cov.contains_key(&(1473 as u64)) {
            let _ = black_box(1473 + 1);
        }
        if flattened_cov.contains_key(&(1474 as u64)) {
            let _ = black_box(1474 + 1);
        }
        if flattened_cov.contains_key(&(1475 as u64)) {
            let _ = black_box(1475 + 1);
        }
        if flattened_cov.contains_key(&(1476 as u64)) {
            let _ = black_box(1476 + 1);
        }
        if flattened_cov.contains_key(&(1477 as u64)) {
            let _ = black_box(1477 + 1);
        }
        if flattened_cov.contains_key(&(1478 as u64)) {
            let _ = black_box(1478 + 1);
        }
        if flattened_cov.contains_key(&(1479 as u64)) {
            let _ = black_box(1479 + 1);
        }
        if flattened_cov.contains_key(&(1480 as u64)) {
            let _ = black_box(1480 + 1);
        }
        if flattened_cov.contains_key(&(1481 as u64)) {
            let _ = black_box(1481 + 1);
        }
        if flattened_cov.contains_key(&(1482 as u64)) {
            let _ = black_box(1482 + 1);
        }
        if flattened_cov.contains_key(&(1483 as u64)) {
            let _ = black_box(1483 + 1);
        }
        if flattened_cov.contains_key(&(1484 as u64)) {
            let _ = black_box(1484 + 1);
        }
        if flattened_cov.contains_key(&(1485 as u64)) {
            let _ = black_box(1485 + 1);
        }
        if flattened_cov.contains_key(&(1486 as u64)) {
            let _ = black_box(1486 + 1);
        }
        if flattened_cov.contains_key(&(1487 as u64)) {
            let _ = black_box(1487 + 1);
        }
        if flattened_cov.contains_key(&(1488 as u64)) {
            let _ = black_box(1488 + 1);
        }
        if flattened_cov.contains_key(&(1489 as u64)) {
            let _ = black_box(1489 + 1);
        }
        if flattened_cov.contains_key(&(1490 as u64)) {
            let _ = black_box(1490 + 1);
        }
        if flattened_cov.contains_key(&(1491 as u64)) {
            let _ = black_box(1491 + 1);
        }
        if flattened_cov.contains_key(&(1492 as u64)) {
            let _ = black_box(1492 + 1);
        }
        if flattened_cov.contains_key(&(1493 as u64)) {
            let _ = black_box(1493 + 1);
        }
        if flattened_cov.contains_key(&(1494 as u64)) {
            let _ = black_box(1494 + 1);
        }
        if flattened_cov.contains_key(&(1495 as u64)) {
            let _ = black_box(1495 + 1);
        }
        if flattened_cov.contains_key(&(1496 as u64)) {
            let _ = black_box(1496 + 1);
        }
        if flattened_cov.contains_key(&(1497 as u64)) {
            let _ = black_box(1497 + 1);
        }
        if flattened_cov.contains_key(&(1498 as u64)) {
            let _ = black_box(1498 + 1);
        }
        if flattened_cov.contains_key(&(1499 as u64)) {
            let _ = black_box(1499 + 1);
        }
        if flattened_cov.contains_key(&(1500 as u64)) {
            let _ = black_box(1500 + 1);
        }
        if flattened_cov.contains_key(&(1501 as u64)) {
            let _ = black_box(1501 + 1);
        }
        if flattened_cov.contains_key(&(1502 as u64)) {
            let _ = black_box(1502 + 1);
        }
        if flattened_cov.contains_key(&(1503 as u64)) {
            let _ = black_box(1503 + 1);
        }
        if flattened_cov.contains_key(&(1504 as u64)) {
            let _ = black_box(1504 + 1);
        }
        if flattened_cov.contains_key(&(1505 as u64)) {
            let _ = black_box(1505 + 1);
        }
        if flattened_cov.contains_key(&(1506 as u64)) {
            let _ = black_box(1506 + 1);
        }
        if flattened_cov.contains_key(&(1507 as u64)) {
            let _ = black_box(1507 + 1);
        }
        if flattened_cov.contains_key(&(1508 as u64)) {
            let _ = black_box(1508 + 1);
        }
        if flattened_cov.contains_key(&(1509 as u64)) {
            let _ = black_box(1509 + 1);
        }
        if flattened_cov.contains_key(&(1510 as u64)) {
            let _ = black_box(1510 + 1);
        }
        if flattened_cov.contains_key(&(1511 as u64)) {
            let _ = black_box(1511 + 1);
        }
        if flattened_cov.contains_key(&(1512 as u64)) {
            let _ = black_box(1512 + 1);
        }
        if flattened_cov.contains_key(&(1513 as u64)) {
            let _ = black_box(1513 + 1);
        }
        if flattened_cov.contains_key(&(1514 as u64)) {
            let _ = black_box(1514 + 1);
        }
        if flattened_cov.contains_key(&(1515 as u64)) {
            let _ = black_box(1515 + 1);
        }
        if flattened_cov.contains_key(&(1516 as u64)) {
            let _ = black_box(1516 + 1);
        }
        if flattened_cov.contains_key(&(1517 as u64)) {
            let _ = black_box(1517 + 1);
        }
        if flattened_cov.contains_key(&(1518 as u64)) {
            let _ = black_box(1518 + 1);
        }
        if flattened_cov.contains_key(&(1519 as u64)) {
            let _ = black_box(1519 + 1);
        }
        if flattened_cov.contains_key(&(1520 as u64)) {
            let _ = black_box(1520 + 1);
        }
        if flattened_cov.contains_key(&(1521 as u64)) {
            let _ = black_box(1521 + 1);
        }
        if flattened_cov.contains_key(&(1522 as u64)) {
            let _ = black_box(1522 + 1);
        }
        if flattened_cov.contains_key(&(1523 as u64)) {
            let _ = black_box(1523 + 1);
        }
        if flattened_cov.contains_key(&(1524 as u64)) {
            let _ = black_box(1524 + 1);
        }
        if flattened_cov.contains_key(&(1525 as u64)) {
            let _ = black_box(1525 + 1);
        }
        if flattened_cov.contains_key(&(1526 as u64)) {
            let _ = black_box(1526 + 1);
        }
        if flattened_cov.contains_key(&(1527 as u64)) {
            let _ = black_box(1527 + 1);
        }
        if flattened_cov.contains_key(&(1528 as u64)) {
            let _ = black_box(1528 + 1);
        }
        if flattened_cov.contains_key(&(1529 as u64)) {
            let _ = black_box(1529 + 1);
        }
        if flattened_cov.contains_key(&(1530 as u64)) {
            let _ = black_box(1530 + 1);
        }
        if flattened_cov.contains_key(&(1531 as u64)) {
            let _ = black_box(1531 + 1);
        }
        if flattened_cov.contains_key(&(1532 as u64)) {
            let _ = black_box(1532 + 1);
        }
        if flattened_cov.contains_key(&(1533 as u64)) {
            let _ = black_box(1533 + 1);
        }
        if flattened_cov.contains_key(&(1534 as u64)) {
            let _ = black_box(1534 + 1);
        }
        if flattened_cov.contains_key(&(1535 as u64)) {
            let _ = black_box(1535 + 1);
        }
        if flattened_cov.contains_key(&(1536 as u64)) {
            let _ = black_box(1536 + 1);
        }
        if flattened_cov.contains_key(&(1537 as u64)) {
            let _ = black_box(1537 + 1);
        }
        if flattened_cov.contains_key(&(1538 as u64)) {
            let _ = black_box(1538 + 1);
        }
        if flattened_cov.contains_key(&(1539 as u64)) {
            let _ = black_box(1539 + 1);
        }
        if flattened_cov.contains_key(&(1540 as u64)) {
            let _ = black_box(1540 + 1);
        }
        if flattened_cov.contains_key(&(1541 as u64)) {
            let _ = black_box(1541 + 1);
        }
        if flattened_cov.contains_key(&(1542 as u64)) {
            let _ = black_box(1542 + 1);
        }
        if flattened_cov.contains_key(&(1543 as u64)) {
            let _ = black_box(1543 + 1);
        }
        if flattened_cov.contains_key(&(1544 as u64)) {
            let _ = black_box(1544 + 1);
        }
        if flattened_cov.contains_key(&(1545 as u64)) {
            let _ = black_box(1545 + 1);
        }
        if flattened_cov.contains_key(&(1546 as u64)) {
            let _ = black_box(1546 + 1);
        }
        if flattened_cov.contains_key(&(1547 as u64)) {
            let _ = black_box(1547 + 1);
        }
        if flattened_cov.contains_key(&(1548 as u64)) {
            let _ = black_box(1548 + 1);
        }
        if flattened_cov.contains_key(&(1549 as u64)) {
            let _ = black_box(1549 + 1);
        }
        if flattened_cov.contains_key(&(1550 as u64)) {
            let _ = black_box(1550 + 1);
        }
        if flattened_cov.contains_key(&(1551 as u64)) {
            let _ = black_box(1551 + 1);
        }
        if flattened_cov.contains_key(&(1552 as u64)) {
            let _ = black_box(1552 + 1);
        }
        if flattened_cov.contains_key(&(1553 as u64)) {
            let _ = black_box(1553 + 1);
        }
        if flattened_cov.contains_key(&(1554 as u64)) {
            let _ = black_box(1554 + 1);
        }
        if flattened_cov.contains_key(&(1555 as u64)) {
            let _ = black_box(1555 + 1);
        }
        if flattened_cov.contains_key(&(1556 as u64)) {
            let _ = black_box(1556 + 1);
        }
        if flattened_cov.contains_key(&(1557 as u64)) {
            let _ = black_box(1557 + 1);
        }
        if flattened_cov.contains_key(&(1558 as u64)) {
            let _ = black_box(1558 + 1);
        }
        if flattened_cov.contains_key(&(1559 as u64)) {
            let _ = black_box(1559 + 1);
        }
        if flattened_cov.contains_key(&(1560 as u64)) {
            let _ = black_box(1560 + 1);
        }
        if flattened_cov.contains_key(&(1561 as u64)) {
            let _ = black_box(1561 + 1);
        }
        if flattened_cov.contains_key(&(1562 as u64)) {
            let _ = black_box(1562 + 1);
        }
        if flattened_cov.contains_key(&(1563 as u64)) {
            let _ = black_box(1563 + 1);
        }
        if flattened_cov.contains_key(&(1564 as u64)) {
            let _ = black_box(1564 + 1);
        }
        if flattened_cov.contains_key(&(1565 as u64)) {
            let _ = black_box(1565 + 1);
        }
        if flattened_cov.contains_key(&(1566 as u64)) {
            let _ = black_box(1566 + 1);
        }
        if flattened_cov.contains_key(&(1567 as u64)) {
            let _ = black_box(1567 + 1);
        }
        if flattened_cov.contains_key(&(1568 as u64)) {
            let _ = black_box(1568 + 1);
        }
        if flattened_cov.contains_key(&(1569 as u64)) {
            let _ = black_box(1569 + 1);
        }
        if flattened_cov.contains_key(&(1570 as u64)) {
            let _ = black_box(1570 + 1);
        }
        if flattened_cov.contains_key(&(1571 as u64)) {
            let _ = black_box(1571 + 1);
        }
        if flattened_cov.contains_key(&(1572 as u64)) {
            let _ = black_box(1572 + 1);
        }
        if flattened_cov.contains_key(&(1573 as u64)) {
            let _ = black_box(1573 + 1);
        }
        if flattened_cov.contains_key(&(1574 as u64)) {
            let _ = black_box(1574 + 1);
        }
        if flattened_cov.contains_key(&(1575 as u64)) {
            let _ = black_box(1575 + 1);
        }
        if flattened_cov.contains_key(&(1576 as u64)) {
            let _ = black_box(1576 + 1);
        }
        if flattened_cov.contains_key(&(1577 as u64)) {
            let _ = black_box(1577 + 1);
        }
        if flattened_cov.contains_key(&(1578 as u64)) {
            let _ = black_box(1578 + 1);
        }
        if flattened_cov.contains_key(&(1579 as u64)) {
            let _ = black_box(1579 + 1);
        }
        if flattened_cov.contains_key(&(1580 as u64)) {
            let _ = black_box(1580 + 1);
        }
        if flattened_cov.contains_key(&(1581 as u64)) {
            let _ = black_box(1581 + 1);
        }
        if flattened_cov.contains_key(&(1582 as u64)) {
            let _ = black_box(1582 + 1);
        }
        if flattened_cov.contains_key(&(1583 as u64)) {
            let _ = black_box(1583 + 1);
        }
        if flattened_cov.contains_key(&(1584 as u64)) {
            let _ = black_box(1584 + 1);
        }
        if flattened_cov.contains_key(&(1585 as u64)) {
            let _ = black_box(1585 + 1);
        }
        if flattened_cov.contains_key(&(1586 as u64)) {
            let _ = black_box(1586 + 1);
        }
        if flattened_cov.contains_key(&(1587 as u64)) {
            let _ = black_box(1587 + 1);
        }
        if flattened_cov.contains_key(&(1588 as u64)) {
            let _ = black_box(1588 + 1);
        }
        if flattened_cov.contains_key(&(1589 as u64)) {
            let _ = black_box(1589 + 1);
        }
        if flattened_cov.contains_key(&(1590 as u64)) {
            let _ = black_box(1590 + 1);
        }
        if flattened_cov.contains_key(&(1591 as u64)) {
            let _ = black_box(1591 + 1);
        }
        if flattened_cov.contains_key(&(1592 as u64)) {
            let _ = black_box(1592 + 1);
        }
        if flattened_cov.contains_key(&(1593 as u64)) {
            let _ = black_box(1593 + 1);
        }
        if flattened_cov.contains_key(&(1594 as u64)) {
            let _ = black_box(1594 + 1);
        }
        if flattened_cov.contains_key(&(1595 as u64)) {
            let _ = black_box(1595 + 1);
        }
        if flattened_cov.contains_key(&(1596 as u64)) {
            let _ = black_box(1596 + 1);
        }
        if flattened_cov.contains_key(&(1597 as u64)) {
            let _ = black_box(1597 + 1);
        }
        if flattened_cov.contains_key(&(1598 as u64)) {
            let _ = black_box(1598 + 1);
        }
        if flattened_cov.contains_key(&(1599 as u64)) {
            let _ = black_box(1599 + 1);
        }
        if flattened_cov.contains_key(&(1600 as u64)) {
            let _ = black_box(1600 + 1);
        }
        if flattened_cov.contains_key(&(1601 as u64)) {
            let _ = black_box(1601 + 1);
        }
        if flattened_cov.contains_key(&(1602 as u64)) {
            let _ = black_box(1602 + 1);
        }
        if flattened_cov.contains_key(&(1603 as u64)) {
            let _ = black_box(1603 + 1);
        }
        if flattened_cov.contains_key(&(1604 as u64)) {
            let _ = black_box(1604 + 1);
        }
        if flattened_cov.contains_key(&(1605 as u64)) {
            let _ = black_box(1605 + 1);
        }
        if flattened_cov.contains_key(&(1606 as u64)) {
            let _ = black_box(1606 + 1);
        }
        if flattened_cov.contains_key(&(1607 as u64)) {
            let _ = black_box(1607 + 1);
        }
        if flattened_cov.contains_key(&(1608 as u64)) {
            let _ = black_box(1608 + 1);
        }
        if flattened_cov.contains_key(&(1609 as u64)) {
            let _ = black_box(1609 + 1);
        }
        if flattened_cov.contains_key(&(1610 as u64)) {
            let _ = black_box(1610 + 1);
        }
        if flattened_cov.contains_key(&(1611 as u64)) {
            let _ = black_box(1611 + 1);
        }
        if flattened_cov.contains_key(&(1612 as u64)) {
            let _ = black_box(1612 + 1);
        }
        if flattened_cov.contains_key(&(1613 as u64)) {
            let _ = black_box(1613 + 1);
        }
        if flattened_cov.contains_key(&(1614 as u64)) {
            let _ = black_box(1614 + 1);
        }
        if flattened_cov.contains_key(&(1615 as u64)) {
            let _ = black_box(1615 + 1);
        }
        if flattened_cov.contains_key(&(1616 as u64)) {
            let _ = black_box(1616 + 1);
        }
        if flattened_cov.contains_key(&(1617 as u64)) {
            let _ = black_box(1617 + 1);
        }
        if flattened_cov.contains_key(&(1618 as u64)) {
            let _ = black_box(1618 + 1);
        }
        if flattened_cov.contains_key(&(1619 as u64)) {
            let _ = black_box(1619 + 1);
        }
        if flattened_cov.contains_key(&(1620 as u64)) {
            let _ = black_box(1620 + 1);
        }
        if flattened_cov.contains_key(&(1621 as u64)) {
            let _ = black_box(1621 + 1);
        }
        if flattened_cov.contains_key(&(1622 as u64)) {
            let _ = black_box(1622 + 1);
        }
        if flattened_cov.contains_key(&(1623 as u64)) {
            let _ = black_box(1623 + 1);
        }
        if flattened_cov.contains_key(&(1624 as u64)) {
            let _ = black_box(1624 + 1);
        }
        if flattened_cov.contains_key(&(1625 as u64)) {
            let _ = black_box(1625 + 1);
        }
        if flattened_cov.contains_key(&(1626 as u64)) {
            let _ = black_box(1626 + 1);
        }
        if flattened_cov.contains_key(&(1627 as u64)) {
            let _ = black_box(1627 + 1);
        }
        if flattened_cov.contains_key(&(1628 as u64)) {
            let _ = black_box(1628 + 1);
        }
        if flattened_cov.contains_key(&(1629 as u64)) {
            let _ = black_box(1629 + 1);
        }
        if flattened_cov.contains_key(&(1630 as u64)) {
            let _ = black_box(1630 + 1);
        }
        if flattened_cov.contains_key(&(1631 as u64)) {
            let _ = black_box(1631 + 1);
        }
        if flattened_cov.contains_key(&(1632 as u64)) {
            let _ = black_box(1632 + 1);
        }
        if flattened_cov.contains_key(&(1633 as u64)) {
            let _ = black_box(1633 + 1);
        }
        if flattened_cov.contains_key(&(1634 as u64)) {
            let _ = black_box(1634 + 1);
        }
        if flattened_cov.contains_key(&(1635 as u64)) {
            let _ = black_box(1635 + 1);
        }
        if flattened_cov.contains_key(&(1636 as u64)) {
            let _ = black_box(1636 + 1);
        }
        if flattened_cov.contains_key(&(1637 as u64)) {
            let _ = black_box(1637 + 1);
        }
        if flattened_cov.contains_key(&(1638 as u64)) {
            let _ = black_box(1638 + 1);
        }
        if flattened_cov.contains_key(&(1639 as u64)) {
            let _ = black_box(1639 + 1);
        }
        if flattened_cov.contains_key(&(1640 as u64)) {
            let _ = black_box(1640 + 1);
        }
        if flattened_cov.contains_key(&(1641 as u64)) {
            let _ = black_box(1641 + 1);
        }
        if flattened_cov.contains_key(&(1642 as u64)) {
            let _ = black_box(1642 + 1);
        }
        if flattened_cov.contains_key(&(1643 as u64)) {
            let _ = black_box(1643 + 1);
        }
        if flattened_cov.contains_key(&(1644 as u64)) {
            let _ = black_box(1644 + 1);
        }
        if flattened_cov.contains_key(&(1645 as u64)) {
            let _ = black_box(1645 + 1);
        }
        if flattened_cov.contains_key(&(1646 as u64)) {
            let _ = black_box(1646 + 1);
        }
        if flattened_cov.contains_key(&(1647 as u64)) {
            let _ = black_box(1647 + 1);
        }
        if flattened_cov.contains_key(&(1648 as u64)) {
            let _ = black_box(1648 + 1);
        }
        if flattened_cov.contains_key(&(1649 as u64)) {
            let _ = black_box(1649 + 1);
        }
        if flattened_cov.contains_key(&(1650 as u64)) {
            let _ = black_box(1650 + 1);
        }
        if flattened_cov.contains_key(&(1651 as u64)) {
            let _ = black_box(1651 + 1);
        }
        if flattened_cov.contains_key(&(1652 as u64)) {
            let _ = black_box(1652 + 1);
        }
        if flattened_cov.contains_key(&(1653 as u64)) {
            let _ = black_box(1653 + 1);
        }
        if flattened_cov.contains_key(&(1654 as u64)) {
            let _ = black_box(1654 + 1);
        }
        if flattened_cov.contains_key(&(1655 as u64)) {
            let _ = black_box(1655 + 1);
        }
        if flattened_cov.contains_key(&(1656 as u64)) {
            let _ = black_box(1656 + 1);
        }
        if flattened_cov.contains_key(&(1657 as u64)) {
            let _ = black_box(1657 + 1);
        }
        if flattened_cov.contains_key(&(1658 as u64)) {
            let _ = black_box(1658 + 1);
        }
        if flattened_cov.contains_key(&(1659 as u64)) {
            let _ = black_box(1659 + 1);
        }
        if flattened_cov.contains_key(&(1660 as u64)) {
            let _ = black_box(1660 + 1);
        }
        if flattened_cov.contains_key(&(1661 as u64)) {
            let _ = black_box(1661 + 1);
        }
        if flattened_cov.contains_key(&(1662 as u64)) {
            let _ = black_box(1662 + 1);
        }
        if flattened_cov.contains_key(&(1663 as u64)) {
            let _ = black_box(1663 + 1);
        }
        if flattened_cov.contains_key(&(1664 as u64)) {
            let _ = black_box(1664 + 1);
        }
        if flattened_cov.contains_key(&(1665 as u64)) {
            let _ = black_box(1665 + 1);
        }
        if flattened_cov.contains_key(&(1666 as u64)) {
            let _ = black_box(1666 + 1);
        }
        if flattened_cov.contains_key(&(1667 as u64)) {
            let _ = black_box(1667 + 1);
        }
        if flattened_cov.contains_key(&(1668 as u64)) {
            let _ = black_box(1668 + 1);
        }
        if flattened_cov.contains_key(&(1669 as u64)) {
            let _ = black_box(1669 + 1);
        }
        if flattened_cov.contains_key(&(1670 as u64)) {
            let _ = black_box(1670 + 1);
        }
        if flattened_cov.contains_key(&(1671 as u64)) {
            let _ = black_box(1671 + 1);
        }
        if flattened_cov.contains_key(&(1672 as u64)) {
            let _ = black_box(1672 + 1);
        }
        if flattened_cov.contains_key(&(1673 as u64)) {
            let _ = black_box(1673 + 1);
        }
        if flattened_cov.contains_key(&(1674 as u64)) {
            let _ = black_box(1674 + 1);
        }
        if flattened_cov.contains_key(&(1675 as u64)) {
            let _ = black_box(1675 + 1);
        }
        if flattened_cov.contains_key(&(1676 as u64)) {
            let _ = black_box(1676 + 1);
        }
        if flattened_cov.contains_key(&(1677 as u64)) {
            let _ = black_box(1677 + 1);
        }
        if flattened_cov.contains_key(&(1678 as u64)) {
            let _ = black_box(1678 + 1);
        }
        if flattened_cov.contains_key(&(1679 as u64)) {
            let _ = black_box(1679 + 1);
        }
        if flattened_cov.contains_key(&(1680 as u64)) {
            let _ = black_box(1680 + 1);
        }
        if flattened_cov.contains_key(&(1681 as u64)) {
            let _ = black_box(1681 + 1);
        }
        if flattened_cov.contains_key(&(1682 as u64)) {
            let _ = black_box(1682 + 1);
        }
        if flattened_cov.contains_key(&(1683 as u64)) {
            let _ = black_box(1683 + 1);
        }
        if flattened_cov.contains_key(&(1684 as u64)) {
            let _ = black_box(1684 + 1);
        }
        if flattened_cov.contains_key(&(1685 as u64)) {
            let _ = black_box(1685 + 1);
        }
        if flattened_cov.contains_key(&(1686 as u64)) {
            let _ = black_box(1686 + 1);
        }
        if flattened_cov.contains_key(&(1687 as u64)) {
            let _ = black_box(1687 + 1);
        }
        if flattened_cov.contains_key(&(1688 as u64)) {
            let _ = black_box(1688 + 1);
        }
        if flattened_cov.contains_key(&(1689 as u64)) {
            let _ = black_box(1689 + 1);
        }
        if flattened_cov.contains_key(&(1690 as u64)) {
            let _ = black_box(1690 + 1);
        }
        if flattened_cov.contains_key(&(1691 as u64)) {
            let _ = black_box(1691 + 1);
        }
        if flattened_cov.contains_key(&(1692 as u64)) {
            let _ = black_box(1692 + 1);
        }
        if flattened_cov.contains_key(&(1693 as u64)) {
            let _ = black_box(1693 + 1);
        }
        if flattened_cov.contains_key(&(1694 as u64)) {
            let _ = black_box(1694 + 1);
        }
        if flattened_cov.contains_key(&(1695 as u64)) {
            let _ = black_box(1695 + 1);
        }
        if flattened_cov.contains_key(&(1696 as u64)) {
            let _ = black_box(1696 + 1);
        }
        if flattened_cov.contains_key(&(1697 as u64)) {
            let _ = black_box(1697 + 1);
        }
        if flattened_cov.contains_key(&(1698 as u64)) {
            let _ = black_box(1698 + 1);
        }
        if flattened_cov.contains_key(&(1699 as u64)) {
            let _ = black_box(1699 + 1);
        }
        if flattened_cov.contains_key(&(1700 as u64)) {
            let _ = black_box(1700 + 1);
        }
        if flattened_cov.contains_key(&(1701 as u64)) {
            let _ = black_box(1701 + 1);
        }
        if flattened_cov.contains_key(&(1702 as u64)) {
            let _ = black_box(1702 + 1);
        }
        if flattened_cov.contains_key(&(1703 as u64)) {
            let _ = black_box(1703 + 1);
        }
        if flattened_cov.contains_key(&(1704 as u64)) {
            let _ = black_box(1704 + 1);
        }
        if flattened_cov.contains_key(&(1705 as u64)) {
            let _ = black_box(1705 + 1);
        }
        if flattened_cov.contains_key(&(1706 as u64)) {
            let _ = black_box(1706 + 1);
        }
        if flattened_cov.contains_key(&(1707 as u64)) {
            let _ = black_box(1707 + 1);
        }
        if flattened_cov.contains_key(&(1708 as u64)) {
            let _ = black_box(1708 + 1);
        }
        if flattened_cov.contains_key(&(1709 as u64)) {
            let _ = black_box(1709 + 1);
        }
        if flattened_cov.contains_key(&(1710 as u64)) {
            let _ = black_box(1710 + 1);
        }
        if flattened_cov.contains_key(&(1711 as u64)) {
            let _ = black_box(1711 + 1);
        }
        if flattened_cov.contains_key(&(1712 as u64)) {
            let _ = black_box(1712 + 1);
        }
        if flattened_cov.contains_key(&(1713 as u64)) {
            let _ = black_box(1713 + 1);
        }
        if flattened_cov.contains_key(&(1714 as u64)) {
            let _ = black_box(1714 + 1);
        }
        if flattened_cov.contains_key(&(1715 as u64)) {
            let _ = black_box(1715 + 1);
        }
        if flattened_cov.contains_key(&(1716 as u64)) {
            let _ = black_box(1716 + 1);
        }
        if flattened_cov.contains_key(&(1717 as u64)) {
            let _ = black_box(1717 + 1);
        }
        if flattened_cov.contains_key(&(1718 as u64)) {
            let _ = black_box(1718 + 1);
        }
        if flattened_cov.contains_key(&(1719 as u64)) {
            let _ = black_box(1719 + 1);
        }
        if flattened_cov.contains_key(&(1720 as u64)) {
            let _ = black_box(1720 + 1);
        }
        if flattened_cov.contains_key(&(1721 as u64)) {
            let _ = black_box(1721 + 1);
        }
        if flattened_cov.contains_key(&(1722 as u64)) {
            let _ = black_box(1722 + 1);
        }
        if flattened_cov.contains_key(&(1723 as u64)) {
            let _ = black_box(1723 + 1);
        }
        if flattened_cov.contains_key(&(1724 as u64)) {
            let _ = black_box(1724 + 1);
        }
        if flattened_cov.contains_key(&(1725 as u64)) {
            let _ = black_box(1725 + 1);
        }
        if flattened_cov.contains_key(&(1726 as u64)) {
            let _ = black_box(1726 + 1);
        }
        if flattened_cov.contains_key(&(1727 as u64)) {
            let _ = black_box(1727 + 1);
        }
        if flattened_cov.contains_key(&(1728 as u64)) {
            let _ = black_box(1728 + 1);
        }
        if flattened_cov.contains_key(&(1729 as u64)) {
            let _ = black_box(1729 + 1);
        }
        if flattened_cov.contains_key(&(1730 as u64)) {
            let _ = black_box(1730 + 1);
        }
        if flattened_cov.contains_key(&(1731 as u64)) {
            let _ = black_box(1731 + 1);
        }
        if flattened_cov.contains_key(&(1732 as u64)) {
            let _ = black_box(1732 + 1);
        }
        if flattened_cov.contains_key(&(1733 as u64)) {
            let _ = black_box(1733 + 1);
        }
        if flattened_cov.contains_key(&(1734 as u64)) {
            let _ = black_box(1734 + 1);
        }
        if flattened_cov.contains_key(&(1735 as u64)) {
            let _ = black_box(1735 + 1);
        }
        if flattened_cov.contains_key(&(1736 as u64)) {
            let _ = black_box(1736 + 1);
        }
        if flattened_cov.contains_key(&(1737 as u64)) {
            let _ = black_box(1737 + 1);
        }
        if flattened_cov.contains_key(&(1738 as u64)) {
            let _ = black_box(1738 + 1);
        }
        if flattened_cov.contains_key(&(1739 as u64)) {
            let _ = black_box(1739 + 1);
        }
        if flattened_cov.contains_key(&(1740 as u64)) {
            let _ = black_box(1740 + 1);
        }
        if flattened_cov.contains_key(&(1741 as u64)) {
            let _ = black_box(1741 + 1);
        }
        if flattened_cov.contains_key(&(1742 as u64)) {
            let _ = black_box(1742 + 1);
        }
        if flattened_cov.contains_key(&(1743 as u64)) {
            let _ = black_box(1743 + 1);
        }
        if flattened_cov.contains_key(&(1744 as u64)) {
            let _ = black_box(1744 + 1);
        }
        if flattened_cov.contains_key(&(1745 as u64)) {
            let _ = black_box(1745 + 1);
        }
        if flattened_cov.contains_key(&(1746 as u64)) {
            let _ = black_box(1746 + 1);
        }
        if flattened_cov.contains_key(&(1747 as u64)) {
            let _ = black_box(1747 + 1);
        }
        if flattened_cov.contains_key(&(1748 as u64)) {
            let _ = black_box(1748 + 1);
        }
        if flattened_cov.contains_key(&(1749 as u64)) {
            let _ = black_box(1749 + 1);
        }
        if flattened_cov.contains_key(&(1750 as u64)) {
            let _ = black_box(1750 + 1);
        }
        if flattened_cov.contains_key(&(1751 as u64)) {
            let _ = black_box(1751 + 1);
        }
        if flattened_cov.contains_key(&(1752 as u64)) {
            let _ = black_box(1752 + 1);
        }
        if flattened_cov.contains_key(&(1753 as u64)) {
            let _ = black_box(1753 + 1);
        }
        if flattened_cov.contains_key(&(1754 as u64)) {
            let _ = black_box(1754 + 1);
        }
        if flattened_cov.contains_key(&(1755 as u64)) {
            let _ = black_box(1755 + 1);
        }
        if flattened_cov.contains_key(&(1756 as u64)) {
            let _ = black_box(1756 + 1);
        }
        if flattened_cov.contains_key(&(1757 as u64)) {
            let _ = black_box(1757 + 1);
        }
        if flattened_cov.contains_key(&(1758 as u64)) {
            let _ = black_box(1758 + 1);
        }
        if flattened_cov.contains_key(&(1759 as u64)) {
            let _ = black_box(1759 + 1);
        }
        if flattened_cov.contains_key(&(1760 as u64)) {
            let _ = black_box(1760 + 1);
        }
        if flattened_cov.contains_key(&(1761 as u64)) {
            let _ = black_box(1761 + 1);
        }
        if flattened_cov.contains_key(&(1762 as u64)) {
            let _ = black_box(1762 + 1);
        }
        if flattened_cov.contains_key(&(1763 as u64)) {
            let _ = black_box(1763 + 1);
        }
        if flattened_cov.contains_key(&(1764 as u64)) {
            let _ = black_box(1764 + 1);
        }
        if flattened_cov.contains_key(&(1765 as u64)) {
            let _ = black_box(1765 + 1);
        }
        if flattened_cov.contains_key(&(1766 as u64)) {
            let _ = black_box(1766 + 1);
        }
        if flattened_cov.contains_key(&(1767 as u64)) {
            let _ = black_box(1767 + 1);
        }
        if flattened_cov.contains_key(&(1768 as u64)) {
            let _ = black_box(1768 + 1);
        }
        if flattened_cov.contains_key(&(1769 as u64)) {
            let _ = black_box(1769 + 1);
        }
        if flattened_cov.contains_key(&(1770 as u64)) {
            let _ = black_box(1770 + 1);
        }
        if flattened_cov.contains_key(&(1771 as u64)) {
            let _ = black_box(1771 + 1);
        }
        if flattened_cov.contains_key(&(1772 as u64)) {
            let _ = black_box(1772 + 1);
        }
        if flattened_cov.contains_key(&(1773 as u64)) {
            let _ = black_box(1773 + 1);
        }
        if flattened_cov.contains_key(&(1774 as u64)) {
            let _ = black_box(1774 + 1);
        }
        if flattened_cov.contains_key(&(1775 as u64)) {
            let _ = black_box(1775 + 1);
        }
        if flattened_cov.contains_key(&(1776 as u64)) {
            let _ = black_box(1776 + 1);
        }
        if flattened_cov.contains_key(&(1777 as u64)) {
            let _ = black_box(1777 + 1);
        }
        if flattened_cov.contains_key(&(1778 as u64)) {
            let _ = black_box(1778 + 1);
        }
        if flattened_cov.contains_key(&(1779 as u64)) {
            let _ = black_box(1779 + 1);
        }
        if flattened_cov.contains_key(&(1780 as u64)) {
            let _ = black_box(1780 + 1);
        }
        if flattened_cov.contains_key(&(1781 as u64)) {
            let _ = black_box(1781 + 1);
        }
        if flattened_cov.contains_key(&(1782 as u64)) {
            let _ = black_box(1782 + 1);
        }
        if flattened_cov.contains_key(&(1783 as u64)) {
            let _ = black_box(1783 + 1);
        }
        if flattened_cov.contains_key(&(1784 as u64)) {
            let _ = black_box(1784 + 1);
        }
        if flattened_cov.contains_key(&(1785 as u64)) {
            let _ = black_box(1785 + 1);
        }
        if flattened_cov.contains_key(&(1786 as u64)) {
            let _ = black_box(1786 + 1);
        }
        if flattened_cov.contains_key(&(1787 as u64)) {
            let _ = black_box(1787 + 1);
        }
        if flattened_cov.contains_key(&(1788 as u64)) {
            let _ = black_box(1788 + 1);
        }
        if flattened_cov.contains_key(&(1789 as u64)) {
            let _ = black_box(1789 + 1);
        }
        if flattened_cov.contains_key(&(1790 as u64)) {
            let _ = black_box(1790 + 1);
        }
        if flattened_cov.contains_key(&(1791 as u64)) {
            let _ = black_box(1791 + 1);
        }
        if flattened_cov.contains_key(&(1792 as u64)) {
            let _ = black_box(1792 + 1);
        }
        if flattened_cov.contains_key(&(1793 as u64)) {
            let _ = black_box(1793 + 1);
        }
        if flattened_cov.contains_key(&(1794 as u64)) {
            let _ = black_box(1794 + 1);
        }
        if flattened_cov.contains_key(&(1795 as u64)) {
            let _ = black_box(1795 + 1);
        }
        if flattened_cov.contains_key(&(1796 as u64)) {
            let _ = black_box(1796 + 1);
        }
        if flattened_cov.contains_key(&(1797 as u64)) {
            let _ = black_box(1797 + 1);
        }
        if flattened_cov.contains_key(&(1798 as u64)) {
            let _ = black_box(1798 + 1);
        }
        if flattened_cov.contains_key(&(1799 as u64)) {
            let _ = black_box(1799 + 1);
        }
        if flattened_cov.contains_key(&(1800 as u64)) {
            let _ = black_box(1800 + 1);
        }
        if flattened_cov.contains_key(&(1801 as u64)) {
            let _ = black_box(1801 + 1);
        }
        if flattened_cov.contains_key(&(1802 as u64)) {
            let _ = black_box(1802 + 1);
        }
        if flattened_cov.contains_key(&(1803 as u64)) {
            let _ = black_box(1803 + 1);
        }
        if flattened_cov.contains_key(&(1804 as u64)) {
            let _ = black_box(1804 + 1);
        }
        if flattened_cov.contains_key(&(1805 as u64)) {
            let _ = black_box(1805 + 1);
        }
        if flattened_cov.contains_key(&(1806 as u64)) {
            let _ = black_box(1806 + 1);
        }
        if flattened_cov.contains_key(&(1807 as u64)) {
            let _ = black_box(1807 + 1);
        }
        if flattened_cov.contains_key(&(1808 as u64)) {
            let _ = black_box(1808 + 1);
        }
        if flattened_cov.contains_key(&(1809 as u64)) {
            let _ = black_box(1809 + 1);
        }
        if flattened_cov.contains_key(&(1810 as u64)) {
            let _ = black_box(1810 + 1);
        }
        if flattened_cov.contains_key(&(1811 as u64)) {
            let _ = black_box(1811 + 1);
        }
        if flattened_cov.contains_key(&(1812 as u64)) {
            let _ = black_box(1812 + 1);
        }
        if flattened_cov.contains_key(&(1813 as u64)) {
            let _ = black_box(1813 + 1);
        }
        if flattened_cov.contains_key(&(1814 as u64)) {
            let _ = black_box(1814 + 1);
        }
        if flattened_cov.contains_key(&(1815 as u64)) {
            let _ = black_box(1815 + 1);
        }
        if flattened_cov.contains_key(&(1816 as u64)) {
            let _ = black_box(1816 + 1);
        }
        if flattened_cov.contains_key(&(1817 as u64)) {
            let _ = black_box(1817 + 1);
        }
        if flattened_cov.contains_key(&(1818 as u64)) {
            let _ = black_box(1818 + 1);
        }
        if flattened_cov.contains_key(&(1819 as u64)) {
            let _ = black_box(1819 + 1);
        }
        if flattened_cov.contains_key(&(1820 as u64)) {
            let _ = black_box(1820 + 1);
        }
        if flattened_cov.contains_key(&(1821 as u64)) {
            let _ = black_box(1821 + 1);
        }
        if flattened_cov.contains_key(&(1822 as u64)) {
            let _ = black_box(1822 + 1);
        }
        if flattened_cov.contains_key(&(1823 as u64)) {
            let _ = black_box(1823 + 1);
        }
        if flattened_cov.contains_key(&(1824 as u64)) {
            let _ = black_box(1824 + 1);
        }
        if flattened_cov.contains_key(&(1825 as u64)) {
            let _ = black_box(1825 + 1);
        }
        if flattened_cov.contains_key(&(1826 as u64)) {
            let _ = black_box(1826 + 1);
        }
        if flattened_cov.contains_key(&(1827 as u64)) {
            let _ = black_box(1827 + 1);
        }
        if flattened_cov.contains_key(&(1828 as u64)) {
            let _ = black_box(1828 + 1);
        }
        if flattened_cov.contains_key(&(1829 as u64)) {
            let _ = black_box(1829 + 1);
        }
        if flattened_cov.contains_key(&(1830 as u64)) {
            let _ = black_box(1830 + 1);
        }
        if flattened_cov.contains_key(&(1831 as u64)) {
            let _ = black_box(1831 + 1);
        }
        if flattened_cov.contains_key(&(1832 as u64)) {
            let _ = black_box(1832 + 1);
        }
        if flattened_cov.contains_key(&(1833 as u64)) {
            let _ = black_box(1833 + 1);
        }
        if flattened_cov.contains_key(&(1834 as u64)) {
            let _ = black_box(1834 + 1);
        }
        if flattened_cov.contains_key(&(1835 as u64)) {
            let _ = black_box(1835 + 1);
        }
        if flattened_cov.contains_key(&(1836 as u64)) {
            let _ = black_box(1836 + 1);
        }
        if flattened_cov.contains_key(&(1837 as u64)) {
            let _ = black_box(1837 + 1);
        }
        if flattened_cov.contains_key(&(1838 as u64)) {
            let _ = black_box(1838 + 1);
        }
        if flattened_cov.contains_key(&(1839 as u64)) {
            let _ = black_box(1839 + 1);
        }
        if flattened_cov.contains_key(&(1840 as u64)) {
            let _ = black_box(1840 + 1);
        }
        if flattened_cov.contains_key(&(1841 as u64)) {
            let _ = black_box(1841 + 1);
        }
        if flattened_cov.contains_key(&(1842 as u64)) {
            let _ = black_box(1842 + 1);
        }
        if flattened_cov.contains_key(&(1843 as u64)) {
            let _ = black_box(1843 + 1);
        }
        if flattened_cov.contains_key(&(1844 as u64)) {
            let _ = black_box(1844 + 1);
        }
        if flattened_cov.contains_key(&(1845 as u64)) {
            let _ = black_box(1845 + 1);
        }
        if flattened_cov.contains_key(&(1846 as u64)) {
            let _ = black_box(1846 + 1);
        }
        if flattened_cov.contains_key(&(1847 as u64)) {
            let _ = black_box(1847 + 1);
        }
        if flattened_cov.contains_key(&(1848 as u64)) {
            let _ = black_box(1848 + 1);
        }
        if flattened_cov.contains_key(&(1849 as u64)) {
            let _ = black_box(1849 + 1);
        }
        if flattened_cov.contains_key(&(1850 as u64)) {
            let _ = black_box(1850 + 1);
        }
        if flattened_cov.contains_key(&(1851 as u64)) {
            let _ = black_box(1851 + 1);
        }
        if flattened_cov.contains_key(&(1852 as u64)) {
            let _ = black_box(1852 + 1);
        }
        if flattened_cov.contains_key(&(1853 as u64)) {
            let _ = black_box(1853 + 1);
        }
        if flattened_cov.contains_key(&(1854 as u64)) {
            let _ = black_box(1854 + 1);
        }
        if flattened_cov.contains_key(&(1855 as u64)) {
            let _ = black_box(1855 + 1);
        }
        if flattened_cov.contains_key(&(1856 as u64)) {
            let _ = black_box(1856 + 1);
        }
        if flattened_cov.contains_key(&(1857 as u64)) {
            let _ = black_box(1857 + 1);
        }
        if flattened_cov.contains_key(&(1858 as u64)) {
            let _ = black_box(1858 + 1);
        }
        if flattened_cov.contains_key(&(1859 as u64)) {
            let _ = black_box(1859 + 1);
        }
        if flattened_cov.contains_key(&(1860 as u64)) {
            let _ = black_box(1860 + 1);
        }
        if flattened_cov.contains_key(&(1861 as u64)) {
            let _ = black_box(1861 + 1);
        }
        if flattened_cov.contains_key(&(1862 as u64)) {
            let _ = black_box(1862 + 1);
        }
        if flattened_cov.contains_key(&(1863 as u64)) {
            let _ = black_box(1863 + 1);
        }
        if flattened_cov.contains_key(&(1864 as u64)) {
            let _ = black_box(1864 + 1);
        }
        if flattened_cov.contains_key(&(1865 as u64)) {
            let _ = black_box(1865 + 1);
        }
        if flattened_cov.contains_key(&(1866 as u64)) {
            let _ = black_box(1866 + 1);
        }
        if flattened_cov.contains_key(&(1867 as u64)) {
            let _ = black_box(1867 + 1);
        }
        if flattened_cov.contains_key(&(1868 as u64)) {
            let _ = black_box(1868 + 1);
        }
        if flattened_cov.contains_key(&(1869 as u64)) {
            let _ = black_box(1869 + 1);
        }
        if flattened_cov.contains_key(&(1870 as u64)) {
            let _ = black_box(1870 + 1);
        }
        if flattened_cov.contains_key(&(1871 as u64)) {
            let _ = black_box(1871 + 1);
        }
        if flattened_cov.contains_key(&(1872 as u64)) {
            let _ = black_box(1872 + 1);
        }
        if flattened_cov.contains_key(&(1873 as u64)) {
            let _ = black_box(1873 + 1);
        }
        if flattened_cov.contains_key(&(1874 as u64)) {
            let _ = black_box(1874 + 1);
        }
        if flattened_cov.contains_key(&(1875 as u64)) {
            let _ = black_box(1875 + 1);
        }
        if flattened_cov.contains_key(&(1876 as u64)) {
            let _ = black_box(1876 + 1);
        }
        if flattened_cov.contains_key(&(1877 as u64)) {
            let _ = black_box(1877 + 1);
        }
        if flattened_cov.contains_key(&(1878 as u64)) {
            let _ = black_box(1878 + 1);
        }
        if flattened_cov.contains_key(&(1879 as u64)) {
            let _ = black_box(1879 + 1);
        }
        if flattened_cov.contains_key(&(1880 as u64)) {
            let _ = black_box(1880 + 1);
        }
        if flattened_cov.contains_key(&(1881 as u64)) {
            let _ = black_box(1881 + 1);
        }
        if flattened_cov.contains_key(&(1882 as u64)) {
            let _ = black_box(1882 + 1);
        }
        if flattened_cov.contains_key(&(1883 as u64)) {
            let _ = black_box(1883 + 1);
        }
        if flattened_cov.contains_key(&(1884 as u64)) {
            let _ = black_box(1884 + 1);
        }
        if flattened_cov.contains_key(&(1885 as u64)) {
            let _ = black_box(1885 + 1);
        }
        if flattened_cov.contains_key(&(1886 as u64)) {
            let _ = black_box(1886 + 1);
        }
        if flattened_cov.contains_key(&(1887 as u64)) {
            let _ = black_box(1887 + 1);
        }
        if flattened_cov.contains_key(&(1888 as u64)) {
            let _ = black_box(1888 + 1);
        }
        if flattened_cov.contains_key(&(1889 as u64)) {
            let _ = black_box(1889 + 1);
        }
        if flattened_cov.contains_key(&(1890 as u64)) {
            let _ = black_box(1890 + 1);
        }
        if flattened_cov.contains_key(&(1891 as u64)) {
            let _ = black_box(1891 + 1);
        }
        if flattened_cov.contains_key(&(1892 as u64)) {
            let _ = black_box(1892 + 1);
        }
        if flattened_cov.contains_key(&(1893 as u64)) {
            let _ = black_box(1893 + 1);
        }
        if flattened_cov.contains_key(&(1894 as u64)) {
            let _ = black_box(1894 + 1);
        }
        if flattened_cov.contains_key(&(1895 as u64)) {
            let _ = black_box(1895 + 1);
        }
        if flattened_cov.contains_key(&(1896 as u64)) {
            let _ = black_box(1896 + 1);
        }
        if flattened_cov.contains_key(&(1897 as u64)) {
            let _ = black_box(1897 + 1);
        }
        if flattened_cov.contains_key(&(1898 as u64)) {
            let _ = black_box(1898 + 1);
        }
        if flattened_cov.contains_key(&(1899 as u64)) {
            let _ = black_box(1899 + 1);
        }
        if flattened_cov.contains_key(&(1900 as u64)) {
            let _ = black_box(1900 + 1);
        }
        if flattened_cov.contains_key(&(1901 as u64)) {
            let _ = black_box(1901 + 1);
        }
        if flattened_cov.contains_key(&(1902 as u64)) {
            let _ = black_box(1902 + 1);
        }
        if flattened_cov.contains_key(&(1903 as u64)) {
            let _ = black_box(1903 + 1);
        }
        if flattened_cov.contains_key(&(1904 as u64)) {
            let _ = black_box(1904 + 1);
        }
        if flattened_cov.contains_key(&(1905 as u64)) {
            let _ = black_box(1905 + 1);
        }
        if flattened_cov.contains_key(&(1906 as u64)) {
            let _ = black_box(1906 + 1);
        }
        if flattened_cov.contains_key(&(1907 as u64)) {
            let _ = black_box(1907 + 1);
        }
        if flattened_cov.contains_key(&(1908 as u64)) {
            let _ = black_box(1908 + 1);
        }
        if flattened_cov.contains_key(&(1909 as u64)) {
            let _ = black_box(1909 + 1);
        }
        if flattened_cov.contains_key(&(1910 as u64)) {
            let _ = black_box(1910 + 1);
        }
        if flattened_cov.contains_key(&(1911 as u64)) {
            let _ = black_box(1911 + 1);
        }
        if flattened_cov.contains_key(&(1912 as u64)) {
            let _ = black_box(1912 + 1);
        }
        if flattened_cov.contains_key(&(1913 as u64)) {
            let _ = black_box(1913 + 1);
        }
        if flattened_cov.contains_key(&(1914 as u64)) {
            let _ = black_box(1914 + 1);
        }
        if flattened_cov.contains_key(&(1915 as u64)) {
            let _ = black_box(1915 + 1);
        }
        if flattened_cov.contains_key(&(1916 as u64)) {
            let _ = black_box(1916 + 1);
        }
        if flattened_cov.contains_key(&(1917 as u64)) {
            let _ = black_box(1917 + 1);
        }
        if flattened_cov.contains_key(&(1918 as u64)) {
            let _ = black_box(1918 + 1);
        }
        if flattened_cov.contains_key(&(1919 as u64)) {
            let _ = black_box(1919 + 1);
        }
        if flattened_cov.contains_key(&(1920 as u64)) {
            let _ = black_box(1920 + 1);
        }
        if flattened_cov.contains_key(&(1921 as u64)) {
            let _ = black_box(1921 + 1);
        }
        if flattened_cov.contains_key(&(1922 as u64)) {
            let _ = black_box(1922 + 1);
        }
        if flattened_cov.contains_key(&(1923 as u64)) {
            let _ = black_box(1923 + 1);
        }
        if flattened_cov.contains_key(&(1924 as u64)) {
            let _ = black_box(1924 + 1);
        }
        if flattened_cov.contains_key(&(1925 as u64)) {
            let _ = black_box(1925 + 1);
        }
        if flattened_cov.contains_key(&(1926 as u64)) {
            let _ = black_box(1926 + 1);
        }
        if flattened_cov.contains_key(&(1927 as u64)) {
            let _ = black_box(1927 + 1);
        }
        if flattened_cov.contains_key(&(1928 as u64)) {
            let _ = black_box(1928 + 1);
        }
        if flattened_cov.contains_key(&(1929 as u64)) {
            let _ = black_box(1929 + 1);
        }
        if flattened_cov.contains_key(&(1930 as u64)) {
            let _ = black_box(1930 + 1);
        }
        if flattened_cov.contains_key(&(1931 as u64)) {
            let _ = black_box(1931 + 1);
        }
        if flattened_cov.contains_key(&(1932 as u64)) {
            let _ = black_box(1932 + 1);
        }
        if flattened_cov.contains_key(&(1933 as u64)) {
            let _ = black_box(1933 + 1);
        }
        if flattened_cov.contains_key(&(1934 as u64)) {
            let _ = black_box(1934 + 1);
        }
        if flattened_cov.contains_key(&(1935 as u64)) {
            let _ = black_box(1935 + 1);
        }
        if flattened_cov.contains_key(&(1936 as u64)) {
            let _ = black_box(1936 + 1);
        }
        if flattened_cov.contains_key(&(1937 as u64)) {
            let _ = black_box(1937 + 1);
        }
        if flattened_cov.contains_key(&(1938 as u64)) {
            let _ = black_box(1938 + 1);
        }
        if flattened_cov.contains_key(&(1939 as u64)) {
            let _ = black_box(1939 + 1);
        }
        if flattened_cov.contains_key(&(1940 as u64)) {
            let _ = black_box(1940 + 1);
        }
        if flattened_cov.contains_key(&(1941 as u64)) {
            let _ = black_box(1941 + 1);
        }
        if flattened_cov.contains_key(&(1942 as u64)) {
            let _ = black_box(1942 + 1);
        }
        if flattened_cov.contains_key(&(1943 as u64)) {
            let _ = black_box(1943 + 1);
        }
        if flattened_cov.contains_key(&(1944 as u64)) {
            let _ = black_box(1944 + 1);
        }
        if flattened_cov.contains_key(&(1945 as u64)) {
            let _ = black_box(1945 + 1);
        }
        if flattened_cov.contains_key(&(1946 as u64)) {
            let _ = black_box(1946 + 1);
        }
        if flattened_cov.contains_key(&(1947 as u64)) {
            let _ = black_box(1947 + 1);
        }
        if flattened_cov.contains_key(&(1948 as u64)) {
            let _ = black_box(1948 + 1);
        }
        if flattened_cov.contains_key(&(1949 as u64)) {
            let _ = black_box(1949 + 1);
        }
        if flattened_cov.contains_key(&(1950 as u64)) {
            let _ = black_box(1950 + 1);
        }
        if flattened_cov.contains_key(&(1951 as u64)) {
            let _ = black_box(1951 + 1);
        }
        if flattened_cov.contains_key(&(1952 as u64)) {
            let _ = black_box(1952 + 1);
        }
        if flattened_cov.contains_key(&(1953 as u64)) {
            let _ = black_box(1953 + 1);
        }
        if flattened_cov.contains_key(&(1954 as u64)) {
            let _ = black_box(1954 + 1);
        }
        if flattened_cov.contains_key(&(1955 as u64)) {
            let _ = black_box(1955 + 1);
        }
        if flattened_cov.contains_key(&(1956 as u64)) {
            let _ = black_box(1956 + 1);
        }
        if flattened_cov.contains_key(&(1957 as u64)) {
            let _ = black_box(1957 + 1);
        }
        if flattened_cov.contains_key(&(1958 as u64)) {
            let _ = black_box(1958 + 1);
        }
        if flattened_cov.contains_key(&(1959 as u64)) {
            let _ = black_box(1959 + 1);
        }
        if flattened_cov.contains_key(&(1960 as u64)) {
            let _ = black_box(1960 + 1);
        }
        if flattened_cov.contains_key(&(1961 as u64)) {
            let _ = black_box(1961 + 1);
        }
        if flattened_cov.contains_key(&(1962 as u64)) {
            let _ = black_box(1962 + 1);
        }
        if flattened_cov.contains_key(&(1963 as u64)) {
            let _ = black_box(1963 + 1);
        }
        if flattened_cov.contains_key(&(1964 as u64)) {
            let _ = black_box(1964 + 1);
        }
        if flattened_cov.contains_key(&(1965 as u64)) {
            let _ = black_box(1965 + 1);
        }
        if flattened_cov.contains_key(&(1966 as u64)) {
            let _ = black_box(1966 + 1);
        }
        if flattened_cov.contains_key(&(1967 as u64)) {
            let _ = black_box(1967 + 1);
        }
        if flattened_cov.contains_key(&(1968 as u64)) {
            let _ = black_box(1968 + 1);
        }
        if flattened_cov.contains_key(&(1969 as u64)) {
            let _ = black_box(1969 + 1);
        }
        if flattened_cov.contains_key(&(1970 as u64)) {
            let _ = black_box(1970 + 1);
        }
        if flattened_cov.contains_key(&(1971 as u64)) {
            let _ = black_box(1971 + 1);
        }
        if flattened_cov.contains_key(&(1972 as u64)) {
            let _ = black_box(1972 + 1);
        }
        if flattened_cov.contains_key(&(1973 as u64)) {
            let _ = black_box(1973 + 1);
        }
        if flattened_cov.contains_key(&(1974 as u64)) {
            let _ = black_box(1974 + 1);
        }
        if flattened_cov.contains_key(&(1975 as u64)) {
            let _ = black_box(1975 + 1);
        }
        if flattened_cov.contains_key(&(1976 as u64)) {
            let _ = black_box(1976 + 1);
        }
        if flattened_cov.contains_key(&(1977 as u64)) {
            let _ = black_box(1977 + 1);
        }
        if flattened_cov.contains_key(&(1978 as u64)) {
            let _ = black_box(1978 + 1);
        }
        if flattened_cov.contains_key(&(1979 as u64)) {
            let _ = black_box(1979 + 1);
        }
        if flattened_cov.contains_key(&(1980 as u64)) {
            let _ = black_box(1980 + 1);
        }
        if flattened_cov.contains_key(&(1981 as u64)) {
            let _ = black_box(1981 + 1);
        }
        if flattened_cov.contains_key(&(1982 as u64)) {
            let _ = black_box(1982 + 1);
        }
        if flattened_cov.contains_key(&(1983 as u64)) {
            let _ = black_box(1983 + 1);
        }
        if flattened_cov.contains_key(&(1984 as u64)) {
            let _ = black_box(1984 + 1);
        }
        if flattened_cov.contains_key(&(1985 as u64)) {
            let _ = black_box(1985 + 1);
        }
        if flattened_cov.contains_key(&(1986 as u64)) {
            let _ = black_box(1986 + 1);
        }
        if flattened_cov.contains_key(&(1987 as u64)) {
            let _ = black_box(1987 + 1);
        }
        if flattened_cov.contains_key(&(1988 as u64)) {
            let _ = black_box(1988 + 1);
        }
        if flattened_cov.contains_key(&(1989 as u64)) {
            let _ = black_box(1989 + 1);
        }
        if flattened_cov.contains_key(&(1990 as u64)) {
            let _ = black_box(1990 + 1);
        }
        if flattened_cov.contains_key(&(1991 as u64)) {
            let _ = black_box(1991 + 1);
        }
        if flattened_cov.contains_key(&(1992 as u64)) {
            let _ = black_box(1992 + 1);
        }
        if flattened_cov.contains_key(&(1993 as u64)) {
            let _ = black_box(1993 + 1);
        }
        if flattened_cov.contains_key(&(1994 as u64)) {
            let _ = black_box(1994 + 1);
        }
        if flattened_cov.contains_key(&(1995 as u64)) {
            let _ = black_box(1995 + 1);
        }
        if flattened_cov.contains_key(&(1996 as u64)) {
            let _ = black_box(1996 + 1);
        }
        if flattened_cov.contains_key(&(1997 as u64)) {
            let _ = black_box(1997 + 1);
        }
        if flattened_cov.contains_key(&(1998 as u64)) {
            let _ = black_box(1998 + 1);
        }
        if flattened_cov.contains_key(&(1999 as u64)) {
            let _ = black_box(1999 + 1);
        }
        if flattened_cov.contains_key(&(2000 as u64)) {
            let _ = black_box(2000 + 1);
        }
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
