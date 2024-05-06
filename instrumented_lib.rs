#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod dns {
    #[ink(event, anonymous)]
    pub struct Coverage {
        cov_of: i32,
    }
    use ink::storage::Mapping;
    use ink::storage::StorageVec;
    #[ink(event, anonymous)]
    pub struct Coverage {
        cov_of: i32,
    }
    #[doc = " Emitted whenever a new name is being registered."]
    #[ink(event)]
    pub struct Register {
        #[ink(topic)]
        name: Hash,
        #[ink(topic)]
        from: AccountId,
    }
    #[doc = " Emitted whenever an address changes."]
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
    #[doc = " Emitted whenever a name is being transferred."]
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
    ];
    #[ink(storage)]
    pub struct DomainNameService {
        #[doc = " A hashmap to store all name to addresses mapping."]
        name_to_address: Mapping<Hash, AccountId>,
        #[doc = " A hashmap to store all name to owners mapping."]
        name_to_owner: Mapping<Hash, AccountId>,
        #[doc = " The default address."]
        default_address: AccountId,
        #[doc = " Simple storage vec that contains every registered domain"]
        domains: StorageVec<Hash>,
        #[doc = " Another invariant testing"]
        dangerous_number: i32,
    }
    impl Default for DomainNameService {
        fn default() -> Self {
            Self::env().emit_event(Coverage { cov_of: 66 });
            let mut name_to_address = Mapping::new();
            Self::env().emit_event(Coverage { cov_of: 67 });
            name_to_address.insert(Hash::default(), &zero_address());
            Self::env().emit_event(Coverage { cov_of: 68 });
            let mut name_to_owner = Mapping::new();
            Self::env().emit_event(Coverage { cov_of: 69 });
            name_to_owner.insert(Hash::default(), &zero_address());
            Self::env().emit_event(Coverage { cov_of: 70 });
            let mut domains = StorageVec::new();
            Self::env().emit_event(Coverage { cov_of: 71 });
            domains.push(&Hash::default());
            Self::env().emit_event(Coverage { cov_of: 73 });
            Self {
                name_to_address,
                name_to_owner,
                default_address: zero_address(),
                domains,
                dangerous_number: 42_i32,
            }
        }
    }
    #[doc = " Errors that can occur upon calling this contract."]
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        #[doc = " Returned if the name already exists upon registration."]
        NameAlreadyExists,
        #[doc = " Returned if caller is not owner while required to."]
        CallerIsNotOwner,
        #[doc = " Forbidden domain, we can't register that one... except if ?"]
        ForbiddenDomain,
    }
    #[doc = " Type alias for the contract's result type."]
    pub type Result<T> = core::result::Result<T, Error>;
    impl DomainNameService {
        #[doc = " Creates a new domain name service contract."]
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::env().emit_event(Coverage { cov_of: 102 });
            Default::default()
        }
        #[doc = " Register specific name with caller as owner."]
        #[ink(message)]
        pub fn register(&mut self, name: Hash) -> Result<()> {
            Self::env().emit_event(Coverage { cov_of: 108 });
            let caller = self.env().caller();
            Self::env().emit_event(Coverage { cov_of: 109 });
            if self.name_to_owner.contains(name) {
                Self::env().emit_event(Coverage { cov_of: 110 });
                return Err(Error::NameAlreadyExists);
            }
            Self::env().emit_event(Coverage { cov_of: 114 });
            if name.clone().as_mut() == FORBIDDEN_DOMAIN {
                Self::env().emit_event(Coverage { cov_of: 115 });
                return Err(Error::ForbiddenDomain);
            }
            Self::env().emit_event(Coverage { cov_of: 118 });
            self.name_to_owner.insert(name, &caller);
            Self::env().emit_event(Coverage { cov_of: 119 });
            self.env().emit_event(Register { name, from: caller });
            Self::env().emit_event(Coverage { cov_of: 120 });
            self.domains.push(&name);
            Self::env().emit_event(Coverage { cov_of: 122 });
            Ok(())
        }
        #[doc = " Set address for specific name."]
        #[ink(message)]
        pub fn set_address(&mut self, name: Hash, new_address: AccountId) -> Result<()> {
            Self::env().emit_event(Coverage { cov_of: 128 });
            let caller = self.env().caller();
            Self::env().emit_event(Coverage { cov_of: 129 });
            let owner = self.get_owner_or_default(name);
            Self::env().emit_event(Coverage { cov_of: 130 });
            if caller != owner {
                Self::env().emit_event(Coverage { cov_of: 131 });
                return Err(Error::CallerIsNotOwner);
            }
            Self::env().emit_event(Coverage { cov_of: 134 });
            let old_address = self.name_to_address.get(name);
            Self::env().emit_event(Coverage { cov_of: 135 });
            self.name_to_address.insert(name, &new_address);
            Self::env().emit_event(Coverage { cov_of: 137 });
            self.env().emit_event(SetAddress {
                name,
                from: caller,
                old_address,
                new_address,
            });
            Self::env().emit_event(Coverage { cov_of: 143 });
            Ok(())
        }
        #[doc = " Transfer owner to another address."]
        #[doc = " Don't tell anyone, but this contract is vulnerable!"]
        #[doc = " A user can push FORBIDDEN_DOMAIN, as the developer forgot to handle `Error::ForbiddenDomain`"]
        #[ink(message)]
        pub fn transfer(&mut self, name: Hash, to: AccountId, number: i32) -> Result<()> {
            Self::env().emit_event(Coverage { cov_of: 151 });
            let caller = self.env().caller();
            Self::env().emit_event(Coverage { cov_of: 159 });
            let old_owner = self.name_to_owner.get(name);
            Self::env().emit_event(Coverage { cov_of: 160 });
            self.name_to_owner.insert(name, &to);
            Self::env().emit_event(Coverage { cov_of: 162 });
            self.dangerous_number = number;
            Self::env().emit_event(Coverage { cov_of: 163 });
            self.domains.push(&name);
            Self::env().emit_event(Coverage { cov_of: 165 });
            self.env().emit_event(Transfer {
                name,
                from: caller,
                old_owner,
                new_owner: to,
            });
            Self::env().emit_event(Coverage { cov_of: 172 });
            Ok(())
        }
        #[doc = " Get address for specific name."]
        #[ink(message)]
        pub fn get_address(&self, name: Hash) -> AccountId {
            Self::env().emit_event(Coverage { cov_of: 178 });
            self.get_address_or_default(name)
        }
        #[doc = " Get owner of specific name."]
        #[ink(message)]
        pub fn get_owner(&self, name: Hash) -> AccountId {
            Self::env().emit_event(Coverage { cov_of: 184 });
            Self::env().emit_event(Coverage { cov_of: 1 });
            Self::env().emit_event(Coverage { cov_of: 185 });
            self.get_owner_or_default(name)
        }
        #[doc = " Returns the owner given the hash or the default address."]
        fn get_owner_or_default(&self, name: Hash) -> AccountId {
            Self::env().emit_event(Coverage { cov_of: 190 });
            self.name_to_owner.get(name).unwrap_or(self.default_address)
        }
        #[doc = " Returns the address given the hash or the default address."]
        fn get_address_or_default(&self, name: Hash) -> AccountId {
            Self::env().emit_event(Coverage { cov_of: 195 });
            self.name_to_address
                .get(name)
                .unwrap_or(self.default_address)
        }
    }
    #[doc = " Helper for referencing the zero address (`0x00`). Note that in practice this"]
    #[doc = " address should not be treated in any special way (such as a default"]
    #[doc = " placeholder) since it has a known private key."]
    fn zero_address() -> AccountId {
        Self::env().emit_event(Coverage { cov_of: 205 });
        [0u8; 32].into()
    }
    #[cfg(test)]
    mod tests {
        #[ink(event, anonymous)]
        pub struct Coverage {
            cov_of: i32,
        }
        use super::*;
        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            Self::env().emit_event(Coverage { cov_of: 213 });
            ink::env::test::default_accounts::<Environment>()
        }
        fn set_next_caller(caller: AccountId) {
            Self::env().emit_event(Coverage { cov_of: 217 });
            ink::env::test::set_caller::<Environment>(caller);
        }
        #[ink::test]
        fn register_works() {
            Self::env().emit_event(Coverage { cov_of: 222 });
            let default_accounts = default_accounts();
            Self::env().emit_event(Coverage { cov_of: 223 });
            let name = Hash::from([0x99; 32]);
            Self::env().emit_event(Coverage { cov_of: 225 });
            set_next_caller(default_accounts.alice);
            Self::env().emit_event(Coverage { cov_of: 226 });
            let mut contract = DomainNameService::new();
            Self::env().emit_event(Coverage { cov_of: 228 });
            assert_eq!(contract.register(name), Ok(()));
            Self::env().emit_event(Coverage { cov_of: 229 });
            assert_eq!(contract.register(name), Err(Error::NameAlreadyExists));
        }
        #[ink::test]
        fn set_address_works() {
            Self::env().emit_event(Coverage { cov_of: 234 });
            let accounts = default_accounts();
            Self::env().emit_event(Coverage { cov_of: 235 });
            let name = Hash::from([0x99; 32]);
            Self::env().emit_event(Coverage { cov_of: 237 });
            set_next_caller(accounts.alice);
            Self::env().emit_event(Coverage { cov_of: 239 });
            let mut contract = DomainNameService::new();
            Self::env().emit_event(Coverage { cov_of: 240 });
            assert_eq!(contract.register(name), Ok(()));
            Self::env().emit_event(Coverage { cov_of: 243 });
            set_next_caller(accounts.bob);
            Self::env().emit_event(Coverage { cov_of: 244 });
            assert_eq!(
                contract.set_address(name, accounts.bob),
                Err(Error::CallerIsNotOwner)
            );
            Self::env().emit_event(Coverage { cov_of: 250 });
            set_next_caller(accounts.alice);
            Self::env().emit_event(Coverage { cov_of: 251 });
            assert_eq!(contract.set_address(name, accounts.bob), Ok(()));
            Self::env().emit_event(Coverage { cov_of: 252 });
            assert_eq!(contract.get_address(name), accounts.bob);
            Self::env().emit_event(Coverage { cov_of: 253 });
            contract.phink_assert_hash42_cant_be_registered();
        }
        #[ink::test]
        fn should_panic() {
            Self::env().emit_event(Coverage { cov_of: 258 });
            let accounts = default_accounts();
            Self::env().emit_event(Coverage { cov_of: 259 });
            set_next_caller(accounts.alice);
            Self::env().emit_event(Coverage { cov_of: 260 });
            let mut contract = DomainNameService::new();
            Self::env().emit_event(Coverage { cov_of: 261 });
            let illegal = Hash::from(FORBIDDEN_DOMAIN);
            Self::env().emit_event(Coverage { cov_of: 262 });
            println!("{:?}", illegal);
            Self::env().emit_event(Coverage { cov_of: 263 });
            assert_eq!(contract.transfer(illegal, accounts.bob), Ok(()));
            Self::env().emit_event(Coverage { cov_of: 264 });
            contract.phink_assert_hash42_cant_be_registered();
        }
        #[ink::test]
        fn transfer_works() {
            Self::env().emit_event(Coverage { cov_of: 269 });
            let accounts = default_accounts();
            Self::env().emit_event(Coverage { cov_of: 270 });
            let name = Hash::from([0x99; 32]);
            Self::env().emit_event(Coverage { cov_of: 272 });
            set_next_caller(accounts.alice);
            Self::env().emit_event(Coverage { cov_of: 274 });
            let mut contract = DomainNameService::new();
            Self::env().emit_event(Coverage { cov_of: 275 });
            assert_eq!(contract.register(name), Ok(()));
            Self::env().emit_event(Coverage { cov_of: 276 });
            contract.phink_assert_hash42_cant_be_registered();
            Self::env().emit_event(Coverage { cov_of: 278 });
            let illegal = Hash::from(FORBIDDEN_DOMAIN);
            Self::env().emit_event(Coverage { cov_of: 281 });
            assert_eq!(contract.transfer(illegal, accounts.bob), Ok(()));
            Self::env().emit_event(Coverage { cov_of: 284 });
            contract.phink_assert_hash42_cant_be_registered();
            Self::env().emit_event(Coverage { cov_of: 287 });
            assert_eq!(
                contract.set_address(name, accounts.bob),
                Err(Error::CallerIsNotOwner)
            );
            Self::env().emit_event(Coverage { cov_of: 292 });
            set_next_caller(accounts.bob);
            Self::env().emit_event(Coverage { cov_of: 294 });
            assert_eq!(contract.set_address(name, accounts.bob), Ok(()));
            Self::env().emit_event(Coverage { cov_of: 295 });
            assert_eq!(contract.get_address(name), accounts.bob);
        }
    }
}
