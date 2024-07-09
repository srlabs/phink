#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod cross_message_bug {

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
    }
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct CrossMessageBug {
        state: u8,
    }

    impl CrossMessageBug {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { state: 0 }
        }

        #[ink(message)]
        pub fn a(&mut self) -> Result<()> {
            if self.state == 1 {
                self.state = 2;
            } else {
                self.state = 0;
            }
            Ok(())
        }

        #[ink(message)]
        pub fn b(&mut self) -> Result<()>{
            self.state = 1;
            Ok(())
        }

        #[ink(message)]
        pub fn c(&mut self) -> Result<()>{
            if self.state == 2 {
                self.state = 3;
            } else {
                self.state = 0;
            }
            Ok(())
        }

        #[ink(message)]
        pub fn reset(&mut self) -> Result<()>{
            self.state = 0;
            Ok(())
        }

    }

    #[cfg(feature = "phink")]
    #[ink(impl)]
    impl CrossMessageBug {
        /// This will crash if
        ///     1. B() is called
        ///     2. A() is called
        ///     3. C() is called
        /// reset() is never called
        #[ink(message)]
        #[cfg(feature = "phink")]
        pub fn phink_assert_ultimate_crash(&self) {
            assert_ne!(self.state, 3);
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let mut cross_message_bug = CrossMessageBug::new();
            assert_eq!(cross_message_bug.b(), Ok(()));
            assert_eq!(cross_message_bug.state, 1);

            assert_eq!(cross_message_bug.a(), Ok(()));
            assert_eq!(cross_message_bug.state, 2);

            assert_eq!(cross_message_bug.c(), Ok(()));
            assert_eq!(cross_message_bug.state, 3)
        }
    }
}
