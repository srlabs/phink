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
        pub fn toz(&mut self, a: u32, name: Hash) {
            let a = 1 + 1;
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
}
