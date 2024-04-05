#![recursion_limit = "1024"]

use crate::mocks::{
    Balances, Contracts, ExistentialDeposit, Runtime, RuntimeOrigin, System, DOLLARS,
    EXISTENTIAL_DEPOSIT,
};

use frame::deps::{frame_support, frame_system, sp_io, sp_runtime};

use frame_support::parameter_types;

use frame::prelude::Weight;
use frame::testing_prelude::{assert_ok, BuildStorage};
use frame::traits::{Currency, OnGenesis, StorageVersion};
use pallet_contracts::migration::v12::{CodeInfo, CodeInfoOf};
use pallet_contracts::{Code, CollectEvents, DebugInfo, Pallet};
use sp_core::crypto::AccountId32;
use sp_keystore::{testing::MemoryKeystore, KeystoreExt};

type CodeHash<T> = <T as frame_system::Config>::Hash;

mod mocks;

pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);
pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

fn main() {
    // let (wasm, _) = compile_module::<Test>("dummy").unwrap();
    ExtBuilder::default().build().execute_with(|| {
        let _ = Balances::deposit_creating(&ALICE, 1_000_000);

        let account_id = Contracts::bare_instantiate(
            ALICE,
            0,
            GAS_LIMIT,
            None,
            Code::Upload(vec![3]),
            vec![],
            vec![0x41, 0x41, 0x41, 0x41],
            DebugInfo::Skip,
            CollectEvents::Skip,
        )
            .result
            .unwrap()
            .account_id;

        assert_ok!(Contracts::call(
            RuntimeOrigin::signed(ALICE),
            account_id.clone(),
            0,
            GAS_LIMIT,
            None,
            vec![],
        ));
    });
}

pub struct ExtBuilder {
    existential_deposit: u64,
    storage_version: Option<StorageVersion>,
    code_hashes: Vec<CodeHash<Runtime>>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            existential_deposit: ExistentialDeposit::get() as u64,
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
        use env_logger::{Builder, Env};
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
                panic!("Kevin TODO: fix me if you use code_hashes")
                // CodeInfoOf::<Test>::insert(code_hash, crate::CodeInfo::new(ALICE));
            }
        });
        ext
    }
}
// pub fn compile_module<T>(
//     fixture_name: &str,
// ) -> anyhow::Result<(Vec<u8>, <T::Hashing as Hash>::Output)>
//     where
//         T: frame_system::Config,
// {
//     let out_dir: std::path::PathBuf = env!("OUT_DIR").into();
//     let fixture_path = out_dir.join(format!("{fixture_name}.wasm"));
//     match fs::read(fixture_path) {
//         Ok(wasm_binary) => {
//             let code_hash = T::Hashing::hash(&wasm_binary);
//             Ok((wasm_binary, code_hash))
//         }
//         Err(_) => legacy_compile_module::<T>(fixture_name),
//     }
// }
//
// fn legacy_compile_module<T>(
//     fixture_name: &str,
// ) -> anyhow::Result<(Vec<u8>, <T::Hashing as Hash>::Output)>
//     where
//         T: frame_system::Config,
// {
//     let fixture_path = wat_root_dir().join(format!("{fixture_name}.wat"));
//     let wasm_binary = wat::parse_file(fixture_path)?;
//     let code_hash = T::Hashing::hash(&wasm_binary);
//     Ok((wasm_binary, code_hash))
// }
