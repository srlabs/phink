#[ink::contract]
mod logger {
    #[ink(storage)]
    pub struct Logger {}

    #[ink(event)]
    pub struct LogEvent {
        #[ink(topic)]
        message: String,
    }

    impl Logger {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn log_event(&self, message: String) {
            self.env().emit_event(LogEvent { message });
        }
    }
}
