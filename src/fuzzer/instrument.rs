use std::ffi::OsStr;
use std::fs;
use std::fs::{copy, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use quote::quote;
use rand::distributions::Alphanumeric;
use rand::Rng;
use syn::parse_file;
use syn::visit_mut::VisitMut;
use walkdir::WalkDir;

use crate::fuzzer::instrument::instrument::ContractCovUpdater;

/// The objective of this `struct` is to assist Phink in instrumenting ink! smart contracts.
/// In a fuzzing context, instrumenting a smart contract involves modifying the target (i.e., the WASM blob),
/// for example, by adding additional code to branches to obtain a coverage map during the execution of the smart contract.
/// By doing so, we can effectively generate a coverage map that will be provided to Ziggy
/// or LibAFL, transforming Phink from a basic brute-forcing tool into a powerful coverage-guided fuzzer.
///
/// Phink opted for a Rust AST approach. For each code instruction on the smart-contract, Phink will
/// automatically add a tracing code, which will then be fetched at the end of the input execution
/// in order to get coverage.
#[derive(Default)]
pub struct CoverageEngine {
    pub contract_dir: PathBuf,
}

#[derive(Debug)]
pub struct InkFilesPath {
    pub wasm_path: PathBuf,
    pub specs_path: PathBuf,
}

pub trait ContractBuilder {
    fn build(&self) -> Result<InkFilesPath, String>;
}

pub trait ContractForker {
    fn fork(&self) -> Result<PathBuf, String>;
}

pub trait ContractInstrumenter {
    fn instrument(&mut self) -> Result<&mut Self, String>
    where
        Self: Sized;
    fn parse_and_visit(code: &str, visitor: impl VisitMut) -> Result<String, ()>;
    fn save_and_format(source_code: String, lib_rs: PathBuf) -> Result<(), std::io::Error>;
}

impl CoverageEngine {
    pub fn new(dir: PathBuf) -> Self {
        Self { contract_dir: dir }
    }
}

impl ContractBuilder for CoverageEngine {
    fn build(&self) -> Result<InkFilesPath, String> {
        let status = Command::new("cargo")
            .current_dir(&self.contract_dir)
            .args(["contract", "build", "--features=phink"])
            .status()
            .map_err(|e| format!("Failed to execute cargo command: {:?}", e))?;

        if status.success() {
            let wasm_path = fs::read_dir(self.contract_dir.join("target/ink/"))
                .map_err(|e| format!("Failed to read target directory: {:?}", e))?
                .filter_map(|entry| {
                    let path = entry.ok()?.path();
                    if path.is_file() && path.extension().and_then(OsStr::to_str) == Some("wasm") {
                        Some(path)
                    } else {
                        None
                    }
                })
                .next()
                .ok_or("No .wasm file found in target directory")?;

            let specs_path = PathBuf::from(wasm_path.to_str().unwrap().replace(".wasm", ".json"));

            Ok(InkFilesPath {
                wasm_path,
                specs_path,
            })
        } else {
            Err(format!(
                "It seems that your instrumented smart contract did not compile properly. \
                Please go to {:?}, edit the lib.rs file, and run cargo contract build again.\
                Detailed error — {:?}",
                &self.contract_dir, status
            ))
        }
    }
}

impl ContractForker for CoverageEngine {
    fn fork(&self) -> Result<PathBuf, String> {
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();

        let new_dir = Path::new("/tmp").join(format!("ink_fuzzed_{}", random_string));
        fs::create_dir_all(&new_dir).map_err(|e| format!("Failed to create directory: {:?}", e))?;

        for entry in WalkDir::new(&self.contract_dir) {
            let entry = entry.map_err(|e| format!("Failed to read entry: {:?}", e))?;
            let target_path = new_dir.join(
                entry
                    .path()
                    .strip_prefix(&self.contract_dir)
                    .map_err(|e| format!("Failed to strip prefix: {:?}", e))?,
            );

            if entry.path().is_dir() {
                fs::create_dir_all(&target_path)
                    .map_err(|e| format!("Failed to create subdirectory: {:?}", e))?;
            } else {
                copy(entry.path(), &target_path)
                    .map_err(|e| format!("Failed to copy file: {:?}", e))?;
            }
        }

        Ok(new_dir)
    }
}

impl ContractInstrumenter for CoverageEngine {
    fn instrument(&mut self) -> Result<&mut CoverageEngine, String> {
        let new_working_dir = self.fork()?;
        let lib_rs = new_working_dir.join("lib.rs");
        let code =
            fs::read_to_string(&lib_rs).map_err(|e| format!("Failed to read lib.rs: {:?}", e))?;

        let modified_code = Self::parse_and_visit(&code, ContractCovUpdater)
            .map_err(|_| "Failed to parse and visit code".to_string())?;

        Self::save_and_format(modified_code, lib_rs.clone())
            .map_err(|e| format!("Failed to save and format code: {:?}", e))?;

        self.contract_dir = new_working_dir;
        Ok(self)
    }

    fn parse_and_visit(code: &str, mut visitor: impl VisitMut) -> Result<String, ()> {
        let mut ast = parse_file(code).unwrap();
        visitor.visit_file_mut(&mut ast);
        Ok(quote!(#ast).to_string())
    }

    fn save_and_format(source_code: String, lib_rs: PathBuf) -> Result<(), std::io::Error> {
        let mut file = File::create(lib_rs.clone())?;
        file.write_all(source_code.as_bytes())?;
        file.flush()?;
        Command::new("rustfmt").arg(lib_rs).status()?;
        Ok(())
    }
}

mod instrument {
    use proc_macro2::Span;
    use syn::{parse_quote, spanned::Spanned, visit_mut::VisitMut, Expr, LitInt, Stmt, Token};

    pub struct ContractCovUpdater;

    impl VisitMut for ContractCovUpdater {
        fn visit_block_mut(&mut self, block: &mut syn::Block) {
            let mut new_stmts = Vec::new();
            // Temporarily replace block.stmts with an empty Vec to avoid borrowing issues
            let mut stmts = std::mem::replace(&mut block.stmts, Vec::new());
            for mut stmt in stmts.drain(..) {
                let line_lit =
                    LitInt::new(&stmt.span().start().line.to_string(), Span::call_site());
                let insert_expr: Expr = parse_quote! {
                    ink::env::debug_println!("COV={}", #line_lit)
                };
                // Convert this expression into a statement
                let pre_stmt: Stmt = Stmt::Expr(insert_expr, Some(Token![;](Span::call_site())));
                new_stmts.push(pre_stmt);
                // Use recursive visitation to handle nested blocks and other statement types
                self.visit_stmt_mut(&mut stmt);
                new_stmts.push(stmt.clone());
            }
            block.stmts = new_stmts;
        }
    }
}

mod test {
    use parity_scale_codec::{Decode, DecodeLimit};
    use std::path::PathBuf;
    use std::{fs, fs::File, io::Write, process::Command};

    use quote::quote;
    use syn::__private::ToTokens;
    use syn::parse_file;
    use syn::visit_mut::VisitMut;

    use crate::fuzzer::instrument::{ContractForker, CoverageEngine};

    #[test]
    fn adding_cov_insertion_works() {
        let signature = "COV_MAP . insert (";

        let code = fs::read_to_string("sample/dns/lib.rs").unwrap();
        let mut ast = parse_file(&code).expect("Unable to parse file");

        let mut visitor = crate::fuzzer::instrument::instrument::ContractCovUpdater;
        visitor.visit_file_mut(&mut ast);

        let modified_code = quote!(#ast).to_string();
        assert!(modified_code.contains(signature)); //spaces are required :shrug:
        export(modified_code);
    }

    #[test]
    fn do_fork() {
        let engine: CoverageEngine = CoverageEngine::new(PathBuf::from("sample/dns"));
        let fork = engine.fork().unwrap();
        println!("{:?}", fork);
        let exists = fork.exists();
        fs::remove_file(fork).unwrap(); //remove after test passed to avoid spam of /tmp
        assert!(exists);
    }

    /// This function simply saves some `modified_code` Rust code into /tmp/toz.rs
    /// Format it with `rustfmt` and `ccat` it into stdout
    /// Used only for debugging purposes
    fn export(modified_code: String) {
        let mut file = File::create("/tmp/toz.rs").expect("Unable to create file");
        write!(file, "{}", modified_code).expect("Unable to write data");

        Command::new("rustfmt")
            .arg("/tmp/toz.rs")
            .status()
            .expect("Failed to run rustfmt");
        Command::new("ccat")
            .arg("/tmp/toz.rs")
            .arg("--bg=dark")
            .status()
            .expect("Just install ccat... please");
    }
}
