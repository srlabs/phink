#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod fuzzme {
    use ink::{prelude::vec::Vec, storage::Mapping, storage::StorageVec};
    use ink_prelude::string::String;

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        Oui,
    }

    #[ink(storage)]
    pub struct DomainNameService {
        dangerous_number: u32,
    }

    impl Default for DomainNameService {
        fn default() -> Self {
            Self {
                dangerous_number: 0,
            }
        }
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl DomainNameService {
        /// Creates a new domain name service contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        #[ink(message)]
        pub fn crash_with_invariant(&mut self, data: String) -> Result<()> {
            if data.len() == 4{

                    if data.chars().nth(0).unwrap() == 'f' {
                        if data.chars().nth(1).unwrap() == 'u' {
                            if data.chars().nth(2).unwrap() == 'z' {
                                if data.chars().nth(3).unwrap() == 'z' {
                                    self.dangerous_number = 69;
                                }
                            }
                        }
                    }

            }

            Ok(())
        }
    }

    #[cfg(feature = "phink")]
    #[ink(impl)]
    impl DomainNameService {
        // This invariant ensures that nobody register the forbidden number
        #[cfg(feature = "phink")]
        #[ink(message)]
        pub fn phink_assert_dangerous_number(&self) {
            let forbidden_number = 69;
            assert_ne!(self.dangerous_number, forbidden_number);
        }
    }
}
