pub mod custom;

use sp_core::storage::Storage;
pub struct Preferences {}

pub trait DevelopperPreferences {
    /// This function allows developers to add their own storage configurations 🛠️.
    /// It is used to mock the state and provide sufficient data for the fuzzer 🐛.
    /// You should definitely adapt this function to your needs 🔧.
    fn runtime_storage() -> Storage;

    /// Developpers can `impl` this function in order to execute any code during the main
    /// contract initialization. This can be for example, uploading other contracts or
    /// other dependencies. Often, you might want this function to be empty
    fn on_contract_initialize();
}
