use std::str::FromStr;

#[derive(Debug, Default)]
pub struct SeedExtractor {
    raw_input: String,
}

impl FromStr for SeedExtractor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            raw_input: s.to_string(),
        })
    }
}
type Seed = Vec<u8>;

impl SeedExtractor {
    pub fn new(input: String) -> Self {
        Self { raw_input: input }
    }

    /// Extracts all seeds from the input string
    pub fn extract_seeds(&self) -> Vec<Seed> {
        self.raw_input
            .lines()
            .filter_map(|line| {
                let line = line.split('\n').next().unwrap_or(line);

                // This is for unit test
                if let Some(encoded_seed) = line.trim().strip_prefix("ENCODED_SEED=") {
                    return Self::decode_hex(encoded_seed);
                }

                // This is for E2E tests
                if let Some(debug_msg) = line.trim().strip_prefix("DEBUG_MESSAGE_FROM_INK = ") {
                    if let Some(encoded_seed) = debug_msg
                        .trim_matches('"')
                        .trim()
                        .strip_prefix("ENCODED_SEED=")
                    {
                        return Self::decode_hex(encoded_seed);
                    }
                }

                None
            })
            .collect()
    }
    fn decode_hex(hex_str: &str) -> Option<Vec<u8>> {
        let hex_str = hex_str.trim_end_matches('\n');
        let decoded = hex_str
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .filter_map(|chunk| {
                let hex_pair: String = chunk.iter().collect();
                u8::from_str_radix(&hex_pair, 16).ok()
            })
            .collect();
        Some(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_seeds() {
        let input = r#"
 [==] Generating bundle
Using "/var/folders/zz/l24gry9516j1qhdft_hszcph0000gn/T/phink_seedgen" for contract output
running 3 tests
test dummy::e2e_tests::it_works ... ok
test dummy::tests::for_seedgen ... ok
test dummy::tests::new_works ... ok

successes:

---- dummy::e2e_tests::it_works stdout ----
 [==] Checking clippy linting rules
 [==] Building cargo project
 [==] Post processing code
 [==] Generating metadata
 [==] Generating bundle
DEBUG_MESSAGE_FROM_INK = "ENCODED_SEED=4c406b48309906001212121212121212121212121212121212121212121212121212121212121212\n"

---- dummy::tests::for_seedgen stdout ----
ENCODED_SEED=4c406b48200000009999999999999999999999999999999999999999999999999999999999999999

---- dummy::tests::new_works stdout ----
ENCODED_SEED=fa80c2f60c616263
ENCODED_SEED=fa80c2f60c66757a

successes:
    dummy::e2e_tests::it_works
    dummy::tests::new_works
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.68s
"#;

        let extractor = SeedExtractor::new(input.to_string());
        let seeds = extractor.extract_seeds();

        assert_eq!(seeds.len(), 4);
        assert_eq!(
            seeds[0],
            [
                76, 64, 107, 72, 48, 153, 6, 0, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18,
                18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18
            ]
        );

        assert_eq!(
            seeds[1],
            [
                76, 64, 107, 72, 32, 0, 0, 0, 153, 153, 153, 153, 153, 153, 153, 153, 153, 153,
                153, 153, 153, 153, 153, 153, 153, 153, 153, 153, 153, 153, 153, 153, 153, 153,
                153, 153, 153, 153, 153, 153
            ]
        );

        assert_eq!(seeds[2], [250, 128, 194, 246, 12, 97, 98, 99]);
        assert_eq!(seeds[3], [250, 128, 194, 246, 12, 102, 117, 122]);
    }

    #[test]
    fn test_from_str() {
        let input = "ENCODED_SEED=fa80c2f60c616263";
        let extractor = SeedExtractor::from_str(input).unwrap();
        let seeds = extractor.extract_seeds();

        assert_eq!(seeds.len(), 1);
        assert_eq!(
            seeds[0],
            vec![0xfa, 0x80, 0xc2, 0xf6, 0x0c, 0x61, 0x62, 0x63]
        );
    }
}
