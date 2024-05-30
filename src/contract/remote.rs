use std::fs;
use std::path::PathBuf;

use frame_support::__private::BasicExternalities;
use frame_support::pallet_prelude::Weight;
use frame_support::traits::fungible::Inspect;
use pallet_contracts::ContractResult;
use pallet_contracts::{Code, CollectEvents, Config, DebugInfo, Determinism, ExecReturnValue};
use sp_core::{crypto::AccountId32, storage::Storage, H256};
use sp_runtime::{BuildStorage, DispatchError};

use crate::contract::payload;
use crate::contract::runtime::{BalancesConfig, Contracts, Runtime, RuntimeGenesisConfig};

pub type BalanceOf<T> =
    <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
pub type Test = Runtime;
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
    pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

    pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

    /// Create a proper genesis storage, deploy and instantiate a given ink! contract
    ///
    /// # Arguments
    ///
    /// * `wasm_bytes`: the bytes of the WASM contract
    /// * `json_specs`: JSON specs of the contract, i.e. dns.json

    pub fn initialize_wasm(wasm_bytes: Vec<u8>, path_to_specs: &PathBuf) -> ContractBridge {
        let mut contract_addr: AccountIdOf<Test> = AccountId32::new([42u8; 32]); // dummy account
        let json_specs = fs::read_to_string(path_to_specs.clone()).unwrap();
        let genesis_storage: Storage = {
            let storage = Self::storage();
            let mut chain = BasicExternalities::new(storage.clone());
            chain.execute_with(|| {
                let code_hash = Self::upload(&wasm_bytes);
                contract_addr = Self::instantiate(&json_specs, code_hash).expect(
                    "Can't fetch the contract address because because of incorrect instantiation",
                );
                // We verify if the contract is correctly instantiated
                assert!(
                    pallet_contracts::migration::v13::ContractInfoOf::<Test>::contains_key(
                        &contract_addr
                    )
                );
            });
            chain.into_storages()
        };

        Self {
            genesis: genesis_storage,
            contract_address: contract_addr,
            json_specs,
            path_to_specs: path_to_specs.clone(),
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
        payload: &Vec<u8>,
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
            payload.clone(),
            DebugInfo::UnsafeDebug,
            CollectEvents::UnsafeCollect,
            Determinism::Enforced,
        )
    }

    pub fn upload(wasm_bytes: &Vec<u8>) -> H256 {
        let code_hash = Contracts::bare_upload_code(
            Self::ALICE,
            wasm_bytes.clone(),
            None,
            Determinism::Enforced,
        )
        .unwrap()
        .code_hash;
        code_hash
    }

    pub fn instantiate(json_specs: &String, code_hash: H256) -> Option<AccountIdOf<Test>> {
        let instantiate = Contracts::bare_instantiate(
            Self::ALICE,
            0,
            Self::GAS_LIMIT,
            None,
            Code::Existing(code_hash),
            Vec::from(payload::PayloadCrafter::get_constructor(json_specs).clone()?),
            vec![],
            DebugInfo::UnsafeDebug,
            CollectEvents::UnsafeCollect,
        );

        Some(instantiate.result.unwrap().account_id)
    }

    fn storage() -> Storage {
        let storage = RuntimeGenesisConfig {
            balances: BalancesConfig {
                balances: (0..5) // Lot of money for Alice, Bob ... Ferdie
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
