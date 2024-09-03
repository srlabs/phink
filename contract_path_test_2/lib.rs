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
    pub struct MyBuggedContract {
        forbidden_number: u32,
    }
    impl Default for MyBuggedContract {
        fn default() -> Self {
            ink::env::debug_println!("COV={}", 0);
            Self {
                forbidden_number: 0,
            }
        }
    }
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {}
    pub type Result<T> = core::result::Result<T, Error>;
    impl MyBuggedContract {
        /// Creates a new domain name service contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            ink::env::debug_println!("COV={}", 1);
            Default::default()
        }
        #[ink(message)]
        pub fn crash_with_invariant(&mut self, data: String) -> Result<()> {
            ink::env::debug_println!("COV={}", 2);
            if data.len() == 4 {
                ink::env::debug_println!("COV={}", 3);
                if data.chars().nth(0).unwrap() == 'f' {
                    ink::env::debug_println!("COV={}", 4);
                    if data.chars().nth(1).unwrap() == 'u' {
                        ink::env::debug_println!("COV={}", 5);
                        if data.chars().nth(2).unwrap() == 'z' {
                            ink::env::debug_println!("COV={}", 6);
                            if data.chars().nth(3).unwrap() == 'z' {
                                ink::env::debug_println!("COV={}", 7);
                                self.forbidden_number = 69;
                            }
                        }
                    }
                }
            }
            ink::env::debug_println!("COV={}", 8);
            Ok(())
        }
    }
    #[cfg(feature = "phink")]
    #[ink(impl)]
    impl MyBuggedContract {
        #[cfg(feature = "phink")]
        #[ink(message)]
        pub fn phink_assert_dangerous_number(&self) {
            ink::env::debug_println!("COV={}", 9);
            let forbidden_number = 69;
            ink::env::debug_println!("COV={}", 10);
            ink::env::debug_println!("xx");
            ink::env::debug_println!("COV={}", 11);
            assert_ne!(self.forbidden_number, forbidden_number);
        }
    }
}
