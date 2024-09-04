use crate::{
    cli::ziggy::ZiggyConfig,
    instrumenter::instrumentation::instrument::ContractCovUpdater,
};
use anyhow::{
    bail,
    Context,
};
use quote::quote;
use regex::Regex;
use std::{
    ffi::OsStr,
    fs,
    fs::{
        copy,
        File,
    },
    io::Write,
    path::{
        Path,
        PathBuf,
    },
    process::Command,
};
use syn::{
    parse_file,
    visit_mut::VisitMut,
};
use walkdir::WalkDir;

/// The objective of this `struct` is to assist Phink in instrumenting ink!
///
/// smart contracts. In a fuzzing context, instrumenting a smart contract
/// involves modifying the target (i.e., the WASM blob), for example, by adding
/// additional code to branches to obtain a coverage map during the execution of
/// the smart contract. By doing so, we can effectively generate a coverage map
/// that will be provided to Ziggy transforming Phink from a basic brute-forcing
/// tool into a powerful coverage-guided fuzzer.
///
/// Phink opted for a Rust AST approach. For each code instruction on the
/// smart-contract, Phink will automatically add a tracing code, which will then
/// be fetched at the end of the input execution in order to get coverage.
#[derive(Default, Clone)]
pub struct Instrumenter {
    pub z_config: ZiggyConfig,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InkFilesPath {
    pub wasm_path: PathBuf,
    pub specs_path: PathBuf,
}

pub trait ContractInstrumenter {
    fn instrument(&mut self) -> anyhow::Result<&mut Self>
    where
        Self: Sized;
    fn instrument_file(
        &self,
        path: &Path,
        contract_cov_manager: &mut ContractCovUpdater,
    ) -> anyhow::Result<()>;
    fn parse_and_visit(code: &str, visitor: impl VisitMut) -> anyhow::Result<String>;
    fn save_and_format(source_code: String, lib_rs: PathBuf) -> anyhow::Result<()>;
    fn already_instrumented(code: &str) -> bool;
}

impl Instrumenter {
    pub fn new(z_config: ZiggyConfig) -> Self {
        Self { z_config }
    }

    pub fn find(&self) -> anyhow::Result<InkFilesPath> {
        let wasm_path = fs::read_dir(self.z_config.contract_path.join("target/ink/"))
            .with_context(|| {
                format!(
                    "🙅 It seems that your contract is not compiled into `target/ink`. \
                Please, ensure that your WASM blob and the JSON specs are stored in \
                '{}target/ink/' (more info: {})",
                    self.z_config.contract_path.to_str().unwrap(),
                    self.z_config.contract_path.to_str().unwrap(),
                )
            })?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_file() && path.extension().and_then(OsStr::to_str) == Some("wasm") {
                    Some(path)
                } else {
                    None
                }
            })
            .next()
            .ok_or_else(|| anyhow::anyhow!("🙅 No .wasm file found in target directory"))?;

        let specs_path = PathBuf::from(wasm_path.to_str().unwrap().replace(".wasm", ".json"));

        Ok(InkFilesPath {
            wasm_path,
            specs_path,
        })
    }
}
pub trait ContractBuilder {
    fn build(&self) -> anyhow::Result<()>;
}

impl ContractBuilder for Instrumenter {
    fn build(&self) -> anyhow::Result<()> {
        let status = Command::new("cargo")
            .current_dir(&self.z_config.contract_path)
            .args(["contract", "build", "--features=phink"])
            .status()?;

        if !status.success() {
            bail!(
                "🙅 It seems that your instrumented smart contract did not compile properly. \
                Please go to {}, edit the `lib.rs` file, and run cargo contract build again.\
                (more infos: {status})",
                &self.z_config.contract_path.display()
            )
        }
        Ok(())
    }
}
pub trait ContractForker {
    fn fork(&self) -> anyhow::Result<PathBuf>;
}
impl ContractForker for Instrumenter {
    fn fork(&self) -> anyhow::Result<PathBuf> {
        let new_dir = &self
            .z_config
            .config
            .instrumented_contract_path
            .clone()
            .unwrap_or_default()
            .path;

        println!("🏗️ Creating new directory: {:?}", new_dir);
        fs::create_dir_all(new_dir)
            .with_context(|| format!("🙅 Failed to create directory: {}", new_dir.display()))?;

        println!(
            "📁 Starting to copy files from {:?}",
            self.z_config.contract_path
        );

        for entry in WalkDir::new(&self.z_config.contract_path) {
            let entry = entry?;
            let target_path = new_dir.join(
                entry
                    .path()
                    .strip_prefix(&self.z_config.contract_path)
                    .with_context(|| "Couldn't `strip_prefix`")?,
            );

            if entry.path().is_dir() {
                println!("📂 Creating subdirectory: {:?}", target_path);
                fs::create_dir_all(&target_path)?;
            } else {
                println!("📄 Copying file: {:?} -> {:?}", entry.path(), target_path);
                copy(entry.path(), &target_path).with_context(|| {
                    format!("🙅 Failed to copy file to {}", target_path.display())
                })?;
            }
        }

        println!(
            "✅ Fork completed successfully! New directory: {:?}",
            new_dir
        );
        Ok(new_dir.clone())
    }
}

impl ContractInstrumenter for Instrumenter {
    fn instrument(&mut self) -> anyhow::Result<&mut Instrumenter> {
        let new_working_dir = self.fork()?;

        let mut contract_cov_manager = ContractCovUpdater { line_id: 0 };
        for entry in WalkDir::new(&new_working_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            .filter(|e| !e.path().components().any(|c| c.as_os_str() == "target"))
        // Don't instrument anything inside target
        {
            let path = entry.path();
            self.instrument_file(path, &mut contract_cov_manager)?;
        }
        Ok(self)
    }

    fn instrument_file(
        &self,
        path: &Path,
        contract_cov_manager: &mut ContractCovUpdater,
    ) -> anyhow::Result<()> {
        let code = fs::read_to_string(path)?;

        if Self::already_instrumented(&code) {
            return Ok(())
        }

        println!(
            "📝 Instrumenting file: {} with {contract_cov_manager:?}",
            path.display(),
        );

        let modified_code = Self::parse_and_visit(&code, contract_cov_manager)
            .with_context(|| "⚠️ This is most likely that your ink! contract contains invalid syntax. Try to compile it first. Also, ensure that `cargo-contract` is installed.".to_string())?;

        Self::save_and_format(modified_code, PathBuf::from(path))?;

        Ok(())
    }

    fn parse_and_visit(code: &str, mut visitor: impl VisitMut) -> anyhow::Result<String> {
        let mut ast = parse_file(code)?;

        visitor.visit_file_mut(&mut ast);

        Ok(quote!(#ast).to_string())
    }

    fn save_and_format(source_code: String, rust_file: PathBuf) -> anyhow::Result<()> {
        let mut file = File::create(rust_file.clone())?;
        file.write_all(source_code.as_bytes())?;
        println!("✍️ Writing instrumented source code");
        file.flush()?;
        println!("🛠️ Formatting {} with rustfmt...", rust_file.display());
        Command::new("rustfmt")
            .arg(rust_file)
            .arg("--edition=2021")
            .status()?;
        Ok(())
    }

    /// Checks if the given code string is already instrumented.
    /// This function looks for the presence of the pattern
    /// `ink::env::debug_println!("COV={}", 123)` where `123` can be any number. If
    /// this pattern is found, it means the code is instrumented.
    fn already_instrumented(code: &str) -> bool {
        Regex::new(r#"ink::env::debug_println!\("COV=\{}", \d+\);"#)
            .unwrap()
            .is_match(code)
    }
}

mod instrument {
    use proc_macro2::Span;
    use syn::{
        parse_quote,
        visit_mut::VisitMut,
        Expr,
        LitInt,
        Stmt,
        Token,
    };

    #[derive(Debug, Clone)]
    pub struct ContractCovUpdater {
        pub line_id: u64,
    }

    impl VisitMut for &mut ContractCovUpdater {
        fn visit_block_mut(&mut self, block: &mut syn::Block) {
            let mut new_stmts = Vec::new();
            // Temporarily replace block.stmts with an empty Vec to avoid
            // borrowing issues
            let mut stmts = std::mem::take(&mut block.stmts);
            for mut stmt in stmts.drain(..) {
                let line_lit = LitInt::new(self.line_id.to_string().as_str(), Span::call_site());

                self.line_id += 1;

                let insert_expr: Expr = parse_quote! {
                    ink::env::debug_println!("COV={}", #line_lit)
                };
                // Convert this expression into a statement
                let pre_stmt: Stmt = Stmt::Expr(insert_expr, Some(Token![;](Span::call_site())));
                new_stmts.push(pre_stmt);
                // Use recursive visitation to handle nested blocks and other
                // statement types
                self.visit_stmt_mut(&mut stmt);
                new_stmts.push(stmt.clone());
            }
            block.stmts = new_stmts;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::config::Configuration;
    use std::{
        default::Default,
        fs::{
            self,
            File,
        },
        io::Read,
    };
    use tempfile::tempdir;
    use walkdir::WalkDir;

    // Helper function to create a temporary `ZiggyConfig` for testing
    fn create_temp_ziggy_config() -> ZiggyConfig {
        let temp_dir = tempdir().unwrap();
        ZiggyConfig {
            config: Default::default(),
            contract_path: temp_dir.path().to_path_buf(),
        }
    }

    #[test]
    fn test_find_wasm_and_specs_paths_success() {
        let config = create_temp_ziggy_config();
        let wasm_file = config.contract_path.join("target/ink/test_contract.wasm");
        let specs_file = config.contract_path.join("target/ink/test_contract.json");

        // Create a fake .wasm file and corresponding .json spec file
        fs::create_dir_all(wasm_file.parent().unwrap()).unwrap();
        File::create(&wasm_file).unwrap();
        File::create(&specs_file).unwrap();

        let instrumenter = Instrumenter::new(config);

        let result = instrumenter.find().unwrap();

        assert_eq!(result.wasm_path, wasm_file);
        assert_eq!(result.specs_path, specs_file);
    }

    #[test]
    fn test_find_wasm_file_not_found() {
        let config = create_temp_ziggy_config();
        let instrumenter = Instrumenter::new(config);
        let result = instrumenter.find();
        assert!(result.is_err());
    }

    #[test]
    fn test_instrumentation_already_instrumented() {
        let code = r#"ink::env::debug_println!("COV={}", 123);"#;
        assert!(Instrumenter::already_instrumented(code));
    }

    #[test]
    fn test_instrumentation_fullcode_instrumented() {
        let code = r#"
        fn main() {
            ink::env::debug_println!("COV={}", 123);
            println!("Hello, World!");
        }"#;
        assert!(Instrumenter::already_instrumented(code));
    }
    #[test]
    fn test_instrumentation_not_yet_instrumented() {
        let code = r#"
        fn main() {
            println!("Hello, World!");
        }"#;
        assert!(!Instrumenter::already_instrumented(code));
    }

    #[test]
    fn test_instrument_file_success() {
        let config = ZiggyConfig {
            config: Configuration {
                instrumented_contract_path: Some(tempdir().unwrap().into_path().into()),
                ..Default::default()
            },
            contract_path: PathBuf::from("sample/dummy/"),
        };

        let mut instrumenter = Instrumenter::new(config.clone());
        assert!(instrumenter.instrument().is_ok(), "Instrumentation failed");
        let mut modified_code = String::new();
        File::open(
            config
                .config
                .instrumented_contract_path
                .unwrap_or_default()
                .path
                .join("lib.rs"),
        )
        .unwrap()
        .read_to_string(&mut modified_code)
        .unwrap();

        assert!(
            Instrumenter::already_instrumented(modified_code.as_str()),
            "Instrumentation didn't work properly"
        );
    }

    #[test]
    fn test_save_and_format_creates_and_formats_file() {
        let temp_dir = tempdir().unwrap();
        let rust_file = temp_dir.path().join("lib.rs");
        let source_code = String::from("fn main(){}");

        Instrumenter::save_and_format(source_code.clone(), rust_file.clone()).unwrap();

        let mut file_content = String::new();
        File::open(rust_file)
            .unwrap()
            .read_to_string(&mut file_content)
            .unwrap();

        assert!(file_content.contains("fn main() {}"));
    }

    #[test]
    fn test_fork_creates_new_directory() {
        let config = create_temp_ziggy_config();
        let instrumenter = Instrumenter::new(config.clone());

        let src_file = config.contract_path.join("lib.rs");
        fs::create_dir_all(src_file.parent().unwrap()).unwrap();
        File::create(&src_file).unwrap();

        let result = instrumenter.fork().unwrap();
        let files: Vec<_> = WalkDir::new(&result)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect();

        assert_eq!(files.len(), 2); // New directory and copied file
    }

    #[test]
    fn test_build_successful() {
        let config = ZiggyConfig {
            config: Default::default(),
            contract_path: PathBuf::from("sample/dummy"),
        };
        let instrumenter = Instrumenter::new(config.clone());
        let result = instrumenter.build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_fails() {
        let config = create_temp_ziggy_config();
        let instrumenter = Instrumenter::new(config.clone());

        let result = instrumenter.build();

        assert!(result.is_err());
    }
}
