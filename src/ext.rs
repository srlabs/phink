use crate::mocks::System;
use crate::mocks::{ExistentialDeposit, Runtime, EXISTENTIAL_DEPOSIT};
use crate::CodeHash;
use env_logger::{Builder, Env};
use frame::deps::{frame_system, sp_io};
use frame::prelude::StorageVersion;
use frame::testing_prelude::BuildStorage;
use frame::traits::OnGenesis;
use pallet_contracts::Pallet;
use sp_keystore::testing::MemoryKeystore;
use sp_keystore::KeystoreExt;

pub struct ExtBuilder {
    existential_deposit: u64,
    storage_version: Option<StorageVersion>,
    code_hashes: Vec<CodeHash<Runtime>>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            existential_deposit: ExistentialDeposit::get(),
            storage_version: None,
            code_hashes: vec![],
        }
    }
}

impl ExtBuilder {
    pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
        self.existential_deposit = existential_deposit;
        self
    }
    pub fn with_code_hashes(mut self, code_hashes: Vec<CodeHash<Runtime>>) -> Self {
        self.code_hashes = code_hashes;
        self
    }
    pub fn set_associated_consts(&self) {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
    }
    pub fn set_storage_version(mut self, version: u16) -> Self {
        self.storage_version = Some(StorageVersion::new(version));
        self
    }
    pub fn build(self) -> sp_io::TestExternalities {
        let env = Env::new().default_filter_or("runtime=debug");
        let _ = Builder::from_env(env).is_test(true).try_init();
        self.set_associated_consts();
        let mut t = frame_system::GenesisConfig::<Runtime>::default()
            .build_storage()
            .unwrap();
        pallet_balances::GenesisConfig::<Runtime> { balances: vec![] }
            .assimilate_storage(&mut t)
            .unwrap();
        let mut ext = sp_io::TestExternalities::new(t);
        ext.register_extension(KeystoreExt::new(MemoryKeystore::new()));
        ext.execute_with(|| {
            Pallet::<Runtime>::on_genesis();
            if let Some(storage_version) = self.storage_version {
                storage_version.put::<Pallet<Runtime>>();
            }
            System::set_block_number(1)
        });
        ext.execute_with(|| {
            for code_hash in self.code_hashes {
                panic!("TODO: fix me if you use code_hashes")
                // CodeInfoOf::<Test>::insert(code_hash, crate::CodeInfo::new(ALICE));
            }
        });
        ext
    }
}
