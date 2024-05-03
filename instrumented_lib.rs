#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod dns {
    #[ink(event, anonymous)]
    pub struct Coverage {
        cov_of: i32,
    }
    use ink::storage::Mapping;
    use ink::storage::StorageVec;
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
            self.env().emit_event(Coverage { cov_of: 61 });
            let mut name_to_address = Mapping::new();
            self.env().emit_event(Coverage { cov_of: 62 });
            name_to_address.insert(Hash::default(), &zero_address());
            self.env().emit_event(Coverage { cov_of: 63 });
            let mut name_to_owner = Mapping::new();
            self.env().emit_event(Coverage { cov_of: 64 });
            name_to_owner.insert(Hash::default(), &zero_address());
            self.env().emit_event(Coverage { cov_of: 65 });
            let mut domains = StorageVec::new();
            self.env().emit_event(Coverage { cov_of: 66 });
            domains.push(&Hash::default());
            self.env().emit_event(Coverage { cov_of: 68 });
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
            self.env().emit_event(Coverage { cov_of: 97 });
            Default::default()
        }
        #[doc = " Register specific name with caller as owner."]
        #[ink(message)]
        pub fn register(&mut self, name: Hash) -> Result<()> {
            self.env().emit_event(Coverage { cov_of: 103 });
            let caller = self.env().caller();
            self.env().emit_event(Coverage { cov_of: 104 });
            if self.name_to_owner.contains(name) {
                self.env().emit_event(Coverage { cov_of: 105 });
                return Err(Error::NameAlreadyExists);
            }
            self.env().emit_event(Coverage { cov_of: 109 });
            if name.clone().as_mut() == FORBIDDEN_DOMAIN {
                self.env().emit_event(Coverage { cov_of: 110 });
                return Err(Error::ForbiddenDomain);
            }
            self.env().emit_event(Coverage { cov_of: 113 });
            self.name_to_owner.insert(name, &caller);
            self.env().emit_event(Coverage { cov_of: 114 });
            self.env().emit_event(Register { name, from: caller });
            self.env().emit_event(Coverage { cov_of: 115 });
            self.domains.push(&name);
            self.env().emit_event(Coverage { cov_of: 117 });
            Ok(())
        }
        #[doc = " Set address for specific name."]
        #[ink(message)]
        pub fn set_address(&mut self, name: Hash, new_address: AccountId) -> Result<()> {
            self.env().emit_event(Coverage { cov_of: 123 });
            let caller = self.env().caller();
            self.env().emit_event(Coverage { cov_of: 124 });
            let owner = self.get_owner_or_default(name);
            self.env().emit_event(Coverage { cov_of: 125 });
            if caller != owner {
                self.env().emit_event(Coverage { cov_of: 126 });
                return Err(Error::CallerIsNotOwner);
            }
            self.env().emit_event(Coverage { cov_of: 129 });
            let old_address = self.name_to_address.get(name);
            self.env().emit_event(Coverage { cov_of: 130 });
            self.name_to_address.insert(name, &new_address);
            self.env().emit_event(Coverage { cov_of: 132 });
            self.env().emit_event(SetAddress {
                name,
                from: caller,
                old_address,
                new_address,
            });
            self.env().emit_event(Coverage { cov_of: 138 });
            Ok(())
        }
        #[doc = " Transfer owner to another address."]
        #[doc = " Don't tell anyone, but this contract is vulnerable!"]
        #[doc = " A user can push FORBIDDEN_DOMAIN, as the developer forgot to handle `Error::ForbiddenDomain`"]
        #[ink(message)]
        pub fn transfer(&mut self, name: Hash, to: AccountId, number: i32) -> Result<()> {
            self.env().emit_event(Coverage { cov_of: 146 });
            let caller = self.env().caller();
            self.env().emit_event(Coverage { cov_of: 154 });
            let old_owner = self.name_to_owner.get(name);
            self.env().emit_event(Coverage { cov_of: 155 });
            self.name_to_owner.insert(name, &to);
            self.env().emit_event(Coverage { cov_of: 157 });
            self.dangerous_number = number;
            self.env().emit_event(Coverage { cov_of: 158 });
            self.domains.push(&name);
            self.env().emit_event(Coverage { cov_of: 160 });
            self.env().emit_event(Transfer {
                name,
                from: caller,
                old_owner,
                new_owner: to,
            });
            self.env().emit_event(Coverage { cov_of: 167 });
            Ok(())
        }
        #[doc = " Get address for specific name."]
        #[ink(message)]
        pub fn get_address(&self, name: Hash) -> AccountId {
            self.env().emit_event(Coverage { cov_of: 173 });
            self.get_address_or_default(name)
        }
        #[doc = " Get owner of specific name."]
        #[ink(message)]
        pub fn get_owner(&self, name: Hash) -> AccountId {
            self.env().emit_event(Coverage { cov_of: 179 });
            self.get_owner_or_default(name)
        }
        #[doc = " Returns the owner given the hash or the default address."]
        fn get_owner_or_default(&self, name: Hash) -> AccountId {
            self.env().emit_event(Coverage { cov_of: 184 });
            self.name_to_owner.get(name).unwrap_or(self.default_address)
        }
        #[doc = " Returns the address given the hash or the default address."]
        fn get_address_or_default(&self, name: Hash) -> AccountId {
            self.env().emit_event(Coverage { cov_of: 189 });
            self.name_to_address
                .get(name)
                .unwrap_or(self.default_address)
        }
    }
    #[doc = " Helper for referencing the zero address (`0x00`). Note that in practice this"]
    #[doc = " address should not be treated in any special way (such as a default"]
    #[doc = " placeholder) since it has a known private key."]
    fn zero_address() -> AccountId {
        self.env().emit_event(Coverage { cov_of: 199 });
        [0u8; 32].into()
    }
    #[cfg(feature = "phink")]
    #[ink(impl)]
    impl DomainNameService {
        #[doc = " This invariant should be triggered at some point... the contract being vulnerable"]
        #[ink(message)]
        pub fn phink_assert_hash42_cant_be_registered(&self) {
            self.env().emit_event(Coverage { cov_of: 208 });
            for i in 0..self.domains.len() {
                self.env().emit_event(Coverage { cov_of: 209 });
                if let Some(domain) = self.domains.get(i) {
                    self.env().emit_event(Coverage { cov_of: 212 });
                    assert_ne!(domain.clone().as_mut(), FORBIDDEN_DOMAIN);
                }
            }
        }
        #[ink(message)]
        pub fn phink_assert_dangerous_number(&self) {
            self.env().emit_event(Coverage { cov_of: 219 });
            let FORBIDDEN_NUMBER = 69;
            self.env().emit_event(Coverage { cov_of: 220 });
            assert_ne!(self.dangerous_number, FORBIDDEN_NUMBER);
        }
    }
    #[cfg(test)]
    mod tests {
        #[ink(event, anonymous)]
        pub struct Coverage {
            cov_of: i32,
        }
        use super::*;
        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            self.env().emit_event(Coverage { cov_of: 230 });
            ink::env::test::default_accounts::<Environment>()
        }
        fn set_next_caller(caller: AccountId) {
            self.env().emit_event(Coverage { cov_of: 234 });
            ink::env::test::set_caller::<Environment>(caller);
        }
        #[ink::test]
        fn register_works() {
            self.env().emit_event(Coverage { cov_of: 239 });
            let default_accounts = default_accounts();
            self.env().emit_event(Coverage { cov_of: 240 });
            let name = Hash::from([0x99; 32]);
            self.env().emit_event(Coverage { cov_of: 242 });
            set_next_caller(default_accounts.alice);
            self.env().emit_event(Coverage { cov_of: 243 });
            let mut contract = DomainNameService::new();
            self.env().emit_event(Coverage { cov_of: 245 });
            assert_eq!(contract.register(name), Ok(()));
            self.env().emit_event(Coverage { cov_of: 246 });
            assert_eq!(contract.register(name), Err(Error::NameAlreadyExists));
        }
        #[ink::test]
        fn set_address_works() {
            self.env().emit_event(Coverage { cov_of: 251 });
            let accounts = default_accounts();
            self.env().emit_event(Coverage { cov_of: 252 });
            let name = Hash::from([0x99; 32]);
            self.env().emit_event(Coverage { cov_of: 254 });
            set_next_caller(accounts.alice);
            self.env().emit_event(Coverage { cov_of: 256 });
            let mut contract = DomainNameService::new();
            self.env().emit_event(Coverage { cov_of: 257 });
            assert_eq!(contract.register(name), Ok(()));
            self.env().emit_event(Coverage { cov_of: 260 });
            set_next_caller(accounts.bob);
            self.env().emit_event(Coverage { cov_of: 261 });
            assert_eq!(
                contract.set_address(name, accounts.bob),
                Err(Error::CallerIsNotOwner)
            );
            self.env().emit_event(Coverage { cov_of: 267 });
            set_next_caller(accounts.alice);
            self.env().emit_event(Coverage { cov_of: 268 });
            assert_eq!(contract.set_address(name, accounts.bob), Ok(()));
            self.env().emit_event(Coverage { cov_of: 269 });
            assert_eq!(contract.get_address(name), accounts.bob);
            self.env().emit_event(Coverage { cov_of: 270 });
            contract.phink_assert_hash42_cant_be_registered();
        }
        #[ink::test]
        fn should_panic() {
            self.env().emit_event(Coverage { cov_of: 274 });
            let accounts = default_accounts();
            self.env().emit_event(Coverage { cov_of: 275 });
            set_next_caller(accounts.alice);
            self.env().emit_event(Coverage { cov_of: 276 });
            let mut contract = DomainNameService::new();
            self.env().emit_event(Coverage { cov_of: 277 });
            let illegal = Hash::from(FORBIDDEN_DOMAIN);
            self.env().emit_event(Coverage { cov_of: 278 });
            println!("{:?}", illegal);
            self.env().emit_event(Coverage { cov_of: 279 });
            assert_eq!(contract.transfer(illegal, accounts.bob), Ok(()));
            self.env().emit_event(Coverage { cov_of: 280 });
            contract.phink_assert_hash42_cant_be_registered();
        }
        #[ink::test]
        fn transfer_works() {
            self.env().emit_event(Coverage { cov_of: 284 });
            let accounts = default_accounts();
            self.env().emit_event(Coverage { cov_of: 285 });
            let name = Hash::from([0x99; 32]);
            self.env().emit_event(Coverage { cov_of: 287 });
            set_next_caller(accounts.alice);
            self.env().emit_event(Coverage { cov_of: 289 });
            let mut contract = DomainNameService::new();
            self.env().emit_event(Coverage { cov_of: 290 });
            assert_eq!(contract.register(name), Ok(()));
            self.env().emit_event(Coverage { cov_of: 291 });
            contract.phink_assert_hash42_cant_be_registered();
            self.env().emit_event(Coverage { cov_of: 293 });
            let illegal = Hash::from(FORBIDDEN_DOMAIN);
            self.env().emit_event(Coverage { cov_of: 296 });
            assert_eq!(contract.transfer(illegal, accounts.bob), Ok(()));
            self.env().emit_event(Coverage { cov_of: 299 });
            contract.phink_assert_hash42_cant_be_registered();
            self.env().emit_event(Coverage { cov_of: 302 });
            assert_eq!(
                contract.set_address(name, accounts.bob),
                Err(Error::CallerIsNotOwner)
            );
            self.env().emit_event(Coverage { cov_of: 307 });
            set_next_caller(accounts.bob);
            self.env().emit_event(Coverage { cov_of: 309 });
            assert_eq!(contract.set_address(name, accounts.bob), Ok(()));
            self.env().emit_event(Coverage { cov_of: 310 });
            assert_eq!(contract.get_address(name), accounts.bob);
        }
    }
}
