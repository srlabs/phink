use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemMod;
use syn::ItemStatic;
use syn::visit_mut::{self, VisitMut};

/// The objective of this `struct` is to assist Phink in instrumenting ink! smart contracts.
/// In a fuzzing context, instrumenting a smart contract involves modifying the target (i.e., the WASM blob),
/// for example, by adding additional code to branches to obtain a coverage map during the execution of the smart contract.
/// By doing so, we can effectively generate a coverage map that will be provided to Ziggy
/// or LibAFL, transforming Phink from a basic brute-forcing tool into a powerful coverage-guided fuzzer.
///
/// Phink opted for a Rust AST approach. For each code instruction on the smart-contract, Phink will
/// automatically add a tracing code, which will then be fetched at the end of the input execution
/// in order to get coverage.

pub struct CoverageEngine {}

impl CoverageEngine {}

struct ContractCovUpdater;

struct ContractMapInstantiation;

impl VisitMut for ContractMapInstantiation {
    fn visit_item_mod_mut(&mut self, i: &mut ItemMod) {
        //TODO! Add the COV_MAP for mod that only contains #[ink::contract]
        let static_decl: TokenStream = syn::parse2(quote! {
            static mut COV_MAP: Mapping<i32, i32> = Mapping::new();
        })
        .expect("Failed to parse static declaration");

        let static_item: ItemStatic =
            syn::parse2(static_decl).expect("Failed to parse item static");

        i.content
            .as_mut()
            .map(|(_, items)| items.insert(0, syn::Item::Static(static_item)));

        // Continue traversing the module
        syn::visit_mut::visit_item_mod_mut(self, i);
    }
}
impl VisitMut for ContractCovUpdater {
    fn visit_item_fn_mut(&mut self, i: &mut syn::ItemFn) {
        let mut stmts = Vec::new();

        // Prepare the statement to be inserted
        let new_stmt: syn::Stmt = syn::parse_quote! {
            COV_MAP.insert(1, &4);
        };
        stmts.push(new_stmt); // Insert the new statement at the beginning

        for stmt in &i.block.stmts {
            stmts.push(stmt.clone());
        }
        i.block.stmts = stmts;

        // Continue traversing the AST
        visit_mut::visit_item_fn_mut(self, i);
    }
}

mod test {
    use std::{
        fs::File,
        fs,
        io::Write,
        process::Command
    };

    use quote::quote;
    use syn::__private::ToTokens;
    use syn::parse_file;
    use syn::visit_mut::VisitMut;

    use crate::fuzzer::coverage::{ContractCovUpdater, ContractMapInstantiation};

    #[test]
    fn adding_cov_insertion_works() {
        let code = fs::read_to_string("/Users/kevinvalerio/Desktop/phink/sample/dns/lib.rs").unwrap();
        let mut ast = parse_file(&code).expect("Unable to parse file");

        let mut visitor = ContractCovUpdater;
        visitor.visit_file_mut(&mut ast);

        let modified_code = quote!(#ast).to_string();
        assert!(modified_code.contains("COV_MAP.insert("));
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
