#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod overflow {
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {}
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct Overflow {
        random_number: u16,
    }

    impl Overflow {
        #[ink(constructor)]
        pub fn new() -> Self {
            Overflow {
                random_number: 1,
            }
        }
        #[allow(arithmetic_overflow)]
        #[ink(message, payable)]
        pub fn overflow(&mut self, overflower: u16) -> Result<()> {
            //todo: make me bug
            let abc: u16 = u16::MAX - 1 + overflower; //21845 is `u16::MAX/3`
            self.random_number = abc;
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
        fn overflow_succeed() {
            // let mut contract = Overflow::new();
            // assert_eq!(contract.pay_me(44), Ok(()));
        }
    }


    #[cfg(feature = "phink")]
    #[ink(impl)]
    impl Overflow {
        #[cfg(feature = "phink")]
        #[allow(clippy::absurd_extreme_comparisons)]
        #[ink(message)]
        pub fn phink_assert_cannot_overflow(&self) {
            assert!(self.random_number <= u16::MAX);
        }
    }
}
