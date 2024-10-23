#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod dns {
    use ink::storage::{
        Mapping,
        StorageVec,
    };

    const FORBIDDEN_DOMAIN: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ]; // we forbid it :/

    /// Emitted whenever a new name is being registered.
    #[ink(event)]
    pub struct Register {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
    }

    /// Emitted whenever an address changes.
    #[ink(event)]
    pub struct SetAddress {
        #[ink(topic)]
        name: Hash,
        from: AccountId,
        #[ink(topic)]
        old_address: Option<AccountId>,
        #[ink(topic)]
        new_address: AccountId,
    }

    /// Emitted whenever a name is being transferred.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        name: Hash,
        from: AccountId,
        #[ink(topic)]
        old_owner: Option<AccountId>,
        #[ink(topic)]
        new_owner: AccountId,
    }

    #[ink(storage)]
    pub struct DomainNameService {
        /// A hashmap to store all name to addresses mapping.
        name_to_address: Mapping<Hash, AccountId>,
        /// A hashmap to store all name to owners mapping.
        name_to_owner: Mapping<Hash, AccountId>,
        /// The default address.
        default_address: AccountId,
        /// Simple storage vec that contains every registered domain
        domains: StorageVec<Hash>,
        /// Another invariant testing
        state: i32,

        should_panic_after_three_calls: bool,
    }

    impl Default for DomainNameService {
        fn default() -> Self {
            let mut name_to_address = Mapping::new();
            name_to_address.insert(Hash::default(), &zero_address());
            let mut name_to_owner = Mapping::new();
            name_to_owner.insert(Hash::default(), &zero_address());
            let mut domains = StorageVec::new();
            domains.push(&Hash::default());

            Self {
                name_to_address,
                name_to_owner,
                default_address: zero_address(),
                domains,
                state: 42_i32,
                should_panic_after_three_calls: false,
            }
        }
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Returned if the name already exists upon registration.
        NameAlreadyExists,
        /// Returned if caller is not owner while required to.
        CallerIsNotOwner,
        /// Forbidden domain, we can't register that one... except if ?
        ForbiddenDomain,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl DomainNameService {
        /// Creates a new domain name service contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        /// Register specific name with caller as owner.
        #[ink(message)]
        pub fn register(&mut self, name: Hash) -> Result<()> {
            let caller = self.env().caller();
            if self.name_to_owner.contains(name) {
                return Err(Error::NameAlreadyExists);
            }

            if self.state == 80 {
                if name == FORBIDDEN_DOMAIN.into() {
                    self.state = 120;
                }
            }

            self.name_to_owner.insert(name, &caller);
            self.env().emit_event(Register { name, from: caller });
            self.domains.push(&name);

            Ok(())
        }

        #[ink(message)]
        pub fn set_address(&mut self, name: Hash, new_address: AccountId) -> Result<()> {
            let caller = self.env().caller();
            let owner = self.get_owner_or_default(name);
            if caller != owner {
                return Err(Error::CallerIsNotOwner);
            }

            let old_address = self.name_to_address.get(name);
            self.name_to_address.insert(name, &new_address);

            if self.state == 120 {
                self.should_panic_after_three_calls = true
            }

            self.env().emit_event(SetAddress {
                name,
                from: caller,
                old_address,
                new_address,
            });
            Ok(())
        }
        #[ink(message)]
        pub fn transfer(&mut self, name: Hash, to: AccountId, number: i32) -> Result<()> {
            let caller = self.env().caller();
            // Let's assume we still transfer if the caller isn't the owner

            let owner = self.get_owner_or_default(name);
            if caller != owner {
                return Err(Error::CallerIsNotOwner);
            }

            if number == 42i32 {
                // NOP, 42 is forbidden! right?
                return Err(Error::ForbiddenDomain);
            }

            self.state = number % 70;

            if number == 80 {
                self.state = 80;
            }

            let old_owner = self.name_to_owner.get(name);
            self.name_to_owner.insert(name, &to);

            self.domains.push(&name);

            self.env().emit_event(Transfer {
                name,
                from: caller,
                old_owner,
                new_owner: to,
            });

            Ok(())
        }

        /// Get address for specific name.
        #[ink(message)]
        pub fn get_address(&self, name: Hash) -> AccountId {
            self.get_address_or_default(name)
        }

        /// Get owner of specific name.
        #[ink(message)]
        pub fn get_owner(&self, name: Hash) -> AccountId {
            self.get_owner_or_default(name)
        }

        /// Returns the owner given the hash or the default address.
        fn get_owner_or_default(&self, name: Hash) -> AccountId {
            self.name_to_owner.get(name).unwrap_or(self.default_address)
        }

        /// Returns the address given the hash or the default address.
        fn get_address_or_default(&self, name: Hash) -> AccountId {
            self.name_to_address
                .get(name)
                .unwrap_or(self.default_address)
        }
    }

    /// Helper for referencing the zero address (`0x00`). Note that in practice this
    /// address should not be treated in any special way (such as a default
    /// placeholder) since it has a known private key.
    fn zero_address() -> AccountId {
        [0u8; 32].into()
    }

    #[cfg(feature = "phink")]
    #[ink(impl)]
    impl DomainNameService {
        #[cfg(feature = "phink")]
        #[ink(message)]
        pub fn phink_assert_three_message_calls_required_to_crash(&self) {
            // First, transfer must transfer 80
            // Then, we must have register with hash=FORBIDDEN_DOMAIn
            // Ultimately, we need to call set_address with random value
            assert_eq!(self.should_panic_after_three_calls, false);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_next_caller(caller: AccountId) {
            ink::env::test::set_caller::<Environment>(caller);
        }

        #[ink::test]
        fn test_invariants_nested_calls() {
            let accounts = default_accounts();
            let name = Hash::from([0x99; 32]);

            set_next_caller(accounts.alice);
            let mut contract = DomainNameService::new();

            assert_eq!(
                contract.transfer(Hash::from([1; 32]), accounts.bob, 80),
                Ok(())
            );
            assert_eq!(contract.register(name), Ok(()));
            assert_eq!(contract.should_panic_after_three_calls, true);
        }

        #[ink::test]
        fn set_address_works() {
            let accounts = default_accounts();
            let name = Hash::from([0x99; 32]);

            set_next_caller(accounts.alice);

            let mut contract = DomainNameService::new();
            assert_eq!(contract.register(name), Ok(()));

            // Caller is not owner, `set_address` should fail.
            set_next_caller(accounts.bob);
            assert_eq!(
                contract.set_address(name, accounts.bob),
                Err(Error::CallerIsNotOwner)
            );

            // Caller is owner, set_address will be successful
            set_next_caller(accounts.alice);
            assert_eq!(contract.set_address(name, accounts.bob), Ok(()));
            assert_eq!(contract.get_address(name), accounts.bob);
            assert_eq!(contract.should_panic_after_three_calls, true);
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = default_accounts();
            let name = Hash::from([0x99; 32]);

            set_next_caller(accounts.alice);

            let mut contract = DomainNameService::new();
            assert_eq!(contract.register(name), Ok(()));
            // contract.phink_assert_hash42_cant_be_registered();

            let illegal = Hash::from(FORBIDDEN_DOMAIN);

            // Test transfer of owner.
            assert_eq!(contract.transfer(illegal, accounts.bob, 0), Ok(()));

            // This should panick..
            // contract.phink_assert_hash42_cant_be_registered();

            // Owner is bob, alice `set_address` should fail.
            assert_eq!(
                contract.set_address(name, accounts.bob),
                Err(Error::CallerIsNotOwner)
            );

            set_next_caller(accounts.bob);
            // Now owner is bob, `set_address` should be successful.
            assert_eq!(contract.set_address(name, accounts.bob), Ok(()));
            assert_eq!(contract.get_address(name), accounts.bob);
        }
    }
}
