use regex::Regex;
use std::{
    ffi::OsStr,
    fs,
    fs::{
        copy,
        File,
    },
    io,
    io::Write,
    path::{
        Path,
        PathBuf,
    },
    process::Command,
};

use crate::{
    cli::ziggy::ZiggyConfig,
    instrumenter::{
        instrumentation::instrument::ContractCovUpdater,
        instrumented_path::InstrumentedPath,
    },
};
use quote::quote;
use syn::{
    parse_file,
    visit_mut::VisitMut,
};
use walkdir::WalkDir;

/// The objective of this `struct` is to assist Phink in instrumenting ink!
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

#[derive(Debug)]
pub struct InkFilesPath {
    pub wasm_path: PathBuf,
    pub specs_path: PathBuf,
}

pub trait ContractInstrumenter {
    fn instrument(&mut self) -> Result<&mut Self, String>
    where
        Self: Sized;
    fn instrument_file(
        &self,
        path: &Path,
        contract_cov_manager: &mut ContractCovUpdater,
    ) -> Result<(), String>;
    fn parse_and_visit(code: &str, visitor: impl VisitMut) -> Result<String, ()>;
    fn save_and_format(source_code: String, lib_rs: PathBuf) -> Result<(), io::Error>;
    fn already_instrumented(code: &str) -> bool;
}

impl Instrumenter {
    pub fn new(z_config: ZiggyConfig) -> Self {
        Self { z_config }
    }

    pub fn find(&self) -> Result<InkFilesPath, String> {
        let wasm_path = fs::read_dir(self.z_config.contract_path.join("target/ink/"))
            .map_err(|e| {
                format!(
                    "🙅 It seems that your contract is not compiled into `target/ink`. \
             Please, ensure that your the WASM blob and the JSON specs are stored into \
             '{}target/ink/' (more infos: {})",
                    self.z_config.contract_path.to_str().unwrap(),
                    e
                )
            })?
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
            .ok_or("🙅 No .wasm file found in target directory")?;

        let specs_path =
            PathBuf::from(wasm_path.to_str().unwrap().replace(".wasm", ".json"));

        Ok(InkFilesPath {
            wasm_path,
            specs_path,
        })
    }
}
pub trait ContractBuilder {
    fn build(&self) -> Result<InkFilesPath, String>;
}

impl ContractBuilder for Instrumenter {
    fn build(&self) -> Result<InkFilesPath, String> {
        let status = Command::new("cargo")
            .current_dir(&self.z_config.contract_path)
            .args(["contract", "build", "--features=phink"])
            .status()
            .map_err(|e| {
                format!(
                    "🙅 Failed to execute cargo command: {}.\
            The command was simply 'cargo contract build --features=phink",
                    e
                )
            })?;

        if status.success() {
            self.find()
        } else {
            Err(format!(
                "🙅 It seems that your instrumented smart contract did not compile properly. \
                Please go to {}, edit the `lib.rs` file, and run cargo contract build again.\
                (more infos: {})",
                &self.z_config.contract_path.display(),
                status
            ))
        }
    }
}
pub trait ContractForker {
    fn fork(&self) -> Result<PathBuf, String>;
}
impl ContractForker for Instrumenter {
    fn fork(&self) -> Result<PathBuf, String> {
        let new_dir = &self
            .z_config
            .config
            .instrumented_contract_path
            .clone()
            .unwrap_or(InstrumentedPath::default())
            .path;

        println!("🏗️ Creating new directory: {:?}", new_dir);
        fs::create_dir_all(&new_dir)
            .map_err(|e| format!("🙅 Failed to create directory: {}", e))?;

        println!(
            "📁 Starting to copy files from {:?}",
            self.z_config.contract_path
        );

        for entry in WalkDir::new(&self.z_config.contract_path) {
            let entry = entry.map_err(|e| format!("🙅 Failed to read entry: {}", e))?;
            let target_path = new_dir.join(
                entry
                    .path()
                    .strip_prefix(&self.z_config.contract_path)
                    .map_err(|e| format!("🙅 Failed to strip prefix: {}", e))?,
            );

            if entry.path().is_dir() {
                println!("📂 Creating subdirectory: {:?}", target_path);
                fs::create_dir_all(&target_path)
                    .map_err(|e| format!("🙅 Failed to create subdirectory: {}", e))?;
            } else {
                println!("📄 Copying file: {:?} -> {:?}", entry.path(), target_path);
                copy(entry.path(), &target_path)
                    .map_err(|e| format!("🙅 Failed to copy file: {}", e))?;
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
    fn instrument(&mut self) -> Result<&mut Instrumenter, String> {
        let new_working_dir = self.fork()?;
        // self.z_config.contract_path = new_working_dir.clone(); //todo probably bugged
        // here
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
    ) -> Result<(), String> {
        let code = fs::read_to_string(path)
            .map_err(|e| format!("🙅 Failed to read {}: {:?}", path.display(), e))?;

        if Self::already_instrumented(&code) {
            return Ok(());
        }

        println!(
            "📝 Instrumenting file: {} with {:?}",
            path.display(),
            contract_cov_manager
        );

        let modified_code =
            Self::parse_and_visit(&code, contract_cov_manager).map_err(|_| {
                format!("🙅 Failed to parse and visit code in {}", path.display())
            })?;

        Self::save_and_format(modified_code, PathBuf::from(path)).map_err(|e| {
            format!(
                "🙅 Failed to save and format code in {}: {:?}",
                path.display(),
                e
            )
        })?;

        Ok(())
    }

    fn parse_and_visit(code: &str, mut visitor: impl VisitMut) -> Result<String, ()> {
        let mut ast = parse_file(code).expect(
            "⚠️ This is most likely that your ink! contract \
        contains invalid syntax. Try to compile it first. Also, ensure that `cargo-contract` is installed.",
        );
        visitor.visit_file_mut(&mut ast);

        Ok(quote!(#ast).to_string())
    }

    fn save_and_format(source_code: String, rust_file: PathBuf) -> Result<(), io::Error> {
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
    /// `ink::env::debug_println!("COV=abc")` where `abc` can be any number. If
    /// this pattern is found, it means the code is instrumented.
    fn already_instrumented(code: &str) -> bool {
        Regex::new(r#"\bink::env::debug_println!\("COV=\d+"\)"#)
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

    #[derive(Debug)]
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
                let line_lit =
                    LitInt::new(self.line_id.to_string().as_str(), Span::call_site());

                self.line_id = self.line_id + 1;

                let insert_expr: Expr = parse_quote! {
                    ink::env::debug_println!("COV={}", #line_lit)
                };
                // Convert this expression into a statement
                let pre_stmt: Stmt =
                    Stmt::Expr(insert_expr, Some(Token![;](Span::call_site())));
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
