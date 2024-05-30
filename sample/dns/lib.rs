#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod dns {
    use ink::{prelude::vec::Vec, storage::Mapping, storage::StorageVec};

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

    const FORBIDDEN_DOMAIN: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ]; //we forbid it :/

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
        dangerous_number: u32,
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
                dangerous_number: 0,
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

            // We effectively check that we can't register the forbidden domain
            if name.clone().as_mut() == FORBIDDEN_DOMAIN {
                return Err(Error::ForbiddenDomain);
            }

            self.name_to_owner.insert(name, &caller);
            self.env().emit_event(Register { name, from: caller });
            self.domains.push(&name);

            Ok(())
        }

        /// Set address for specific name.
        #[ink(message)]
        pub fn set_address(&mut self, name: Hash, new_address: AccountId) -> Result<()> {
            let caller = self.env().caller();

            //Random code for coverage purposes below
            let a = 1;
            let b = 3;
            assert_eq!(a, b - 2);

            let owner = self.get_owner_or_default(name);
            if caller != owner {
                return Err(Error::CallerIsNotOwner);
            }

            let old_address = self.name_to_address.get(name);
            self.name_to_address.insert(name, &new_address);

            self.env().emit_event(SetAddress {
                name,
                from: caller,
                old_address,
                new_address,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn crash_with_invariant(&mut self, data: Vec<u8>) -> Result<()> {
            if data.len() == 5 {
                if data[0] == b'a' {
                    if data[1] == b'b' {
                        if data[2] == b'c' {
                            if data[3] == b'd' {
                                self.dangerous_number = 69; //panic!
                            }
                        }
                    }
                }
            }

            Ok(())
        }

        #[ink(message)]
        pub fn crash_with_contract_trapped(&mut self, data: Vec<u8>) -> crate::dns::Result<()> {
            if data.len() < 5 {
                if data[0] == b'a' {
                    // But what if data is empty
                    // --> Contract trapped!
                }
            }
            Ok(())
        }
        /// Transfer owner to another address.
        /// Don't tell anyone, but this contract is vulnerable!
        /// A user can push FORBIDDEN_DOMAIN, as the developer forgot to handle `Error::ForbiddenDomain`
        #[ink(message)]
        pub fn transfer(&mut self, name: Hash, to: AccountId, number: u32) -> Result<()> {
            let caller = self.env().caller();
            // Let's assume we still transfer if the caller isn't the owner

            let owner = self.get_owner_or_default(name);
            if caller != owner {
                return Err(Error::CallerIsNotOwner);
            }

            let old_owner = self.name_to_owner.get(name);
            self.name_to_owner.insert(name, &to);

            self.dangerous_number = number;
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

    fn zero_address() -> AccountId {
        [0u8; 32].into()
    }

    #[cfg(feature = "phink")]
    #[ink(impl)]
    impl DomainNameService {
        // This invariant ensures that `domains` doesn't contain the forbidden domain that nobody should register
        #[cfg(feature = "phink")]
        #[ink(message)]
        pub fn phink_assert_hash42_cant_be_registered(&self) {
            for i in 0..self.domains.len() {
                if let Some(domain) = self.domains.get(i) {
                    assert_ne!(domain.clone().as_mut(), FORBIDDEN_DOMAIN);
                }
            }
        }

        // This invariant ensures that nobody register the forbidden number
        #[cfg(feature = "phink")]
        #[ink(message)]
        pub fn phink_assert_dangerous_number(&self) {
            let forbidden_number = 69;
            assert_ne!(self.dangerous_number, forbidden_number);
        }
    }
}
