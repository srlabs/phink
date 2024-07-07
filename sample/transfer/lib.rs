#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod transfer {
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {}
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct Transfer {
        leet_transfered: bool,
    }

    impl Transfer {
        #[ink(constructor)]
        pub fn new() -> Self {
            Transfer {
                leet_transfered: false,
            }
        }
        #[ink(message, payable)]
        pub fn pay_me(&mut self) -> Result<()> {
            let transferred = self.env().transferred_value();
            if transferred > 1337 {
                    self.leet_transfered = true;
                }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn pay_me_test() {
            let mut contract = Transfer::new();
            assert_eq!(contract.pay_me(), Ok(()));
        }
    }


    #[cfg(feature = "phink")]
    #[ink(impl)]
    impl Transfer {
        // This invariant ensures that 1337 cannot be transfer
        #[cfg(feature = "phink")]
        #[ink(message)]
        pub fn phink_assert_cannot_transfer_1337(&self) {
            assert_ne!(self.leet_transfered, true);
        }
    }
}
