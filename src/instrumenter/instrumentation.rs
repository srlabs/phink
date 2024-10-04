use crate::{
    cli::ziggy::ZiggyConfig,
    instrumenter::instrumentation::instrument::ContractCovUpdater,
};
use anyhow::{
    anyhow,
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
/// Invokes `println!` only if `verbose` is `true`
macro_rules! phink_log {
    ($self:expr, $($arg:tt)*) => {
        if $self.to_owned().verbose() {
            println!($($arg)*);
        }
    };
}
impl Instrumenter {
    pub fn new(z_config: ZiggyConfig) -> Self {
        Self { z_config }
    }

    pub fn verbose(self) -> bool {
        self.z_config.config().verbose
    }

    pub fn find(&self) -> anyhow::Result<InkFilesPath> {
        let c_path = self.z_config.config().instrumented_contract();
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

    /// Go to the instrumented path and compile the contract with the phink feature.
    /// # Important
    /// This function needs to be called after once `instrument()` succeded !
    pub fn build(self) -> anyhow::Result<()> {
        let instrumenter = self.to_owned();
        let path = instrumenter.z_config.config().instrumented_contract();
        let p_display = path.display();
        if !path.exists() {
            bail!("There was probably a fork issue, as {p_display} doesn't exist.")
        }

        let clippy_d = Self::create_temp_clippy()?;

        phink_log!(self, "✂️ Creating `{}` to bypass errors", clippy_d);

        // We must NOT compile in release mode (--release), otherwise we won't receive the
        // debug_pritntln
        let output = Command::new("cargo")
            .current_dir(path.as_path())
            .env("RUST_BACKTRACE", "1")
            .env("CLIPPY_CONF_DIR", clippy_d)
            .args(["contract", "build", "--features=phink"])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            phink_log!(
                self,
                "✂️ Compiling `{p_display}` finished successfully!\n{stdout}\n{stderr}",
            );
        } else {
            bail!(
                "It seems that your instrumented smart contract did not compile properly. \
        Please go to `{p_display}`, edit the source code, and run `cargo contract build --features phink` again. It might be because your contract has a bug inside, or because you haven't created any invariants for instance.\
        Also, make sur that your Cargo.toml contains the `phink` feature.",
            )
        }

        println!(
            "\n🤞 Contract '{}' has been instrumented and compiled.\n🤞 You can find the instrumented contract in '{p_display}'",
            self.z_config.contract_path().display(),
        );

        Ok(())
    }

    /// Return a full path to a temporary `clippy.toml`
    /// Create a temporary `clippy.toml` file and return its full path.
    ///
    /// # Returns
    /// A `Result` containing the canonicalized path of the temporary file as a `String`.
    fn create_temp_clippy() -> anyhow::Result<String> {
        let temp_dir = tempfile::TempDir::new().context("Failed to create temporary directory")?;
        let clippy_toml_path = temp_dir.path().join("clippy.toml");

        let mut clippy_toml =
            File::create(&clippy_toml_path).context("Failed to create clippy.toml file")?;

        writeln!(clippy_toml, "avoid-breaking-exported-api = false")
            .context("Failed to write to clippy.toml file")?;

        let temp_dir_path = temp_dir.into_path();
        let temp_dir_str = temp_dir_path
            .to_str()
            .context("Failed to convert temporary directory path to string")?
            .to_string();

        Ok(temp_dir_str + "/clippy.toml")
    }
    fn fork(self) -> anyhow::Result<PathBuf> {
        let new_dir = &self.z_config.config().instrumented_contract();

        phink_log!(self, "🏗️ Creating new directory: '{}'", new_dir.display());

        fs::create_dir_all(new_dir)
            .with_context(|| format!("🙅 Failed to create directory: {}", new_dir.display()))?;

        phink_log!(
            self,
            "📁 Starting to copy files from {:?}",
            self.z_config.contract_path()
        );

        for entry in WalkDir::new(self.z_config.contract_path()) {
            let entry = entry?;
            let target_path = new_dir.join(
                entry
                    .path()
                    .strip_prefix(self.z_config.contract_path())
                    .with_context(|| "Couldn't `strip_prefix`")?,
            );

            if entry.path().is_dir() {
                phink_log!(self, "📂 Creating subdirectory: {:?}", target_path);
                fs::create_dir_all(&target_path)?;
            } else {
                phink_log!(
                    self,
                    "📄 Copying file: {:?} -> {target_path:?}",
                    entry.path(),
                );

                copy(entry.path(), &target_path).with_context(|| {
                    format!("🙅 Failed to copy file to {}", target_path.display())
                })?;
            }
        }

        println!(
            "✅ Fork completed successfully! New directory: {:?}",
            new_dir
        );
        Ok(new_dir.to_path_buf())
    }

    pub(crate) fn instrument(self) -> anyhow::Result<()> {
        let new_working_dir = self
            .to_owned()
            .fork()
            .with_context(|| "Forking the project to a new directory failed".to_string())?;

        for entry in WalkDir::new(&new_working_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            .filter(|e| !e.path().components().any(|c| c.as_os_str() == "target"))
        // Don't instrument anything inside `target` and only `instrument_file` Rust files
        {
            let path = entry.path();
            self.instrument_file(path).with_context(|| {
                format!(
                    "Instrumenting the file {} wasn't possible",
                    path.to_str().unwrap()
                )
            })?;
        }
        Ok(())
    }

    fn instrument_file(&self, path: &Path) -> anyhow::Result<()> {
        let contract_cov_manager = ContractCovUpdater::new();
        let code = fs::read_to_string(path)?;

        if Self::already_instrumented(&code) {
            println!(
                "{} was already instrumented",
                path.to_path_buf().file_name().unwrap().display()
            );
            return Ok(())
        }

        println!("📝 Instrumenting {}", path.display());

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
        println!("🛠️ Formatting {} with `rustfmt`...", rust_file.display());
        Command::new("rustfmt")
            .args([rust_file.display().to_string().as_str(), "--edition=2021"])
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

    impl ContractCovUpdater {
        pub fn new() -> Self {
            Self { line_id: 0 }
        }
    }

    impl VisitMut for ContractCovUpdater {
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
    use crate::{
        cli::config::Configuration,
        instrumenter::path::InstrumentedPath,
    };
    use std::{
        default::Default,
        fs::{
            self,
            File,
        },
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

        let result = instrumenter.fork().unwrap();
        let files: Vec<_> = WalkDir::new(&result)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            .filter(|e| !e.path().components().any(|c| c.as_os_str() == "target"))
            .collect();

        assert_eq!(files.len(), 1); // `lib.rs`
    }

    #[test]
    fn test_build_successful() -> anyhow::Result<()> {
        let config = create_temp_ziggy_config(false, false);

        let instrumenter = Instrumenter::new(config);
        let a = instrumenter.clone().instrument();
        let b = instrumenter.build();
        assert!(a.is_ok(), "{}", format!("{:?}", a.unwrap_err()));
        assert!(b.is_ok(), "{}", format!("{:?}", b.unwrap_err()));
        Ok(())
    }
}
