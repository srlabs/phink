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
            ink::env::debug_println!("COV={}", 0);
            Default::default()
        }
        #[ink(message)]
        pub fn crash_with_invariant(&mut self, data: String) -> Result<()> {
            ink::env::debug_println!("COV={}", 1);
            if data.len() < 7 && data.len() > 3 {
                ink::env::debug_println!("COV={}", 2);
                if data.chars().nth(0).unwrap() == 'f' {
                    ink::env::debug_println!("COV={}", 3);
                    if data.chars().nth(1).unwrap() == 'u' {
                        ink::env::debug_println!("COV={}", 4);
                        if data.chars().nth(2).unwrap() == 'z' {
                            ink::env::debug_println!("COV={}", 5);
                            if data.chars().nth(3).unwrap() == 'z' {
                                ink::env::debug_println!("COV={}", 6);
                                self.forbidden_number = 42;
                            }
                        }
                    }
                }
            }
            ink::env::debug_println!("COV={}", 7);
            Ok(())
        }
        #[ink(message)]
        pub fn toz(&mut self, a: u32, name: Hash) {
            ink::env::debug_println!("COV={}", 8);
            let a = 1 + 1;
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
        fn new_works() {
            ink::env::debug_println!("COV={}", 9);
            let mut a = MyBuggedContract::new();
            ink::env::debug_println!("COV={}", 10);
            a.crash_with_invariant("abc".to_string()).unwrap();
            ink::env::debug_println!("COV={}", 11);
            a.crash_with_invariant("fuz".to_string()).unwrap();
        }
        #[ink::test]
        fn for_seedgen() {
            ink::env::debug_println!("COV={}", 12);
            let mut a = MyBuggedContract::new();
            ink::env::debug_println!("COV={}", 13);
            a.toz(32, crate::dummy::Hash::from([0x99; 32]));
        }
    }
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
        #[ink_e2e::test]
        async fn it_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            ink::env::debug_println!("COV={}", 14);
            let mut constructor = MyBuggedContractRef::new();
            ink::env::debug_println!("COV={}", 15);
            let contract = client
                .instantiate("dummy", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            ink::env::debug_println!("COV={}", 16);
            let mut call_builder = contract.call_builder::<MyBuggedContract>();
            ink::env::debug_println!("COV={}", 17);
            let flip = call_builder.toz(432432, crate::dummy::Hash::from([0x12; 32]));
            ink::env::debug_println!("COV={}", 18);
            let _flip_res = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");
            ink::env::debug_println!("COV={}", 19);
            Ok(())
        }
    }
    #[cfg(feature = "phink")]
    #[ink(impl)]
    impl MyBuggedContract {
        #[cfg(feature = "phink")]
        #[ink(message)]
        pub fn phink_assert_dangerous_number(&self) {
            ink::env::debug_println!("COV={}", 20);
            let forbidden_number = 42;
            ink::env::debug_println!("COV={}", 21);
            assert_ne!(self.forbidden_number, forbidden_number);
        }
    }
}
