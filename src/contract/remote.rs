use std::fs;
use std::path::PathBuf;
use contract_metadata::ContractMetadata;
use frame_support::__private::BasicExternalities;
use frame_support::pallet_prelude::Weight;
use ink_metadata::InkProject;
use pallet_contracts::{
    Code, CollectEvents, ContractExecResult, DebugInfo, Determinism, ExecReturnValue,
};
use sp_core::{crypto::AccountId32, storage::Storage, H256};
use sp_runtime::{BuildStorage, DispatchError};

use crate::{
    AccountIdOf, Test, ALICE,
};
use crate::contract::payload;
use crate::contract::runtime::{BalancesConfig, Contracts, RuntimeGenesisConfig};

pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

#[derive(Clone)]
pub struct ContractBridge {
    pub genesis: Storage,
    pub contract_address: AccountIdOf<Test>,
    pub json_specs: String,
    pub path_to_specs: PathBuf
}


impl ContractBridge {
    /// Create a proper genesis storage, deploy and instantiate a given ink! contract
    ///
    /// # Arguments
    ///
    /// * `wasm_bytes`: the bytes of the WASM contract
    /// * `json_specs`: JSON specs of the contract, i.e. dns.json
    ///
    /// returns: DeployedSetup
    ///
    /// # Example
    ///
    /// ```
    ///let dns_wasm: Vec<u8> = fs::read("sample/dns/target/ink/dns.wasm").unwrap();
    // let dns_wasm_bytes: Vec<u8> = include_bytes!(".../dns/target/ink/dns.wasm")[..].to_vec();
    // let dns_specs = fs::read_to_string("sample/dns/target/ink/dns.json").unwrap();
    // let ct =
    //     deploy::initialize_contract(dns_wasm_bytes, dns_specs.clone());
    /// ```
    pub fn initialize_contract(wasm_bytes: Vec<u8>, path_to_specs: PathBuf) -> ContractBridge {
        let mut contract_addr: AccountIdOf<Test> = AccountId32::new([42u8; 32]); // dummy account
        let json_specs = fs::read_to_string(path_to_specs.clone()).unwrap();
        let genesis_storage: Storage = {
            let storage = storage();
            let mut chain = BasicExternalities::new(storage.clone());
            chain.execute_with(|| {
                let code_hash = upload(&wasm_bytes);
                contract_addr = instantiate(&json_specs, code_hash).expect(
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
            path_to_specs
        }
    }

    pub fn call(self, payload: &Vec<u8>) -> Result<ExecReturnValue, DispatchError> {
        return Contracts::bare_call(
            ALICE,
            self.contract_address,
            0,
            GAS_LIMIT,
            None,
            payload.clone(),
            DebugInfo::UnsafeDebug,
            CollectEvents::UnsafeCollect,
            Determinism::Relaxed,
        )
        .result;
    }
}

fn instantiate(json_specs: &String, code_hash: H256) -> Option<AccountIdOf<Test>> {
    Some(
        Contracts::bare_instantiate(
            ALICE,
            0,
            GAS_LIMIT,
            None,
            Code::Existing(code_hash),
            Vec::from(payload::PayloadCrafter::get_constructor(json_specs).clone()?),
            vec![],
            DebugInfo::UnsafeDebug,
            CollectEvents::UnsafeCollect,
        )
        .result
        .unwrap()
        .account_id,
    )
}

fn upload(wasm_bytes: &Vec<u8>) -> H256 {
    let code_hash =
        Contracts::bare_upload_code(ALICE, wasm_bytes.clone(), None, Determinism::Relaxed)
            .unwrap()
            .code_hash;
    code_hash
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
