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
    ResultOf,
};

pub type BalanceOf<T> =
    <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type EventRecord = frame_system::EventRecord<
    <Runtime as frame_system::Config>::RuntimeEvent,
    <Runtime as frame_system::Config>::Hash,
>;
pub type ContractResponse =
    ContractResult<Result<ExecReturnValue, DispatchError>, u128, EventRecord>;

#[derive(Clone, scale_info::TypeInfo)]
pub struct FullContractResponse(pub ContractResponse);

impl Display for FullContractResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.clone()
                .debug_message()
                .remove_cov_from_trace()
                .replace("\n", " ")
        )
    }
}
impl FullContractResponse {
    pub fn get_response(&self) -> &ContractResponse {
        &self.0
    }

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
}

impl ContractSetup {
    pub const DEFAULT_GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);
    pub const DEFAULT_DEPLOYER: AccountId32 = AccountId32::new([1u8; 32]);

    /// Create a proper genesis storage, deploy and instantiate a given ink!
    /// contract
    pub fn initialize_wasm(config: ZiggyConfig) -> ResultOf<Self> {
        let finder = Instrumenter::new(config.clone())
            .find()
            .context("Couldn't execute `find` for this current config")?;
        let wasm_bytes = fs::read(&finder.wasm_path)
            .context(format!("Couldn't read WASM from {:?}", finder.wasm_path))?;

        let conf = config.config();
        let contract_address: &AccountIdOf<Runtime> = conf.deployer_address();
        if conf.verbose {
            println!("🛠️Initializing contract address from the origin: {contract_address:?}");
        }

        let path_to_specs = finder.specs_path;
        let json_specs = fs::read_to_string(&path_to_specs).context(format!(
            "Couldn't read JSON from {path_to_specs:?}. \
            Check that you have a WASM AND JSON file in your target/ directory"
        ))?;

        let genesis: (Storage, AccountIdOf<Runtime>) = {
            let storage = <Preferences as DevelopperPreferences>::runtime_storage();

            let mut chain = BasicExternalities::new(storage.clone());
            let address = chain.execute_with(|| {
                let _ = <Preferences as DevelopperPreferences>::on_contract_initialize(); //This is optional and can `Err()` easily, so we use `let _`

                if conf.verbose {
                    println!("📤 Starting upload of WASM bytes by: {contract_address:?}");
                }

                let code_hash = Self::upload(&wasm_bytes, contract_address)?;

                let new_contract_address: AccountIdOf<Runtime> = Self::instantiate(&json_specs, code_hash, contract_address, conf)
                    .context("Can't fetch the contract address because of incorrect instantiation")?;

                // We verify if the contract is correctly instantiated
                if !ContractInfoOf::<Runtime>::contains_key(&new_contract_address) {
                    bail!(
                        "Contract instantiation failed!
                            This error is likely due to a misconfigured constructor payload in the configuration file.
                            Please ensure the correct payload for the constructor (selector + parameters) is provided, just as you would for a regular deployment. You can use the `constructor_payload` field inside the TOML configuration file for this purpose.
                            To generate your payload, please use `cargo contract`, for instance
                            ```sh
                            cargo contract encode --message \"new\" --args 1234 1337 \"0x8bb565d32618e40e8b9991c00d05b52a89ddbed0c7d9103be5610ab8a713fc67\" \"0x2a18c7d454ba9cc46f97fff2f048db136d975fb1401e75c09ed03050864bcd19\" \"0xbf0108f5882aee2e97f84f054c1645c1598499e9dfcf179e367e4d41c3130ee8\" -- target/ink/multi_contract_caller.\
                            \
                            Encoded data: 9BAE9D5E...3130EE8\
                            ```"
                    );
                }
                Ok(new_contract_address)
            })?;

            (chain.into_storages(), address)
        };

        Ok(Self {
            genesis: genesis.0,
            contract_address: genesis.1,
            json_specs,
            path_to_specs,
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
            Configuration::parse_balance(&config.storage_deposit_limit),
            payload.to_owned(),
            DebugInfo::UnsafeDebug,
            CollectEvents::UnsafeCollect,
            Determinism::Enforced,
        ))
    }

    pub fn upload(wasm_bytes: &[u8], who: &AccountId) -> ResultOf<H256> {
        let upload_result = Contracts::bare_upload_code(
            who.clone(),
            Vec::from(wasm_bytes),
            None,
            Determinism::Enforced,
        );
        match upload_result {
            Ok(upload_info) => {
                let hash = upload_info.code_hash;
                println!("✅ Upload successful. Code hash: {hash}",);
                Ok(hash)
            }
            Err(e) => {
                bail!("❌ Upload failed for: {who:?} with error: {e:?}");
            }
        }
    }

    pub fn instantiate(
        json_specs: &str,
        code_hash: H256,
        who: &AccountId,
        config: &Configuration,
    ) -> ResultOf<AccountIdOf<Runtime>> {
        let data: Vec<u8> = if let Some(payload) = &config.constructor_payload {
            hex::decode(payload.replace(" ", ""))
                .context("Impossible to hex-decode this. Check your config file")?
        } else {
            PayloadCrafter::extract_constructor(json_specs)
                .context("Couldn't extract the constructor from the JSON specs")?
                .into()
        };

        let initial_value = Configuration::parse_balance(&config.instantiate_initial_value);

        let instantiate = Contracts::bare_instantiate(
            who.clone(),
            initial_value.unwrap_or(0),
            config.default_gas_limit.unwrap_or_default(),
            None,
            Code::Existing(code_hash),
            data,
            vec![],
            DebugInfo::UnsafeDebug,
            CollectEvents::UnsafeCollect,
        );

        match instantiate.result {
            Ok(contract_info) => {
                println!("🔍 Instantiated the contract, contract's account is {who:?}");
                Ok(contract_info.account_id)
            }
            Err(e) => {
                let debug = String::from_utf8_lossy(instantiate.debug_message.as_ref());
                bail!("❌ Failed to instantiate the contract, double check your `constructor_payload` please ({e:?})\n Details : {debug:?}");
            }
        }
    }
}
