// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod builder;
mod pallet_dummy;
mod test_debug;

use self::{
	test_debug::TestDebug,
	test_utils::{ensure_stored, expected_deposit, hash},
};
use crate::{
	self as pallet_contracts,
	chain_extension::{
		ChainExtension, Environment, Ext, InitState, RegisteredChainExtension,
		Result as ExtensionResult, RetVal, ReturnFlags, SysConfig,
	},
	exec::{Frame, Key},
	migration::codegen::LATEST_MIGRATION_VERSION,
	primitives::CodeUploadReturnValue,
	storage::DeletionQueueManager,
	tests::test_utils::{get_contract, get_contract_checked},
	wasm::{Determinism, ReturnErrorCode as RuntimeReturnCode},
	weights::WeightInfo,
	Array, BalanceOf, Code, CodeHash, CodeInfoOf, CollectEvents, Config, ContractInfo,
	ContractInfoOf, DebugInfo, DefaultAddressGenerator, DeletionQueueCounter, Error, HoldReason,
	MigrationInProgress, Origin, Pallet, PristineCode, Schedule,
};
use assert_matches::assert_matches;
use codec::{Decode, Encode};
use frame_support::{
	assert_err, assert_err_ignore_postinfo, assert_err_with_weight, assert_noop, assert_ok,
	derive_impl,
	dispatch::{DispatchErrorWithPostInfo, PostDispatchInfo},
	pallet_prelude::EnsureOrigin,
	parameter_types,
	storage::child,
	traits::{
		fungible::{BalancedHold, Inspect, Mutate, MutateHold},
		tokens::Preservation,
		ConstU32, ConstU64, Contains, OnIdle, OnInitialize, StorageVersion,
	},
	weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
};
use frame_system::{EventRecord, Phase};
use pallet_contracts_fixtures::compile_module;
use pretty_assertions::{assert_eq, assert_ne};
use sp_core::ByteArray;
use sp_io::hashing::blake2_256;
use sp_keystore::{testing::MemoryKeystore, KeystoreExt};
use sp_runtime::{
	testing::H256,
	traits::{BlakeTwo256, Convert, Hash, IdentityLookup},
	AccountId32, BuildStorage, DispatchError, Perbill, TokenError,
};

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Timestamp: pallet_timestamp,
		Randomness: pallet_insecure_randomness_collective_flip,
		Utility: pallet_utility,
		Contracts: pallet_contracts,
		Proxy: pallet_proxy,
		Dummy: pallet_dummy
	}
);

macro_rules! assert_return_code {
	( $x:expr , $y:expr $(,)? ) => {{
		assert_eq!(u32::from_le_bytes($x.data[..].try_into().unwrap()), $y as u32);
	}};
}

macro_rules! assert_refcount {
	( $code_hash:expr , $should:expr $(,)? ) => {{
		let is = crate::CodeInfoOf::<Test>::get($code_hash).map(|m| m.refcount()).unwrap();
		assert_eq!(is, $should);
	}};
}

pub mod test_utils {

	use super::{Contracts, DepositPerByte, DepositPerItem, Hash, SysConfig, Test};
	use crate::{
		exec::AccountIdOf, BalanceOf, CodeHash, CodeInfo, CodeInfoOf, Config, ContractInfo,
		ContractInfoOf, Nonce, PristineCode,
	};
	use codec::{Encode, MaxEncodedLen};
	use frame_support::traits::fungible::{InspectHold, Mutate};

	pub fn place_contract(address: &AccountIdOf<Test>, code_hash: CodeHash<Test>) {
		let nonce = <Nonce<Test>>::mutate(|counter| {
			*counter += 1;
			*counter
		});
		set_balance(address, Contracts::min_balance() * 10);
		<CodeInfoOf<Test>>::insert(code_hash, CodeInfo::new(address.clone()));
		let contract = <ContractInfo<Test>>::new(&address, nonce, code_hash).unwrap();
		<ContractInfoOf<Test>>::insert(address, contract);
	}
	pub fn set_balance(who: &AccountIdOf<Test>, amount: u64) {
		let _ = <Test as Config>::Currency::set_balance(who, amount);
	}
	pub fn get_balance(who: &AccountIdOf<Test>) -> u64 {
		<Test as Config>::Currency::free_balance(who)
	}
	pub fn get_balance_on_hold(
		reason: &<Test as Config>::RuntimeHoldReason,
		who: &AccountIdOf<Test>,
	) -> u64 {
		<Test as Config>::Currency::balance_on_hold(reason.into(), who)
	}
	pub fn get_contract(addr: &AccountIdOf<Test>) -> ContractInfo<Test> {
		get_contract_checked(addr).unwrap()
	}
	pub fn get_contract_checked(addr: &AccountIdOf<Test>) -> Option<ContractInfo<Test>> {
		ContractInfoOf::<Test>::get(addr)
	}
	pub fn get_code_deposit(code_hash: &CodeHash<Test>) -> BalanceOf<Test> {
		crate::CodeInfoOf::<Test>::get(code_hash).unwrap().deposit()
	}
	pub fn contract_info_storage_deposit(
		addr: &<Test as frame_system::Config>::AccountId,
	) -> BalanceOf<Test> {
		let contract_info = self::get_contract(&addr);
		let info_size = contract_info.encoded_size() as u64;
		DepositPerByte::get()
			.saturating_mul(info_size)
			.saturating_add(DepositPerItem::get())
	}
	pub fn hash<S: Encode>(s: &S) -> <<Test as SysConfig>::Hashing as Hash>::Output {
		<<Test as SysConfig>::Hashing as Hash>::hash_of(s)
	}
	pub fn expected_deposit(code_len: usize) -> u64 {
		// For code_info, the deposit for max_encoded_len is taken.
		let code_info_len = CodeInfo::<Test>::max_encoded_len() as u64;
		// Calculate deposit to be reserved.
		// We add 2 storage items: one for code, other for code_info
		DepositPerByte::get().saturating_mul(code_len as u64 + code_info_len) +
			DepositPerItem::get().saturating_mul(2)
	}
	pub fn ensure_stored(code_hash: CodeHash<Test>) -> usize {
		// Assert that code_info is stored
		assert!(CodeInfoOf::<Test>::contains_key(&code_hash));
		// Assert that contract code is stored, and get its size.
		PristineCode::<Test>::try_get(&code_hash).unwrap().len()
	}
}

impl Test {
	pub fn set_unstable_interface(unstable_interface: bool) {
		UNSTABLE_INTERFACE.with(|v| *v.borrow_mut() = unstable_interface);
	}
}

parameter_types! {
	static TestExtensionTestValue: TestExtension = Default::default();
}

#[derive(Clone)]
pub struct TestExtension {
	enabled: bool,
	last_seen_buffer: Vec<u8>,
	last_seen_input_len: u32,
}

#[derive(Default)]
pub struct RevertingExtension;

#[derive(Default)]
pub struct DisabledExtension;

#[derive(Default)]
pub struct TempStorageExtension {
	storage: u32,
}

impl TestExtension {
	fn disable() {
		TestExtensionTestValue::mutate(|e| e.enabled = false)
	}

	fn last_seen_buffer() -> Vec<u8> {
		TestExtensionTestValue::get().last_seen_buffer.clone()
	}

	fn last_seen_input_len() -> u32 {
		TestExtensionTestValue::get().last_seen_input_len
	}
}

impl Default for TestExtension {
	fn default() -> Self {
		Self { enabled: true, last_seen_buffer: vec![], last_seen_input_len: 0 }
	}
}

impl ChainExtension<Test> for TestExtension {
	fn call<E>(&mut self, env: Environment<E, InitState>) -> ExtensionResult<RetVal>
	where
		E: Ext<T = Test>,
	{
		let func_id = env.func_id();
		let id = env.ext_id() as u32 | func_id as u32;
		match func_id {
			0 => {
				let mut env = env.buf_in_buf_out();
				let input = env.read(8)?;
				env.write(&input, false, None)?;
				TestExtensionTestValue::mutate(|e| e.last_seen_buffer = input);
				Ok(RetVal::Converging(id))
			},
			1 => {
				let env = env.only_in();
				TestExtensionTestValue::mutate(|e| e.last_seen_input_len = env.val1());
				Ok(RetVal::Converging(id))
			},
			2 => {
				let mut env = env.buf_in_buf_out();
				let mut enc = &env.read(9)?[4..8];
				let weight = Weight::from_parts(
					u32::decode(&mut enc).map_err(|_| Error::<Test>::ContractTrapped)?.into(),
					0,
				);
				env.charge_weight(weight)?;
				Ok(RetVal::Converging(id))
			},
			3 => Ok(RetVal::Diverging { flags: ReturnFlags::REVERT, data: vec![42, 99] }),
			_ => {
				panic!("Passed unknown id to test chain extension: {}", func_id);
			},
		}
	}

	fn enabled() -> bool {
		TestExtensionTestValue::get().enabled
	}
}

impl RegisteredChainExtension<Test> for TestExtension {
	const ID: u16 = 0;
}

impl ChainExtension<Test> for RevertingExtension {
	fn call<E>(&mut self, _env: Environment<E, InitState>) -> ExtensionResult<RetVal>
	where
		E: Ext<T = Test>,
	{
		Ok(RetVal::Diverging { flags: ReturnFlags::REVERT, data: vec![0x4B, 0x1D] })
	}

	fn enabled() -> bool {
		TestExtensionTestValue::get().enabled
	}
}

impl RegisteredChainExtension<Test> for RevertingExtension {
	const ID: u16 = 1;
}

impl ChainExtension<Test> for DisabledExtension {
	fn call<E>(&mut self, _env: Environment<E, InitState>) -> ExtensionResult<RetVal>
	where
		E: Ext<T = Test>,
	{
		panic!("Disabled chain extensions are never called")
	}

	fn enabled() -> bool {
		false
	}
}

impl RegisteredChainExtension<Test> for DisabledExtension {
	const ID: u16 = 2;
}

impl ChainExtension<Test> for TempStorageExtension {
	fn call<E>(&mut self, env: Environment<E, InitState>) -> ExtensionResult<RetVal>
	where
		E: Ext<T = Test>,
	{
		let func_id = env.func_id();
		match func_id {
			0 => self.storage = 42,
			1 => assert_eq!(self.storage, 42, "Storage is preserved inside the same call."),
			2 => {
				assert_eq!(self.storage, 0, "Storage is different for different calls.");
				self.storage = 99;
			},
			3 => assert_eq!(self.storage, 99, "Storage is preserved inside the same call."),
			_ => {
				panic!("Passed unknown id to test chain extension: {}", func_id);
			},
		}
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		TestExtensionTestValue::get().enabled
	}
}

impl RegisteredChainExtension<Test> for TempStorageExtension {
	const ID: u16 = 3;
}

parameter_types! {
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(
			Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
		);
	pub static ExistentialDeposit: u64 = 1;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = pallet_balances::AccountData<u64>;
}
impl pallet_insecure_randomness_collective_flip::Config for Test {}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<1>;
	type WeightInfo = ();
}
impl pallet_utility::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

impl pallet_proxy::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = ();
	type ProxyDepositBase = ConstU64<1>;
	type ProxyDepositFactor = ConstU64<1>;
	type MaxProxies = ConstU32<32>;
	type WeightInfo = ();
	type MaxPending = ConstU32<32>;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = ConstU64<1>;
	type AnnouncementDepositFactor = ConstU64<1>;
}

impl pallet_dummy::Config for Test {}

parameter_types! {
	pub MySchedule: Schedule<Test> = {
		let schedule = <Schedule<Test>>::default();
		schedule
	};
	pub static DepositPerByte: BalanceOf<Test> = 1;
	pub const DepositPerItem: BalanceOf<Test> = 2;
	pub static MaxDelegateDependencies: u32 = 32;

	pub static CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(0);
	// We need this one set high enough for running benchmarks.
	pub static DefaultDepositLimit: BalanceOf<Test> = 10_000_000;
}

impl Convert<Weight, BalanceOf<Self>> for Test {
	fn convert(w: Weight) -> BalanceOf<Self> {
		w.ref_time()
	}
}

/// A filter whose filter function can be swapped at runtime.
pub struct TestFilter;

#[derive(Clone)]
pub struct Filters {
	filter: fn(&RuntimeCall) -> bool,
}

impl Default for Filters {
	fn default() -> Self {
		Filters { filter: (|_| true) }
	}
}

parameter_types! {
	static CallFilter: Filters = Default::default();
}

impl TestFilter {
	pub fn set_filter(filter: fn(&RuntimeCall) -> bool) {
		CallFilter::mutate(|fltr| fltr.filter = filter);
	}
}

impl Contains<RuntimeCall> for TestFilter {
	fn contains(call: &RuntimeCall) -> bool {
		(CallFilter::get().filter)(call)
	}
}

parameter_types! {
	pub static UploadAccount: Option<<Test as frame_system::Config>::AccountId> = None;
	pub static InstantiateAccount: Option<<Test as frame_system::Config>::AccountId> = None;
}

pub struct EnsureAccount<T, A>(sp_std::marker::PhantomData<(T, A)>);
impl<T: Config, A: sp_core::Get<Option<crate::AccountIdOf<T>>>>
	EnsureOrigin<<T as frame_system::Config>::RuntimeOrigin> for EnsureAccount<T, A>
where
	<T as frame_system::Config>::AccountId: From<AccountId32>,
{
	type Success = T::AccountId;

	fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
		let who = <frame_system::EnsureSigned<_> as EnsureOrigin<_>>::try_origin(o.clone())?;
		if matches!(A::get(), Some(a) if who != a) {
			return Err(o)
		}

		Ok(who)
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<T::RuntimeOrigin, ()> {
		Err(())
	}
}
parameter_types! {
	pub static UnstableInterface: bool = true;
}

impl Config for Test {
	type Time = Timestamp;
	type Randomness = Randomness;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type CallFilter = TestFilter;
	type CallStack = [Frame<Self>; 5];
	type WeightPrice = Self;
	type WeightInfo = ();
	type ChainExtension =
		(TestExtension, DisabledExtension, RevertingExtension, TempStorageExtension);
	type Schedule = MySchedule;
	type DepositPerByte = DepositPerByte;
	type DepositPerItem = DepositPerItem;
	type DefaultDepositLimit = DefaultDepositLimit;
	type AddressGenerator = DefaultAddressGenerator;
	type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
	type MaxStorageKeyLen = ConstU32<128>;
	type UnsafeUnstableInterface = UnstableInterface;
	type UploadOrigin = EnsureAccount<Self, UploadAccount>;
	type InstantiateOrigin = EnsureAccount<Self, InstantiateAccount>;
	type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
	type RuntimeHoldReason = RuntimeHoldReason;
	type Migrations = crate::migration::codegen::BenchMigrations;
	type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
	type MaxDelegateDependencies = MaxDelegateDependencies;
	type Debug = TestDebug;
	type Environment = ();
	type ApiVersion = ();
	type Xcm = ();
}

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([2u8; 32]);
pub const CHARLIE: AccountId32 = AccountId32::new([3u8; 32]);
pub const DJANGO: AccountId32 = AccountId32::new([4u8; 32]);

pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

pub struct ExtBuilder {
	existential_deposit: u64,
	storage_version: Option<StorageVersion>,
	code_hashes: Vec<CodeHash<Test>>,
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
	pub fn with_code_hashes(mut self, code_hashes: Vec<CodeHash<Test>>) -> Self {
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
		let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
		pallet_balances::GenesisConfig::<Test> { balances: vec![] }
			.assimilate_storage(&mut t)
			.unwrap();
		let mut ext = sp_io::TestExternalities::new(t);
		ext.register_extension(KeystoreExt::new(MemoryKeystore::new()));
		ext.execute_with(|| {
			use frame_support::traits::OnGenesis;

			Pallet::<Test>::on_genesis();
			if let Some(storage_version) = self.storage_version {
				storage_version.put::<Pallet<Test>>();
			}
			System::set_block_number(1)
		});
		ext.execute_with(|| {
			for code_hash in self.code_hashes {
				CodeInfoOf::<Test>::insert(code_hash, crate::CodeInfo::new(ALICE));
			}
		});
		ext
	}
}

fn initialize_block(number: u64) {
	System::reset_events();
	System::initialize(&number, &[0u8; 32].into(), &Default::default());
}

struct ExtensionInput<'a> {
	extension_id: u16,
	func_id: u16,
	extra: &'a [u8],
}

impl<'a> ExtensionInput<'a> {
	fn to_vec(&self) -> Vec<u8> {
		((self.extension_id as u32) << 16 | (self.func_id as u32))
			.to_le_bytes()
			.iter()
			.chain(self.extra)
			.cloned()
			.collect()
	}
}

impl<'a> From<ExtensionInput<'a>> for Vec<u8> {
	fn from(input: ExtensionInput) -> Vec<u8> {
		input.to_vec()
	}
}

impl Default for Origin<Test> {
	fn default() -> Self {
		Self::Signed(ALICE)
	}
}