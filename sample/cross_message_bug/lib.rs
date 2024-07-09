#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod cross_message_bug {

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
        pub fn a(&mut self) {
            if self.state == 1 {
                self.state = 2;
            } else {
                self.state = 0;
            }
        }

        #[ink(message)]
        pub fn b(&mut self) {
            self.state = 1;
        }

        #[ink(message)]
        pub fn c(&mut self) {
            if self.state == 2 {
                self.state = 3;
            }
            self.state = 0;
        }

        #[ink(message)]
        pub fn reset(&mut self) {
            self.state = 0;
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
        pub fn phink_assert_ultimate_crash(&self) {
            assert_ne!(self.state, 3);
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let cross_message_bug = CrossMessageBug::default();
            // assert_eq!(cross_message_bug.get(), false);
        }
    }
}
