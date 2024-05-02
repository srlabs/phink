use crate::fuzzer::coverage::instrumentor_visitors::{
    ContractCovUpdater, ContractMapInstantiation,
};
use quote::quote;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::{Take, Write};
use std::path::PathBuf;
use syn::parse_file;
use syn::visit_mut::VisitMut;

/// The objective of this `struct` is to assist Phink in instrumenting ink! smart contracts.
/// In a fuzzing context, instrumenting a smart contract involves modifying the target (i.e., the WASM blob),
/// for example, by adding additional code to branches to obtain a coverage map during the execution of the smart contract.
/// By doing so, we can effectively generate a coverage map that will be provided to Ziggy
/// or LibAFL, transforming Phink from a basic brute-forcing tool into a powerful coverage-guided fuzzer.
///
/// Phink opted for a Rust AST approach. For each code instruction on the smart-contract, Phink will
/// automatically add a tracing code, which will then be fetched at the end of the input execution
/// in order to get coverage.

pub struct CoverageEngine {
    pub lib_path: PathBuf,
}

impl CoverageEngine {
    pub fn new(lib_path: PathBuf) -> Self {
        Self { lib_path }
    }

    /// This function _forks_ the code base in order to instrument it safely
    /// The fork is performed by default in `/tmp/`, with the format `/tmp/lib.rs_rndnumber.rs`
    /// Returns the new path of the directory
    fn fork(&self) -> PathBuf {
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5) // Generates a random string of length 5
            .map(char::from)
            .collect();

        let new_path = PathBuf::from("/tmp").join(format!(
            "{}_{}.rs",
            self.lib_path.file_name().unwrap().to_string_lossy(),
            random_string
        ));

        fs::copy(&self.lib_path, &new_path).unwrap();

        new_path
    }
    pub fn instrument(&self) -> Result<(), ()> {
        let forked_lib = self.fork();
        let code = fs::read_to_string(forked_lib).unwrap();
        let mut ast = parse_file(&code).unwrap();

        let mut visitor_updated = ContractCovUpdater;
        visitor_updated.visit_file_mut(&mut ast);
        let mut modified_code = quote!(#ast).to_string();

        ast = parse_file(&modified_code).unwrap();

        let mut visitor_instantiation = ContractMapInstantiation;
        visitor_instantiation.visit_file_mut(&mut ast);
        modified_code = quote!(#ast).to_string();
        save_instrumented(modified_code);
        Ok(())
    }
}

fn save_instrumented(source_code: String) {
    let mut file = File::create("instrumented_lib.rs").unwrap();
    file.write_all(source_code.as_bytes()).unwrap();
    file.flush().unwrap();
}

mod instrumentor_visitors {
    use proc_macro2::{Span, TokenStream};
    use quote::quote;
    use syn::token::Semi;
    use syn::{
        parse_quote,
        spanned::Spanned,
        visit_mut::{self, visit_item_mod_mut, VisitMut},
        Expr, Item, ItemMod, ItemStatic, ItemStruct, LitInt, Stmt, Token,
    };

    pub struct ContractMapInstantiation;

    impl VisitMut for ContractMapInstantiation {
        fn visit_item_mod_mut(&mut self, i: &mut ItemMod) {
            // Add the Coverage struct for mod that only contains #[ink::contract]
            let struct_def: TokenStream = syn::parse2(quote! {
                #[ink(event, anonymous)]
                pub struct Coverage {
                    cov_of: i32,
                }
            })
            .unwrap();

            // Parse the TokenStream into a struct item
            let struct_item: ItemStruct = syn::parse2(struct_def).unwrap();
            let struct_as_item = Item::Struct(struct_item);

            // Insert the struct at the beginning of the module's items
            i.content
                .as_mut()
                .map(|(_, items)| items.insert(0, struct_as_item));

            // Continue traversing the module
            visit_item_mod_mut(self, i);
        }
    }

    pub struct ContractCovUpdater;

    impl VisitMut for ContractCovUpdater {
        fn visit_block_mut(&mut self, block: &mut syn::Block) {
            let mut new_stmts = Vec::new();
            // Temporarily replace block.stmts with an empty Vec to avoid borrowing issues
            let mut stmts = std::mem::replace(&mut block.stmts, Vec::new());

            for mut stmt in stmts.drain(..) {
                let line = stmt.span().start().line;

                let line_lit = LitInt::new(&line.to_string(), Span::call_site());

                // Assuming line_lit is of a type that needs to be handled directly as a value,
                // use the parsed expression directly in your parse_quote!
                let insert_expr: Expr = parse_quote! {
                   self.env().emit_event(Coverage { cov_of: #line_lit })
                };

                // Convert this expression into a statement
                let pre_stmt: Stmt = Stmt::Expr(insert_expr, Some(Token![;](Span::call_site())));

                new_stmts.push(pre_stmt);

                match stmt.clone() {
                    Stmt::Item(expr) => {
                        new_stmts.push(Stmt::Item(expr.clone()));

                        // Insert the COV_MAP update after the current statement
                        let post_stmt: Stmt = syn::parse_quote! {
                             self.env().emit_event(Coverage { cov_of: #line_lit });
                        };
                        new_stmts.push(post_stmt);
                    }

                    // For each type of statement, handle appropriately, ensuring pre- and post-insertion
                    _ => {
                        // Use recursive visitation to handle nested blocks and other statement types
                        self.visit_stmt_mut(&mut stmt);
                        new_stmts.push(stmt.clone());
                    }
                }
            }

            block.stmts = new_stmts;
        }
    }
}

mod test {
    use frame_support::assert_ok;
    use std::path::PathBuf;
    use std::{fs, fs::File, io::Write, process::Command};

    use crate::fuzzer::coverage::CoverageEngine;
    use quote::quote;
    use syn::__private::ToTokens;
    use syn::parse_file;
    use syn::visit_mut::VisitMut;

    use crate::fuzzer::coverage::instrumentor_visitors::*;

    #[test]
    fn adding_cov_insertion_works() {
        let signature = "COV_MAP . insert (";

        let code = fs::read_to_string("sample/dns/lib.rs").unwrap();
        let mut ast = parse_file(&code).expect("Unable to parse file");

        let mut visitor = ContractCovUpdater;
        visitor.visit_file_mut(&mut ast);

        let modified_code = quote!(#ast).to_string();
        assert!(modified_code.contains(signature)); //spaces are required :shrug:
        export(modified_code);
    }

    #[test]
    fn do_fork() {
        let engine: CoverageEngine = CoverageEngine::new(PathBuf::from("sample/dns/lib.rs"));
        let fork = engine.fork();
        println!("{:?}", fork);
        let exists = fork.exists();
        fs::remove_file(fork).unwrap(); //remove after test passed to avoid spam of /tmp
        assert!(exists);
    }

    #[test]
    fn adding_cov_declaration_works() {
        let signature = "emit_event";

        let content = fs::read_to_string("sample/dns/lib.rs").unwrap();
        let mut syntax_tree: syn::File = syn::parse_str(&content).unwrap();

        let mut visitor = ContractMapInstantiation;
        visitor.visit_file_mut(&mut syntax_tree);

        let modified_content = quote!(#syntax_tree).to_string();
        println!("{:?}", modified_content);
        export(modified_content.clone());
        // assert that we have more than one "emit_event"
        assert!(modified_content.matches(signature).count() > 1);
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
