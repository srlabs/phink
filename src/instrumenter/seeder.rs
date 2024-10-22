use quote::quote;
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

#[derive(Debug, Clone)]
pub struct SeedExtractInjector {}

impl SeedExtractInjector {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

impl SeedExtractInjector {
    // Check if the function has the #[ink(message)] attribute
    pub fn has_ink_message_attribute(i: &mut ImplItemFn) -> bool {
        for attr in &i.attrs {
            if attr.path().is_ident("ink") {
                let res = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("message") {
                        Ok(())
                    } else {
                        Err(meta.error("Not an ink! message"))
                    }
                });
                return res.is_ok()
            }
        }
        false
    }
}

impl VisitMut for SeedExtractInjector {
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
                if Self::has_ink_message_attribute(method) {
                    let fn_name = &method.sig.ident;
                    // If the visited function isn't an invariant
                    if !fn_name.to_string().starts_with("phink_") {
                        // Collect the argument identifiers
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
                            let mut toz = ink::env::call::ExecutionInput::new(
                                ink::env::call::Selector::new(ink::selector_bytes!(#fn_name_str))
                            )
                        };

                        for arg in args {
                            push_args = quote! {
                                #push_args
                                .push_arg(#arg)
                            };
                        }

                        let snippet: Stmt = parse_quote! {
                            {
                            #push_args;
                            let encoded = ink::scale::Encode::encode(&toz);
                            ink::env::debug_println!("ENCODED_SEED = 0x{}", encoded.iter().map(|byte| format!("{:02x}", byte)).collect::<String>());
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

    // Ensure we visit items inside modules
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

    use crate::instrumenter::seeder::SeedExtractInjector;
    use quote::quote;
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
                                                self.forbidden_number = 69;
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
                            let forbidden_number = 69;
                            assert_ne!(self.forbidden_number, forbidden_number);
                        }
                    }
                }"#;

        let mut syntax_tree: File = parse_str(input_code).expect("Failed to parse code");
        let mut seed_injector = SeedExtractInjector::new().unwrap();
        seed_injector.visit_file_mut(&mut syntax_tree);

        let generated_code = quote!(#syntax_tree).to_string();

        println!("{}", generated_code);

        let expected_snippet = r#"let mut toz = ink :: env :: call :: ExecutionInput :: new (ink :: env :: call :: Selector :: new (ink :: selector_bytes ! ("toz"))) . push_arg (a) . push_arg (name) ;"#;
        assert!(
            generated_code.contains(&expected_snippet.to_string()),
            "The code snippet was not injected correctly."
        );
    }
}
