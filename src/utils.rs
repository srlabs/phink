use std::collections::HashSet;

/// A simple helper to remove some duplicated lines from a `&str`
/// This is used mainly to remove coverage returns being inserted many times in the debug vector
/// in case of any `iter()`, `for` loop and so on
/// # Arguments
/// * `input`: The string to deduplicate
pub fn deduplicate(input: &str) -> String {
    let mut unique_lines = HashSet::new();
    input
        .lines()
        .filter(|&line| unique_lines.insert(line))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::deduplicate;
    use std::borrow::Cow;

    #[test]
    pub fn test_deduplicate() {
        // Test case: input with duplicate lines
        let input = "line1\nline2\nline1\nline3\nline2";
        let expected = "line1\nline2\nline3";
        let cow_input = Cow::Borrowed(input);
        assert_eq!(deduplicate(&cow_input), Cow::Owned(expected.to_string()));

        // Test case: input without duplicate lines
        let input = "line1\nline2\nline3";
        let expected = "line1\nline2\nline3";
        let cow_input = Cow::Borrowed(input);
        assert_eq!(deduplicate(&cow_input), Cow::Owned(expected.to_string()));

        // Test case: empty input
        let input = "";
        let expected = "";
        let cow_input = Cow::Borrowed(input);
        assert_eq!(deduplicate(&cow_input), Cow::Owned(expected.to_string()));

        // Test case: input with consecutive duplicate lines
        let input = "line1\nline1\nline2\nline2\nline3\nline3";
        let expected = "line1\nline2\nline3";
        let cow_input = Cow::Borrowed(input);
        assert_eq!(deduplicate(&cow_input), Cow::Owned(expected.to_string()));

        // Test case: input with non-consecutive duplicate lines
        let input = "line1\nline2\nline3\nline1\nline2";
        let expected = "line1\nline2\nline3";
        let cow_input = Cow::Borrowed(input);
        assert_eq!(deduplicate(&cow_input), Cow::Owned(expected.to_string()));
    }
}
