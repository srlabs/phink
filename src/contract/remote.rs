use anyhow::{
    bail,
    Context,
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
use sp_runtime::{
    DispatchError,
    ModuleError,
};
use std::{
    fmt::{
        Display,
        Formatter,
    },
    fs,
    path::PathBuf,
};
use v13::ContractInfoOf;

use payload::PayloadCrafter;

use crate::{
    cli::{
        config::Configuration,
        ziggy::ZiggyConfig,
    },
    contract::{
        custom::preferences::{
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
    cover::trace::CoverageTrace,
    instrumenter::instrumentation::Instrumenter,
};

pub type BalanceOf<T> =
    <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type EventRecord = frame_system::EventRecord<
    <Runtime as frame_system::Config>::RuntimeEvent,
    <Runtime as frame_system::Config>::Hash,
>;
type ContractResponse = ContractResult<Result<ExecReturnValue, DispatchError>, u128, EventRecord>;

#[derive(Clone)]
pub struct FullContractResponse(ContractResponse);

impl Display for FullContractResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let binding = self.clone().debug_message().remove_cov_from_trace();
        let message = String::from_utf8_lossy(&binding);
        write!(f, "{}", message.replace("\n", " "))
    }
}
impl FullContractResponse {
    pub fn from_contract_result(
        c: ContractResult<Result<ExecReturnValue, DispatchError>, u128, EventRecord>,
    ) -> Self {
        Self(c)
    }
    pub fn result(&self) -> &Result<ExecReturnValue, DispatchError> {
        &self.0.result
    }

    pub fn failed(&self) -> bool {
        self.0.result.is_err()
    }

    pub fn get(&self) -> &ContractResponse {
        &self.0
    }

    pub fn debug_message(self) -> CoverageTrace {
        CoverageTrace::from(self.0.debug_message)
    }
    pub fn is_trapped(&self) -> bool {
        if let Err(DispatchError::Module(ModuleError { message, .. })) = &self.0.result {
            if *message == Some("ContractTrapped") {
                return true;
            }
        }
        false
    }
}

#[derive(Clone)]
pub struct ContractSetup {
    pub genesis: Storage,
    pub contract_address: AccountIdOf<Runtime>,
    pub json_specs: String,
    pub path_to_specs: PathBuf,
    pub contract_path: PathBuf,
}

impl ContractSetup {
    pub const DEFAULT_GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);
    pub const DEFAULT_DEPLOYER: AccountId32 = AccountId32::new([1u8; 32]);

    /// Create a proper genesis storage, deploy and instantiate a given ink!
    /// contract
    pub fn initialize_wasm(config: ZiggyConfig) -> anyhow::Result<Self> {
        let finder = Instrumenter::new(config.clone())
            .find()
            .context("Couldn't execute `find` for this current config")?;
        let wasm_bytes = fs::read(&finder.wasm_path)
            .context(format!("Couldn't read WASM from{:?}", finder.wasm_path))?;

        let mut contract_addr: AccountIdOf<Runtime> = config
            .config
            .deployer_address
            .clone()
            .unwrap_or(ContractSetup::DEFAULT_DEPLOYER);

        println!(
            "🛠️Initializing contract address from the origin: {:?}",
            contract_addr
        );

        let json_specs = fs::read_to_string(&finder.specs_path)
            .context(format!("Couldn't read JSON from {:?}", finder.specs_path))?;

        let genesis_storage: Storage = {
            let storage = <Preferences as DevelopperPreferences>::runtime_storage();

            let mut chain = BasicExternalities::new(storage.clone());
            chain.execute_with(|| {
                let _ = <Preferences as DevelopperPreferences>::on_contract_initialize(); //This is optional and can `Err()` easily, so we use `let _`

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

        Ok(Self {
            genesis: genesis_storage,
            contract_address: contract_addr,
            json_specs,
            path_to_specs: finder.specs_path.to_path_buf(),
            contract_path: config.contract_path.clone(),
        })
    }

    /// Execute a function `payload` from the instantiated contract
    pub fn call(
        self,
        payload: &[u8],
        who: u8,
        transfer_value: BalanceOf<Runtime>,
        config: Configuration,
    ) -> FullContractResponse {
        FullContractResponse::from_contract_result(Contracts::bare_call(
            AccountId32::new([who; 32]),
            self.contract_address,
            transfer_value,
            config.default_gas_limit.unwrap_or(Self::DEFAULT_GAS_LIMIT),
            Configuration::parse_balance(config.storage_deposit_limit),
            payload.to_owned(),
            DebugInfo::UnsafeDebug,
            CollectEvents::UnsafeCollect,
            Determinism::Enforced,
        ))
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
                let hash = upload_info.code_hash;
                println!("✅ Upload successful. Code hash: {hash}",);
                hash
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
    ) -> anyhow::Result<AccountIdOf<Runtime>> {
        let data: Vec<u8> = if let Some(payload) = config.constructor_payload {
            hex::decode(payload.replace(" ", ""))
                .context("Impossible to hex-decode this. Check your config file")?
        } else {
            PayloadCrafter::extract_constructor(json_specs)?.into()
        };

        let initial_value = Configuration::parse_balance(config.instantiate_initial_value);

        let instantiate = Contracts::bare_instantiate(
            who.clone(),
            initial_value.unwrap_or(0),
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
                Ok(contract_info.account_id)
            }
            Err(e) => {
                bail!("❌ Failed to instantiate the contract, double check your `constructor_payload` please : {:?}", e);
            }
        }
    }
}
