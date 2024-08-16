use std::{
    fs,
    path::{
        Path,
        PathBuf,
    },
};

use frame_support::{
    __private::BasicExternalities,
    pallet_prelude::Weight,
    traits::fungible::Inspect,
};
use migration::v13;
use pallet_contracts::{
    migration,
    Code,
    CollectEvents,
    Config,
    ContractResult,
    DebugInfo,
    Determinism,
    ExecReturnValue,
};
use sp_core::{
    crypto::AccountId32,
    storage::Storage,
    H256,
};
use sp_runtime::DispatchError;
use v13::ContractInfoOf;

use payload::PayloadCrafter;

use crate::{
    cli::{
        config::Configuration,
        ziggy::ZiggyConfig,
    },
    contract::{
        custom::{
            DevelopperPreferences,
            Preferences,
        },
        payload,
        runtime::{
            AccountId,
            Contracts,
            Runtime,
        },
    },
    instrumenter::instrumentation::Instrumenter,
};

pub type BalanceOf<T> =
    <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

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
    pub contract_address: AccountIdOf<Runtime>,
    pub json_specs: String,
    pub path_to_specs: PathBuf,
    pub contract_path: PathBuf
}

impl ContractBridge {
    pub const DEFAULT_GAS_LIMIT: Weight =
        Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);
    pub const DEFAULT_DEPLOYER: AccountId32 = AccountId32::new([1u8; 32]);

    /// Create a proper genesis storage, deploy and instantiate a given ink!
    /// contract
    pub fn initialize_wasm(config: ZiggyConfig) -> ContractBridge {
        let finder = Instrumenter::new(config.contract_path.clone()).find().unwrap();
        let wasm_bytes = fs::read(&finder.wasm_path).unwrap();

        let mut contract_addr: AccountIdOf<Runtime> = config
            .config
            .deployer_address
            .clone()
            .unwrap_or(ContractBridge::DEFAULT_DEPLOYER);

        println!(
            "🛠️Initializing contract address from the origin: {:?}",
            contract_addr
        );

        let json_specs = fs::read_to_string(finder.specs_path.clone()).unwrap();
        let genesis_storage: Storage = {
            let storage = <Preferences as DevelopperPreferences>::runtime_storage();

            let mut chain = BasicExternalities::new(storage.clone());
            chain.execute_with(|| {

              <Preferences as DevelopperPreferences>::on_contract_initialize();

                let code_hash = Self::upload(&wasm_bytes, contract_addr.clone());

                contract_addr = Self::instantiate(&json_specs, code_hash, contract_addr.clone(), config.config).expect(
                    "🙅 Can't fetch the contract address because of incorrect instantiation",
                );

                // We verify if the contract is correctly instantiated
                if !ContractInfoOf::<Runtime>::contains_key(&contract_addr) {
                    panic!(
                        "🚨 Contract Instantiation Failed!
                            This error is likely due to a misconfigured constructor payload in the configuration file.
                            Please ensure the correct payload for the constructor (selector + parameters) is provided, just as you would for a regular deployment. You can use the `constructor_payload` field inside the TOML configuration file for this purpose.
                            To generate your payload, please use `cargo contract`:
                            Example:
                            ❯ cargo contract encode --message \"new\" --args 1234 1337 \"0x8bb565d32618e40e8b9991c00d05b52a89ddbed0c7d9103be5610ab8a713fc67\" \"0x2a18c7d454ba9cc46f97fff2f048db136d975fb1401e75c09ed03050864bcd19\" \"0xbf0108f5882aee2e97f84f054c1645c1598499e9dfcf179e367e4d41c3130ee8\" -- target/ink/multi_contract_caller.\
                            Encoded data: 9BAE9D5E...3130EE8"
                    );
                }
            });

            chain.into_storages()
        };

        Self {
            genesis: genesis_storage,
            contract_address: contract_addr,
            json_specs,
            path_to_specs: finder.specs_path.to_path_buf(),
            contract_path: config.contract_path.clone(),
        }
    }

    /// Execute a function `payload` from the instantiated contract
    pub fn call(
        self,
        payload: &[u8],
        who: u8,
        transfer_value: BalanceOf<Runtime>,
        config: Configuration,
    ) -> FullContractResponse {
        Contracts::bare_call(
            AccountId32::new([who; 32]),
            self.contract_address,
            transfer_value,
            config.default_gas_limit.unwrap_or(Self::DEFAULT_GAS_LIMIT),
            Configuration::parse_balance(config.storage_deposit_limit),
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
        config: Configuration,
    ) -> Option<AccountIdOf<Runtime>> {
        let data: Vec<u8> = if let Some(payload) = config.constructor_payload {
            hex::decode(payload)
                .expect("Impossible to hex-decode this. Check your config file")
        } else {
            PayloadCrafter::get_constructor(json_specs)?.into()
        };

        let instantiate_initial_value: Option<BalanceOf<Runtime>> =
            Configuration::parse_balance(config.instantiate_initial_value);

        let instantiate = Contracts::bare_instantiate(
            who.clone(),
            instantiate_initial_value.unwrap_or(0),
            config.default_gas_limit.unwrap_or(Self::DEFAULT_GAS_LIMIT),
            None,
            Code::Existing(code_hash),
            data,
            vec![],
            DebugInfo::UnsafeDebug,
            CollectEvents::UnsafeCollect,
        );

        match instantiate.result {
            Ok(contract_info) => {
                println!("🔍 Instantiated the contract, using account {:?}", who);
                Some(contract_info.account_id)
            }
            Err(e) => {
                eprintln!("❌ Failed to instantiate the contract, double check your `constructor_payload` please : {:?}", e);
                None
            }
        }
    }
}
