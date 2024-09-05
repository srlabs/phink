extern crate phink_lib;
pub mod shared;

#[cfg(test)]
mod tests {
    use crate::shared::samples::Sample;

    use std::{
        ffi::OsStr,
        fs,
        path::Path,
    };

    #[test]
    fn test_sample_files_not_empty() {
        for sample in [
            Sample::CrossMessageBug,
            Sample::DNS,
            Sample::Dummy,
            Sample::MultiContractCaller,
            Sample::Transfer,
        ] {
            let sample_path = Path::new(sample.path());
            assert!(
                sample_path.exists(),
                "Sample path does not exist: {:?}",
                sample_path
            );

            let entries = fs::read_dir(sample_path)
                .unwrap_or_else(|_| panic!("Failed to read directory: {:?}", sample_path));

            let mut file_count = 0;
            for entry in entries {
                let entry = entry.expect("Failed to read directory entry");
                let path = entry.path();

                if path.is_file() && (path.extension() == Some(OsStr::new("rs"))) {
                    file_count += 1;
                    let content = fs::read_to_string(&path)
                        .unwrap_or_else(|_| panic!("Failed to read file: {:?}", path));

                    assert!(!content.trim().is_empty(), "File is empty: {:?}", path);
                }
            }

            assert!(
                file_count > 0,
                "No files found in sample directory: {:?}",
                sample_path
            );
        }
    }

    #[test]
    fn test_dns_contract_complexity() {
        let dns_path = Path::new(Sample::DNS.path());
        let content =
            fs::read_to_string(dns_path.join("lib.rs")).expect("Failed to read DNS contract file");

        // Check for complex logic indicators
        assert!(
            content.contains("struct"),
            "DNS contract should contain struct definitions"
        );
        assert!(
            content.contains("enum"),
            "DNS contract should contain enum definitions"
        );
        assert!(
            content.contains("#[ink(message)]"),
            "DNS contract should contain ink! message functions"
        );
        assert!(
            content.contains("if"),
            "DNS contract should contain conditional statements"
        );
    }

    #[test]
    fn test_cross_message_bug_contract() {
        let bug_path = Path::new(Sample::CrossMessageBug.path());
        let content = fs::read_to_string(bug_path.join("lib.rs"))
            .expect("Failed to read CrossMessageBug contract file");

        // Check for multiple message functions
        let message_count = content.matches("#[ink(message)]").count();
        assert!(
            message_count >= 2,
            "CrossMessageBug contract should contain at least 2 message functions"
        );

        // Check for potential bug indicators
        assert!(
            content.contains("mut"),
            "CrossMessageBug contract should contain mutable state"
        );
        assert!(
            content.contains("self."),
            "CrossMessageBug contract should access contract state"
        );
    }

    #[test]
    fn test_dummy_contract_nested_ifs() {
        let dummy_path = Path::new(Sample::Dummy.path());
        let content = fs::read_to_string(dummy_path.join("lib.rs"))
            .expect("Failed to read Dummy contract file");

        // Check for nested if statements
        let if_count = content.matches("if").count();
        assert!(
            if_count >= 4,
            "Dummy contract should contain at least 4 'if' statements"
        );
    }

    #[test]
    fn test_multi_contract_caller() {
        let caller_path = Path::new(Sample::MultiContractCaller.path());
        let files =
            fs::read_dir(caller_path).expect("Failed to read MultiContractCaller directory");

        for file in files {
            let file = file.expect("Failed to read file in MultiContractCaller directory");
            if file.file_type().expect("Failed to get file type").is_file()
                && file.file_name().to_str().unwrap().ends_with(".rs")
            {
                let content = fs::read_to_string(file.path())
                    .expect("Failed to read contract file in MultiContractCaller");

                assert!(
                    content.contains("#[ink(message)]"),
                    "Each contract in MultiContractCaller should have message functions"
                );
            }
        }
    }

    #[test]
    fn test_transfer_contract_invariants() {
        let transfer_path = Path::new(Sample::Transfer.path());
        let content = fs::read_to_string(transfer_path.join("lib.rs"))
            .expect("Failed to read Transfer contract file");

        // Check for transfer-related functions and checks
        assert!(
            content.contains("transfer"),
            "Transfer contract should contain transfer-related functions"
        );
        assert!(
            content.contains("#[ink(message, payable)]"),
            "Transfer contract should handle balance checks"
        );
        assert!(
            content.contains("pub fn phink_"),
            "Transfer contract should contain assertions for invariant checks"
        );

        assert!(
            content.contains("#[cfg(feature = \"phink\")]"),
            "Transfer contract should contain Phink feature"
        );
    }
}
