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
            a.crash_with_invariant("abc".to_string()).unwrap();
            a.crash_with_invariant("fuz".to_string()).unwrap();
        }

        #[ink::test]
        fn for_seedgen() {
            let mut a = MyBuggedContract::new();
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
            let mut constructor = MyBuggedContractRef::new();
            let contract = client
                .instantiate("dummy", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<MyBuggedContract>();

            let flip = call_builder.toz(432432, crate::dummy::Hash::from([0x12; 32]));
            let _flip_res = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");
            Ok(())
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
}
