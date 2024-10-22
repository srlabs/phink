use anyhow::{
    bail,
    Context,
};
use quote::quote;
use std::{
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
/// Invokes `println!` only if `verbose` is `true`
macro_rules! phink_log {
    ($self:expr, $($arg:tt)*) => {
        if $self.verbose() {
            println!($($arg)*);
        }
    };
}

pub trait ContractVisitor {
    fn input_directory(&self) -> PathBuf;
    fn output_directory(&self) -> PathBuf;
    fn verbose(&self) -> bool;

    /// Execute `fn_manipulate` for each Rust file (*.rs) inside `from`, except *.rs contained
    /// inside target/
    fn for_each_file<F>(from: PathBuf, mut fn_manipulate: F) -> anyhow::Result<()>
    where
        F: FnMut(PathBuf) -> anyhow::Result<()>,
    {
        for entry in WalkDir::new(&from)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            .filter(|e| !e.path().components().any(|c| c.as_os_str() == "target"))
        {
            fn_manipulate(PathBuf::from(entry.path()))?;
        }
        Ok(())
    }
    fn fork(&self) -> anyhow::Result<PathBuf> {
        let new_dir = self.output_directory();
        let c_path = self.input_directory();

        phink_log!(self, "🏗️ Creating new directory {:?}", new_dir.display());

        fs::create_dir_all(new_dir.clone())
            .with_context(|| format!("🙅 Failed to create directory: {new_dir:?}"))?;

        phink_log!(self, "📁 Starting to copy files from {c_path:?}",);

        for entry in WalkDir::new(&c_path) {
            let entry = entry?;
            let target_path = new_dir.join(
                entry
                    .path()
                    .strip_prefix(&c_path)
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

                copy(entry.path(), &target_path)
                    .with_context(|| format!("🙅 Failed to copy file to {target_path:?}"))?;
            }
        }

        Ok(new_dir.clone())
    }

    /// Go to a contract path and compile the contract with the phink feature.
    fn build(&self) -> anyhow::Result<()> {
        let path = self.output_directory();
        let p_display = &path.display();
        if !path.exists() {
            bail!("There was probably a fork issue, as {p_display} doesn't exist.")
        }

        let clippy_d = Self::create_temp_clippy()?;

        phink_log!(self, "✂️ Creating `{}` to bypass errors", clippy_d);

        // We must NOT compile in release mode (`--release`), otherwise we won't receive the
        // `debug_println`
        let output = Command::new("cargo")
            .current_dir(&path)
            .env("RUST_BACKTRACE", "1")
            .env("CLIPPY_CONF_DIR", clippy_d)
            .args(["contract", "build", "--features=phink"])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            phink_log!(
                self,
                "✂️ Compiling `{p_display}` finished successfully!\n{stdout}\n{stderr}",
            );
        } else {
            bail!(
                "{stderr} - {stdout}\n\n\nIt seems that your instrumented smart contract did not compile properly. \
        Please go to `{p_display}`, edit the source code, and run `cargo contract build --features phink` again. It might be because your contract has a bug inside, or because you haven't created any invariants for instance. \
        \nAlso, make sur that your `Cargo.toml` contains the `phink` feature. It can also be that you need to recompile the contract, as you've changed the toolchain.\
        \nMore informations in the stacktrace above.",
            )
        }

        Ok(())
    }

    fn visit_code(code: &str, mut visitor: impl VisitMut) -> anyhow::Result<String> {
        let mut ast = parse_file(code)?;
        visitor.visit_file_mut(&mut ast);
        Ok(quote!(#ast).to_string())
    }

    fn save(source_code: String, rust_file: &Path) -> anyhow::Result<()> {
        let mut file = File::create(rust_file)?;
        file.write_all(source_code.as_bytes())?;
        println!("✍️ Writing instrumented source code");
        file.flush()?;
        Ok(())
    }

    fn format(rust_file: PathBuf) -> anyhow::Result<()> {
        println!("🛠️ Formatting {} with `rustfmt`...", rust_file.display());
        Command::new("rustfmt")
            .args([rust_file.display().to_string().as_str(), "--edition=2021"])
            .status()?;
        Ok(())
    }

    /// Return a full path to a temporary `clippy.toml`
    /// Create a temporary `clippy.toml` file and return its full path.
    ///
    /// # Returns
    /// `Result<String>` containing the canonicalized path of the temporary file as a `String`.
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
}
