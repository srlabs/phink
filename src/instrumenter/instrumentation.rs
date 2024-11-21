use crate::{
    cli::ziggy::ZiggyConfig,
    instrumenter::instrumentation::instrument::CoverageInjector,
    EmptyResult,
    ResultOf,
};
use anyhow::{
    anyhow,
    bail,
    Context,
};

use crate::instrumenter::traits::visitor::ContractVisitor;
use regex::Regex;
use std::{
    ffi::OsStr,
    fs,
    path::PathBuf,
};

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

impl ContractVisitor for Instrumenter {
    fn input_directory(&self) -> PathBuf {
        self.z_config.contract_path().unwrap()
    }

    fn output_directory(&self) -> PathBuf {
        self.z_config.config().instrumented_contract()
    }

    fn verbose(&self) -> bool {
        self.z_config.config().verbose
    }
}

impl Instrumenter {
    pub fn new(z_config: ZiggyConfig) -> Self {
        Self { z_config }
    }

    pub fn find(&self) -> ResultOf<InkFilesPath> {
        let c_path = self.output_directory();
        let c_path_str = c_path.to_str().unwrap();

        let wasm_path = match fs::read_dir(c_path.join("target/ink/")) {
            Ok(entries) => {
                entries
                    .filter_map(|entry| {
                        let path = entry.ok()?.path();
                        if path.is_file()
                            && path.extension().and_then(OsStr::to_str) == Some("wasm")
                        {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .next()
                    .ok_or_else(|| anyhow!("No .wasm file found in target directory"))?
            }
            Err(e) => bail!(format!("It seems that your contract is not compiled into `target/ink`. Please, ensure that your WASM blob and the JSON specs are stored in '{c_path_str}/target/ink/'. More details: {e:?}"))
        };

        let specs_path = PathBuf::from(wasm_path.to_str().unwrap().replace(".wasm", ".json"));
        Ok(InkFilesPath {
            wasm_path,
            specs_path,
        })
    }

    pub fn instrument(self) -> EmptyResult {
        self.fork()
            .context("Forking the project to a new directory failed")?;

        let injector = CoverageInjector::new();

        self.for_each_file(|file_path| {
            let source_code =
                fs::read_to_string(&file_path).context(format!("Couldn't read {file_path:?}"))?;

            if Self::already_instrumented(&source_code) {
                println!("{file_path:?} was already instrumented");
                return Ok(());
            }

            self.instrument_file(file_path, &source_code, injector.clone())
                .context("Failed to instrument the file")
        })?;

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
    pub struct CoverageInjector {
        pub line_id: u64,
    }

    impl CoverageInjector {
        pub fn new() -> Self {
            Self { line_id: 0 }
        }
    }

    impl VisitMut for CoverageInjector {
        fn visit_block_mut(&mut self, block: &mut syn::Block) {
            let mut new_stmts = Vec::new();
            // Temporarily replace block.stmts with an empty Vec to avoid
            // borrowing issues
            let mut stmts = std::mem::take(&mut block.stmts);
            for mut stmt in stmts.drain(..) {
                let line_lit = LitInt::new(self.line_id.to_string().as_str(), Span::call_site());

                println!("line_id = {}", self.line_id);
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
    use crate::{
        cli::config::Configuration,
        instrumenter::path::InstrumentedPath,
        EmptyResult,
    };
    use std::{
        default::Default,
        fs::{
            self,
            File,
        },
        path::Path,
    };
    use tempfile::{
        tempdir,
        Builder,
    };
    use walkdir::WalkDir;

    // Helper function to create a temporary `ZiggyConfig` for testing. If `keep` is set to `true`,
    // the folder (instrumented contract and fuzzing output) won't be deleted once the test passed
    fn create_temp_ziggy_config(keep: bool, verbose: bool) -> ZiggyConfig {
        let fuzz_output = Some(Builder::new().keep(keep).tempdir().unwrap().into_path());
        let instrumented_contract_path = Some(InstrumentedPath::from(
            Builder::new().keep(keep).tempdir().unwrap().into_path(),
        ));

        let configuration = Configuration {
            fuzz_output,
            instrumented_contract_path,
            verbose,
            ..Default::default()
        };
        ZiggyConfig::new_with_contract(configuration, PathBuf::from("sample/dummy")).unwrap()
    }

    #[test]
    fn test_create_temp_clippy() {
        let result = Instrumenter::create_temp_clippy().expect("Failed to create temp clippy.toml");
        assert!(result.ends_with("/clippy.toml"));
        let path = Path::new(&result);
        assert!(path.exists(), "clippy.toml file was not created");
        let content = fs::read_to_string(path).expect("Failed to read clippy.toml file");
        assert_eq!(
            content.trim(),
            "avoid-breaking-exported-api = false",
            "Unexpected content in clippy.toml"
        );
    }
    #[test]
    fn test_find_wasm_and_specs_paths_success() {
        let config = create_temp_ziggy_config(false, true);
        let buf = config.config().instrumented_contract();
        let wasm_file = buf.join("target/ink/dummy.wasm");
        let specs_file = buf.join("target/ink/dummy.json");

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
        let config = ZiggyConfig::new_with_contract(
            Configuration {
                fuzz_output: Some(tempdir().unwrap().into_path()),
                instrumented_contract_path: Some(InstrumentedPath::from(
                    tempdir().unwrap().into_path(),
                )),
                ..Default::default()
            },
            PathBuf::from("../"),
        )
        .unwrap();
        let instrumenter = Instrumenter::new(config);
        let result = instrumenter.find();
        assert!(result.is_err());
    }

    #[test]
    fn test_assert_folder_doesnt_exist() {
        assert!(ZiggyConfig::new_with_contract(
            Configuration {
                fuzz_output: Some(tempdir().unwrap().into_path()),
                instrumented_contract_path: Some(InstrumentedPath::from(
                    tempdir().unwrap().into_path(),
                )),
                ..Default::default()
            },
            PathBuf::from("rezrzerze/idontexistsad"),
        )
        .is_err())
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
    fn test_fork_creates_new_directory() {
        let config = create_temp_ziggy_config(false, false);
        let instrumenter = Instrumenter::new(config.clone());

        instrumenter.fork().unwrap();
        let files: Vec<_> = WalkDir::new(instrumenter.output_directory())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            .filter(|e| !e.path().components().any(|c| c.as_os_str() == "target"))
            .collect();

        assert_eq!(files.len(), 1); // `lib.rs`
    }

    #[test]
    fn test_build_successful() -> EmptyResult {
        let config = create_temp_ziggy_config(false, false);

        let instrumenter = Instrumenter::new(config);
        let a = instrumenter.clone().instrument();
        let b = instrumenter.build();
        assert!(a.is_ok(), "{}", format!("{:?}", a.unwrap_err()));
        assert!(b.is_ok(), "{}", format!("{:?}", b.unwrap_err()));
        Ok(())
    }
}
