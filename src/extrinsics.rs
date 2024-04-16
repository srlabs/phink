use super::{BalanceOf, CodeHash, Runtime, ALICE};
use crate::{RuntimeOrigin, Weight};
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use frame_system::{self, pallet_prelude::OriginFor, EventRecord};
use pallet_contracts::{
    Code, CollectEvents, ContractExecResult, ContractInstantiateResult, DebugInfo, Determinism,
    ExecReturnValue, InstantiateReturnValue, Pallet,
};

use sp_core::crypto::AccountId32;
use sp_runtime::traits::StaticLookup;

use parity_scale_codec::Compact;
use paste::paste;

type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

type Test = Runtime;
type EventRecordOf<T> =
    EventRecord<<T as frame_system::Config>::RuntimeEvent, <T as frame_system::Config>::Hash>;

pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

macro_rules! builder {
	(
		$method:ident($($field:ident: $type:ty,)*) -> $result:ty
	) => {
		paste!{
			builder!([< $method:camel Builder >], $method($($field: $type,)* ) -> $result);
		}
	};
	(
		$name:ident,
		$method:ident(
			$($field:ident: $type:ty,)*
		) -> $result:ty
	) => {
		#[doc = concat!("A builder to construct a ", stringify!($method), " call")]
		pub struct $name {
			$($field: $type,)*
		}

		#[allow(dead_code)]
		impl $name
		{
			$(
				#[doc = concat!("Set the ", stringify!($field))]
				pub fn $field(mut self, value: $type) -> Self {
					self.$field = value;
					self
				}
			)*

			#[doc = concat!("Build the ", stringify!($method), " call")]
			pub fn build(self) -> $result {
				Pallet::<Test>::$method(
					$(self.$field,)*
				)
			}
		}
	}
}

builder!(
    instantiate_with_code(
        origin: OriginFor<Test>,
        value: BalanceOf<Test>,
        gas_limit: Weight,
        storage_deposit_limit: Option<Compact<BalanceOf<Test>>>,
        code: Vec<u8>,
        data: Vec<u8>,
        salt: Vec<u8>,
    ) -> DispatchResultWithPostInfo
);

builder!(
    instantiate(
        origin: OriginFor<Test>,
        value: BalanceOf<Test>,
        gas_limit: Weight,
        storage_deposit_limit: Option<Compact<BalanceOf<Test>>>,
        code_hash: CodeHash<Test>,
        data: Vec<u8>,
        salt: Vec<u8>,
    ) -> DispatchResultWithPostInfo
);

builder!(
    bare_instantiate(
        origin: AccountIdOf<Test>,
        value: BalanceOf<Test>,
        gas_limit: Weight,
        storage_deposit_limit: Option<BalanceOf<Test>>,
        code: Code<CodeHash<Test>>,
        data: Vec<u8>,
        salt: Vec<u8>,
        debug: DebugInfo,
        collect_events: CollectEvents,
    ) -> ContractInstantiateResult<AccountIdOf<Test>, BalanceOf<Test>, EventRecordOf<Test>>
);

builder!(
    call(
        origin: OriginFor<Test>,
        dest: AccountIdLookupOf<Test>,
        value: BalanceOf<Test>,
        gas_limit: Weight,
        storage_deposit_limit: Option<Compact<BalanceOf<Test>>>,
        data: Vec<u8>,
    ) -> DispatchResultWithPostInfo
);

builder!(
    bare_call(
        origin: AccountIdOf<Test>,
        dest: AccountIdOf<Test>,
        value: BalanceOf<Test>,
        gas_limit: Weight,
        storage_deposit_limit: Option<BalanceOf<Test>>,
        data: Vec<u8>,
        debug: DebugInfo,
        collect_events: CollectEvents,
        determinism: Determinism,
    ) -> ContractExecResult<BalanceOf<Test>, EventRecordOf<Test>>
);

/// Create a [`BareInstantiateBuilder`] with default values.
pub fn bare_instantiate(code: Code<CodeHash<Test>>) -> BareInstantiateBuilder {
    BareInstantiateBuilder {
        origin: ALICE,
        value: 0,
        gas_limit: GAS_LIMIT,
        storage_deposit_limit: None,
        code,
        data: vec![],
        salt: vec![],
        debug: DebugInfo::Skip,
        collect_events: CollectEvents::Skip,
    }
}

impl BareInstantiateBuilder {
    /// Build the instantiate call and unwrap the result.
    pub fn build_and_unwrap_result(self) -> InstantiateReturnValue<AccountIdOf<Test>> {
        self.build().result.unwrap()
    }

    /// Build the instantiate call and unwrap the account id.
    pub fn build_and_unwrap_account_id(self) -> AccountIdOf<Test> {
        self.build().result.unwrap().account_id
    }
}

/// Create a [`BareCallBuilder`] with default values.
pub fn bare_call(dest: AccountId32) -> BareCallBuilder {
    BareCallBuilder {
        origin: ALICE,
        dest,
        value: 0,
        gas_limit: GAS_LIMIT,
        storage_deposit_limit: None,
        data: vec![],
        debug: DebugInfo::Skip,
        collect_events: CollectEvents::Skip,
        determinism: Determinism::Enforced,
    }
}

impl BareCallBuilder {
    /// Build the call and unwrap the result.
    pub fn build_and_unwrap_result(self) -> ExecReturnValue {
        self.build().result.unwrap()
    }
}

/// Create an [`InstantiateWithCodeBuilder`] with default values.
pub fn instantiate_with_code(code: Vec<u8>) -> InstantiateWithCodeBuilder {
    InstantiateWithCodeBuilder {
        origin: RuntimeOrigin::signed(ALICE),
        value: 0,
        gas_limit: GAS_LIMIT,
        storage_deposit_limit: None,
        code,
        data: vec![],
        salt: vec![],
    }
}

/// Create an [`InstantiateBuilder`] with default values.
pub fn instantiate(code_hash: CodeHash<Test>) -> InstantiateBuilder {
    InstantiateBuilder {
        origin: RuntimeOrigin::signed(ALICE),
        value: 0,
        gas_limit: GAS_LIMIT,
        storage_deposit_limit: None,
        code_hash,
        data: vec![],
        salt: vec![],
    }
}

/// Create a [`CallBuilder`] with default values.
pub fn call(dest: AccountIdLookupOf<Test>) -> CallBuilder {
    CallBuilder {
        origin: RuntimeOrigin::signed(ALICE),
        dest,
        value: 0,
        gas_limit: GAS_LIMIT,
        storage_deposit_limit: None,
        data: vec![],
    }
}
