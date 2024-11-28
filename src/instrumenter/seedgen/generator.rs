use crate::{
    instrumenter::traits::visitor::ContractVisitor,
    EmptyResult,
    ResultOf,
};
use std::borrow::BorrowMut;

use crate::{
    cli::config::PhinkFiles,
    fuzzer::environment::CorpusManager,
    instrumenter::seedgen::parser::SeedExtractor,
};
use anyhow::{
    bail,
    Context,
};
use quote::quote;
use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
    process::Command,
};
use syn::{
    parse_quote,
    visit_mut::{
        self,
        VisitMut,
    },
    FnArg,
    ImplItem,
    ImplItemFn,
    Item,
    ItemImpl,
    ItemMod,
    Pat,
    Stmt,
};
use toml_edit::{
    DocumentMut,
    Formatted,
    Value,
};

#[derive(Debug, Clone)]
pub struct SeedExtractInjector {
    contract_path: PathBuf,
    compiled_path: Option<PathBuf>,
}

impl SeedExtractInjector {
    pub fn new(contract_path: &Path, compiled_path: Option<PathBuf>) -> ResultOf<Self> {
        if !contract_path.exists() {
            bail!("Couldn't find the contract at {contract_path:?}")
        }
        Ok(Self {
            contract_path: contract_path.to_path_buf(),
            compiled_path,
        })
    }

    /// Fork the contract, insert the snippet to extract the seeds, patch `Cargo.toml`, run the
    /// tests, extracts the seeds.
    pub fn extract(&mut self, output: &PathBuf) -> EmptyResult {
        self.fork()
            .context("Forking the project to a new directory failed")?;

        self.insert_snippet()
            .context("Inserting the snippet into the file for seed extraction wasn't possible")?;

        let is_e2e: bool = self
            .patch_toml()
            .context("Patching Cargo.toml for seed extraction wasn't possible")?;

        self.build()
            .context("Couldn't build the contract required for seed extraction. Try removing `Cargo.lock` from your contract")?;

        let unparsed_seed = self
            .run_tests(is_e2e)
            .context("Couldn't run `cargo test ...` to run the seeds")?;

        let amount = self.save_seeds(unparsed_seed, output)?;

        if self.verbose() {
            if is_e2e {
                println!(
                    "The contract has the e2e-tests features, so we run seed generator with E2E."
                );
            }
            println!(
                "\nDone! We've saved {amount} seeds in total. If your campaign already started,\
             you can use `cargo ziggy add-seeds` to include the seeds."
            );
        }
        Ok(())
    }

    /// Save all the seeds properly to a .bin file
    /// # Returns
    /// The number of seeds saved
    fn save_seeds(&self, unparsed_seed: String, output: &PathBuf) -> ResultOf<usize> {
        let seeds_as_bin = SeedExtractor::new(unparsed_seed).extract_seeds();
        let pfile = PhinkFiles::new_by_ref(output);
        let writer = &CorpusManager::new(&pfile)?;
        for (i, seed) in seeds_as_bin.iter().enumerate() {
            let bytes = seed.as_ref();
            let name = format!("seedgen_{i}");
            writer.write_seed(name.as_str(), bytes)?;
            if self.verbose() {
                let path = format!("{pfile}/phink/corpus/{name}.bin");
                println!("Writing bytes 0x{} to `{path}`", hex::encode(bytes));
            }
        }

        Ok(seeds_as_bin.len())
    }

    pub fn run_tests(&self, is_e2e: bool) -> ResultOf<String> {
        let path = self.output_directory();
        let p_display = &path.display();

        if !path.exists() {
            bail!("There was probably a fork issue, as {p_display} doesn't exist.")
        }

        let mut args = vec!["test"];
        if is_e2e {
            args.push("--features=e2e-tests");
        }
        args.extend_from_slice(&["--", "--show-output"]);

        let output = Command::new("cargo")
            .current_dir(&path)
            .args(args)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            if self.verbose() {
                println!(
                    "{stdout}\n\n\n=========================================\n\n\
                You can find the stringified seeds below. \
                No worries, they're already saved with the correct format to the corpus. \
                You don't need to do anything.\n"
                );
            }
            Ok(stdout.parse()?)
        } else {
            bail!(
                "{stderr} - {stdout}\n\n\nIt seems that we couldn't run the E2E tests or unit tests for your contract. \
        Please go to `{p_display}`, edit the source code, and run `cargo test --features=e2e-tests -- --show-output`.\n\
        It might be because your contract cannot run the tests properly. Maybe additionnal parameters to `cargo test` are required ? \
        \nMore informations in the stacktrace above.",
            )
        }
    }

    fn collect_file_paths(&self) -> ResultOf<Vec<PathBuf>> {
        let mut paths = Vec::new();
        self.for_each_file(|file_path| {
            paths.push(file_path);
            Ok(())
        })
        .context("Couldn't fetch the contract's files")?;
        Ok(paths)
    }

    /// Insert the snippet that will extract each call and send it via `debug_println!`
    fn insert_snippet(&mut self) -> EmptyResult {
        let paths: Vec<PathBuf> = self.collect_file_paths()?;
        for file_path in paths {
            let source_code =
                fs::read_to_string(&file_path).context(format!("Couldn't read {file_path:?}"))?;

            self.instrument_file(file_path, &source_code, &mut self.clone().borrow_mut())
                .context("Failed to instrument the file")?;
        }

        Ok(())
    }

    /// Patch the `Cargo.toml` to use our own version of ink!
    /// # Returns
    /// Returns `Result<true>` if the Cargo.toml also has `e2e-test` enabled
    fn patch_toml(&self) -> ResultOf<bool> {
        // TODO: Seed extraction if we have multiple contracts, so multiple Cargo.toml
        let cargo_path = &self.output_directory().join("Cargo.toml");
        let cargo_content = fs::read_to_string(cargo_path)?;
        if !cargo_content.contains("ink_prelude =") {
            bail!(format!(
                "Please, add ink_prelude dependency to your project.\n\
            You can add `ink_prelude = {{ version = \"5.0.0\", default-features = false }}`"
            ))
        }
        let mut doc = cargo_content.parse::<DocumentMut>()?;
        const REPO: &str = "https://github.com/kevin-valerio/ink";

        // Function to update ink dependencies in a table
        fn update_ink_deps(table: &mut toml_edit::Table) {
            for (key, value) in table.iter_mut() {
                if let toml_edit::Item::Value(Value::InlineTable(dep_table)) = value {
                    // Only modify if it's a table and contains a version
                    if dep_table.contains_key("version") {
                        // Check package name if specified, otherwise use the key
                        let dep_name = dep_table
                            .get("package")
                            .and_then(|v| v.as_str())
                            .unwrap_or(&*key);

                        if dep_name.starts_with("ink_") || dep_name == "ink" {
                            dep_table.insert(
                                "git",
                                Value::String(Formatted::new(REPO.parse().unwrap())),
                            );
                        }
                    }
                }
            }
        }

        if let Some(deps) = doc.get_mut("dependencies").and_then(|d| d.as_table_mut()) {
            update_ink_deps(deps);
        }

        if let Some(dev_deps) = doc
            .get_mut("dev-dependencies")
            .and_then(|d| d.as_table_mut())
        {
            update_ink_deps(dev_deps);
        }

        fs::write(cargo_path, doc.to_string())?;
        Ok(cargo_content.contains("ink_e2e = "))
    }

    /// Check if the function has the `#[ink(message)]` attribute
    fn has_ink_message_attribute(i: &mut ImplItemFn) -> bool {
        for attr in &i.attrs {
            if attr.path().is_ident("ink") {
                let res = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("message") {
                        Ok(())
                    } else {
                        Err(meta.error("Not an ink! message"))
                    }
                });
                return res.is_ok();
            }
        }
        false
    }
}

impl ContractVisitor for SeedExtractInjector {
    fn input_directory(&self) -> PathBuf {
        self.contract_path.to_path_buf()
    }

    fn output_directory(&self) -> PathBuf {
        match &self.compiled_path {
            None => {
                // Create a new directory that is not tied to `TempDir`
                let dir = std::env::temp_dir().join("phink_seedgen");
                fs::create_dir_all(&dir).expect("Failed to create output directory for seedgen");
                dir
            }
            Some(contract) => contract.to_path_buf(),
        }
    }

    fn verbose(&self) -> bool {
        true
    }
}

impl VisitMut for &mut SeedExtractInjector {
    fn visit_item_mut(&mut self, item: &mut Item) {
        match item {
            Item::Fn(f) => self.visit_item_fn_mut(f),
            Item::Mod(m) => self.visit_item_mod_mut(m),
            Item::Impl(i) => self.visit_item_impl_mut(i),
            _ => visit_mut::visit_item_mut(self, item),
        }
    }

    // Visit all items inside `impl` blocks
    fn visit_item_impl_mut(&mut self, i: &mut ItemImpl) {
        for item in &mut i.items {
            if let ImplItem::Fn(method) = item {
                if SeedExtractInjector::has_ink_message_attribute(method) {
                    let fn_name = &method.sig.ident;
                    // If the visited function isn't an invariant
                    if !fn_name.to_string().starts_with("phink_") {
                        // Args of the ink! message
                        let args: Vec<syn::Ident> = method
                            .sig
                            .inputs
                            .iter()
                            .filter_map(|arg| {
                                if let FnArg::Typed(pat_type) = arg {
                                    if let Pat::Ident(pat_ident) = &*pat_type.pat {
                                        Some(pat_ident.ident.clone())
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                            .collect();

                        let fn_name_str = fn_name.to_string();

                        let mut push_args = quote! {
                            use ink_prelude::format;
                            let mut toz = ink::env::call::ExecutionInput::new(
                                ink::env::call::Selector::new(ink::selector_bytes!(#fn_name_str))
                            )
                        };

                        for arg in args {
                            push_args = quote! {
                                #push_args
                                .push_arg(&#arg)
                            };
                        }

                        // Full snippet
                        let snippet: Stmt = parse_quote! {
                            {
                            #push_args;
                            let encoded = ink::scale::Encode::encode(&toz);
                            ink::env::debug_println!("ENCODED_SEED={}", encoded.iter().map(|byte| format!("{:02x}", byte)).collect::<ink_prelude::string::String>());
                            }
                        };

                        let stmts = &mut method.block.stmts;
                        stmts.insert(0, snippet);
                    }
                }
            }
        }
        visit_mut::visit_item_impl_mut(self, i);
    }

    fn visit_item_mod_mut(&mut self, i: &mut ItemMod) {
        if let Some(ref mut content) = i.content {
            for item in content.1.iter_mut() {
                self.visit_item_mut(item);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instrumenter::seedgen::generator::SeedExtractInjector;
    use quote::quote;
    use std::path::PathBuf;
    use syn::{
        parse_str,
        visit_mut::VisitMut,
        File,
    };

    #[test]
    fn test_seed_injector() {
        let input_code = r#"
                #![cfg_attr(not(feature = "std"), no_std, no_main)]
                #[ink::contract]
                mod dummy {
                    use ink::{
                        prelude::vec::Vec,
                        storage::{
                            Mapping,
                            StorageVec,
                        },
                    };
                    use ink_prelude::string::String;

                    #[ink(storage)]
                    #[derive(Default)]
                    pub struct MyBuggedContract {
                        forbidden_number: u32,
                    }

                    #[derive(Debug, PartialEq, Eq)]
                    #[ink::scale_derive(Encode, Decode, TypeInfo)]
                    pub enum Error {}
                    pub type Result<T> = core::result::Result<T, Error>;
                    impl MyBuggedContract {
                        /// Creates a new domain name service contract.
                        #[ink(constructor)]
                        pub fn new() -> Self {
                            Default::default()
                        }

                        #[ink(message)]
                        pub fn crash_with_invariant(&mut self, data: String) -> Result<()> {
                            if data.len() == 4 {
                                if data.chars().nth(0).unwrap() == 'f' {
                                    if data.chars().nth(1).unwrap() == 'u' {
                                        if data.chars().nth(2).unwrap() == 'z' {
                                            if data.chars().nth(3).unwrap() == 'z' {
                                                self.forbidden_number = 42;
                                            }
                                        }
                                    }
                                }
                            }
                            Ok(())
                        }

                        #[ink(message)]
                        pub fn toz(a: u32, name: Hash) {
                            let a = 1 + 1;
                        }

                         #[ink(message)]
                        pub fn abcccc(&self, a: u32, name: Hash) {
                            let bbbb = 3 + 1;
                        }
                    }

                    #[cfg(test)]
                    mod tests {
                        use super::*;

                        #[ink::test]
                        fn new_works() {
                            let mut a = MyBuggedContract::new();
                            // a.toz(32, Hash::from([0x99; 32]));
                            a.crash_with_invariant("xxx".to_string()).unwrap();
                        }
                    }

                    #[cfg(feature = "phink")]
                    #[ink(impl)]
                    impl MyBuggedContract {
                        // This invariant ensures that nobody register the forbidden number
                        #[cfg(feature = "phink")]
                        #[ink(message)]
                        pub fn phink_assert_dangerous_number(&self) {
                            let forbidden_number = 42;
                            assert_ne!(self.forbidden_number, forbidden_number);
                        }
                    }
                }"#;

        let mut syntax_tree: File = parse_str(input_code).expect("Failed to parse code");
        let mut seed_injector =
            &mut SeedExtractInjector::new(&PathBuf::from("sample/dummy"), None).unwrap();
        seed_injector.visit_file_mut(&mut syntax_tree);

        let generated_code = quote!(#syntax_tree).to_string();
        // println!("{generated_code}");

        let expected_snippet = r#"let mut toz = ink :: env :: call :: ExecutionInput :: new (ink :: env :: call :: Selector :: new (ink :: selector_bytes ! ("toz"))) . push_arg (& a) . push_arg (& name) ;"#;
        assert!(
            generated_code.contains(&expected_snippet.to_string()),
            "The code snippet was not injected correctly. Expected: \n{expected_snippet} == \n to be contained in: \n\n{generated_code} ==\n"
        );
    }
}
