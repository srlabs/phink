use crate::runtime::System;
use crate::runtime::{ExistentialDeposit, Runtime, EXISTENTIAL_DEPOSIT};
use crate::CodeHash;
use env_logger::{Builder, Env};
use frame_support::__private::BasicExternalities;
use frame_support::pallet_prelude::StorageVersion;
use frame_support::traits::OnGenesis;
use pallet_contracts::Pallet;
use sp_keystore::{testing::MemoryKeystore, KeystoreExt};
use sp_runtime::BuildStorage;

pub struct ExtBuilder {
    existential_deposit: u64,
    storage_version: Option<StorageVersion>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            existential_deposit: ExistentialDeposit::get(),
            storage_version: None,
        }
    }
}

impl ExtBuilder {
    pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
        self.existential_deposit = existential_deposit;
        self
    }

    pub fn set_associated_consts(&self) {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
    }
    pub fn set_storage_version(mut self, version: u16) -> Self {
        self.storage_version = Some(StorageVersion::new(version));
        self
    }
    pub fn build(self) -> BasicExternalities {
        let env = Env::new().default_filter_or("runtime=debug");
        let _ = Builder::from_env(env).is_test(true).try_init();
        self.set_associated_consts();
        let mut t = frame_system::GenesisConfig::<Runtime>::default()
            .build_storage()
            .unwrap();
        pallet_balances::GenesisConfig::<Runtime> {
            balances: (0..5)
                .map(|i| [i; 32].into())
                .collect::<Vec<_>>()
                .iter()
                .cloned()
                .map(|k| (k, 10000000000000000000 * 2))
                .collect(),
        }
            .assimilate_storage(&mut t)
            .unwrap();
        let mut ext = BasicExternalities::new(t);
        ext.register_extension(KeystoreExt::new(MemoryKeystore::new()));
        ext.execute_with(|| {
            Pallet::<Runtime>::on_genesis();
            if let Some(storage_version) = self.storage_version {
                storage_version.put::<Pallet<Runtime>>();
            }
            System::set_block_number(1)
        });

        ext
    }
}
