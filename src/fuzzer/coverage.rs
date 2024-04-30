use crate::contract::payload::Selector;
use crate::contract::remote::ContractBridge;
use paste::expr;
use proc_macro2::TokenStream;
use quote::quote;
use std::fs;
use std::path::PathBuf;
use syn::visit_mut::{self, VisitMut};
use syn::{parse_file, Item, ItemStatic};
use syn::{ItemFn, ItemMod};
use visit_mut::visit_item_mod_mut;

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
    pub fn instrument(&self) -> Result<(), ()> {
        let code = fs::read_to_string(&self.lib_path).unwrap();
        let mut ast = parse_file(&code).unwrap();

        let mut visitor = ContractCovUpdater;
        visitor.visit_file_mut(&mut ast);
        let modified_code = quote!(#ast).to_string();
        Ok(())
    }
}

struct ContractCovUpdater;

struct ContractCovMessage;

struct ContractMapInstantiation;

impl VisitMut for ContractCovMessage {
    fn visit_item_mod_mut(&mut self, i: &mut ItemMod) {
        let get_func: TokenStream = syn::parse2(quote! {
            //TODO make the cov_map_return better. Maybe encode ?
                #[ink(message)]
                pub fn phink_get_cov(&mut self) -> i32 {
                    unsafe { COV_MAP.get(1).unwrap() }
                }
        })
            .unwrap();

        let parsed_fn: ItemFn = syn::parse2(get_func).unwrap();

        i.content.as_mut().map(|(_, items)| {
            items.insert(
                0,
                Item::Fn(ItemFn {
                    attrs: parsed_fn.attrs,
                    vis: parsed_fn.vis,
                    sig: parsed_fn.sig,
                    block: parsed_fn.block,
                }),
            )
        });

        // Continue traversing the module
        visit_item_mod_mut(self, i);
    }
}

impl VisitMut for ContractMapInstantiation {
    fn visit_item_mod_mut(&mut self, i: &mut ItemMod) {
        //TODO! Add the COV_MAP for mod that only contains #[ink::contract]
        let cov_static: TokenStream = syn::parse2(quote! {
            static mut COV_MAP: Mapping<i32, i32> = Mapping::new();
        })
            .unwrap();

        let static_item: ItemStatic = syn::parse2(cov_static).unwrap();

        i.content
            .as_mut()
            .map(|(_, items)| items.insert(0, Item::Static(static_item)));

        // Continue traversing the module
        visit_item_mod_mut(self, i);
    }
}

impl VisitMut for ContractCovUpdater {
    fn visit_block_mut(&mut self, block: &mut syn::Block) {
        let mut new_stmts = Vec::new();
        // Temporarily replace block.stmts with an empty Vec to avoid borrowing issues
        let mut stmts = std::mem::replace(&mut block.stmts, Vec::new());

        for mut stmt in stmts.drain(..) {
            // Insert the COV_MAP update statement before the current statement
            let pre_stmt: syn::Stmt = syn::parse_quote! {
               unsafe { COV_MAP.insert(1, &4);}
            };
            new_stmts.push(pre_stmt);

            match stmt.clone() {
                syn::Stmt::Item(expr) => {
                    new_stmts.push(syn::Stmt::Item(expr.clone()));

                    // Insert the COV_MAP update after the current statement
                    let post_stmt: syn::Stmt = syn::parse_quote! {
                        COV_MAP.insert(1, &4);
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

mod test {
    use std::{fs, fs::File, io::Write, process::Command};

    use quote::quote;
    use syn::__private::ToTokens;
    use syn::parse_file;
    use syn::visit_mut::VisitMut;

    use crate::fuzzer::coverage::{
        ContractCovMessage, ContractCovUpdater, ContractMapInstantiation,
    };

    #[test]
    fn adding_cov_insertion_works() {
        let code = fs::read_to_string("sample/dns/lib.rs").unwrap();
        let mut ast = parse_file(&code).expect("Unable to parse file");

        let mut visitor = ContractCovUpdater;
        visitor.visit_file_mut(&mut ast);

        let modified_code = quote!(#ast).to_string();
        assert!(modified_code.contains("COV_MAP . insert (")); //spaces are required :shrug:
        export(modified_code);
    }

    #[test]
    fn adding_cov_declaration_works() {
        let content = fs::read_to_string("sample/dns/lib.rs").unwrap();
        let mut syntax_tree: syn::File = syn::parse_str(&content).unwrap();

        let mut visitor = ContractMapInstantiation;
        visitor.visit_file_mut(&mut syntax_tree);

        let modified_content = quote!(#syntax_tree).to_string();
        println!("{:?}", modified_content);
        export(modified_content.clone());
        assert!(modified_content.contains("static mut COV_MAP"));
    }

    #[test]
    fn adding_cov_get_message_works() {
        let content = fs::read_to_string("sample/dns/lib.rs").unwrap();
        let mut syntax_tree: syn::File = syn::parse_str(&content).unwrap();

        let mut visitor = ContractCovMessage;
        visitor.visit_file_mut(&mut syntax_tree);

        let modified_content = quote!(#syntax_tree).to_string();
        println!("{:?}", modified_content);
        export(modified_content.clone());
        assert!(modified_content.contains("pub fn get_cov"));
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
