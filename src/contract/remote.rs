use std::path::Path;
use std::{fs, path::PathBuf};

use frame_support::{
    __private::BasicExternalities, pallet_prelude::Weight, traits::fungible::Inspect,
};
use pallet_contracts::{
    Code, CollectEvents, Config, ContractResult, DebugInfo, Determinism, ExecReturnValue,
};
use sp_core::{crypto::AccountId32, storage::Storage, H256};
use sp_runtime::{BuildStorage, DispatchError};

use crate::contract::runtime::AccountId;
use crate::{
    contract::payload,
    contract::runtime::{BalancesConfig, Contracts, Runtime, RuntimeGenesisConfig},
};

pub type BalanceOf<T> =
    <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
pub type Test = Runtime; // Alias to your own Runtime
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type EventRecord = frame_system::EventRecord<
    <Runtime as frame_system::Config>::RuntimeEvent,
    <Runtime as frame_system::Config>::Hash,
>;

pub type FullContractResponse =
    ContractResult<Result<ExecReturnValue, DispatchError>, u128, EventRecord>;

#[derive(Clone)]
pub struct ContractBridge {
    pub genesis: Storage,
    pub contract_address: AccountIdOf<Test>,
    pub json_specs: String,
    pub path_to_specs: PathBuf,
}

impl ContractBridge {
    //TODO: make this configurable
    pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

    /// Create a proper genesis storage, deploy and instantiate a given ink! contract
    ///
    /// # Arguments
    ///
    /// * `wasm_bytes`: the bytes of the WASM contract
    /// * `json_specs`: JSON specs of the contract, i.e. contract.json

    pub fn initialize_wasm(
        wasm_bytes: Vec<u8>,
        path_to_specs: &Path,
        origin: AccountId,
    ) -> ContractBridge {
        let mut contract_addr: AccountIdOf<Test> = origin;
        println!(
            "🛠️Initializing contract address from the origin: {:?}",
            contract_addr
        );

        let json_specs = fs::read_to_string(path_to_specs).unwrap();
        let genesis_storage: Storage = {
            let storage = Self::storage();
            let mut chain = BasicExternalities::new(storage.clone());
            chain.execute_with(|| {
                let code_hash = Self::upload(&wasm_bytes, contract_addr.clone());
                println!("🔍 Uploaded the WASM bytes. Code hash: {:?}", code_hash);

                contract_addr = Self::instantiate(&json_specs, code_hash, contract_addr.clone()).expect(
                    "🙅 Can't fetch the contract address because because of incorrect instantiation",
                );

                // We verify if the contract is correctly instantiated
                assert!(
                    pallet_contracts::migration::v13::ContractInfoOf::<Test>::contains_key(
                        &contract_addr
                    )
                );
                println!("🏗️ Instantiated the contract. New contract address: {:?}", contract_addr);
            });
            chain.into_storages()
        };

        Self {
            genesis: genesis_storage,
            contract_address: contract_addr,
            json_specs,
            path_to_specs: path_to_specs.to_path_buf(),
        }
    }

    /// Execute a function (`payload`) from the instantiated contract
    ///
    /// # Arguments
    ///
    /// * `payload`: The scale-encoded `data` to pass to the contract
    /// * `who`: AccountId of the caller
    /// * `amount`: Amount to pass to the contract
    pub fn call(
        self,
        payload: &[u8],
        who: u8,
        transfer_value: BalanceOf<Test>,
    ) -> FullContractResponse {
        let acc = AccountId32::new([who.try_into().unwrap(); 32]);
        Contracts::bare_call(
            acc,
            self.contract_address,
            transfer_value,
            Self::GAS_LIMIT,
            None,
            payload.to_owned(),
            DebugInfo::UnsafeDebug,
            CollectEvents::UnsafeCollect,
            Determinism::Enforced,
        )
    }

    pub fn upload(wasm_bytes: &[u8], who: AccountId) -> H256 {
        println!("📤 Starting upload of WASM bytes by: {:?}", who);
        let upload_result = Contracts::bare_upload_code(
            who.clone(),
            wasm_bytes.to_owned(),
            None,
            Determinism::Enforced,
        );
        match upload_result {
            Ok(upload_info) => {
                println!(
                    "✅ Upload successful. Code hash: {:?}",
                    upload_info.code_hash
                );
                upload_info.code_hash
            }
            Err(e) => {
                panic!("❌ Upload failed for: {:?} with error: {:?}", who, e);
            }
        }
    }

    pub fn instantiate(
        json_specs: &str,
        code_hash: H256,
        who: AccountId,
    ) -> Option<AccountIdOf<Test>> {
        let instantiate = Contracts::bare_instantiate(
            who,
            0,
            Self::GAS_LIMIT,
            None,
            Code::Existing(code_hash),
            Vec::from(payload::PayloadCrafter::get_constructor(json_specs)?),
            vec![],
            DebugInfo::UnsafeDebug,
            CollectEvents::UnsafeCollect,
        );

        Some(instantiate.result.unwrap().account_id)
    }

    //TODO: Make this configurable as a Generic kind of
    fn storage() -> Storage {
        let storage = RuntimeGenesisConfig {
            balances: BalancesConfig {
                balances: (0..u8::MAX) // Lot of money for Alice, Bob ... Ferdie
                    .map(|i| [i; 32].into())
                    .collect::<Vec<_>>()
                    .iter()
                    .cloned()
                    .map(|k| (k, 10000000000000000000 * 2))
                    .collect(),
            },
            ..Default::default()
        }
        .build_storage()
        .unwrap();
        storage
    }
}
