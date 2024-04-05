use frame::deps::{frame_support, frame_system, sp_runtime};
use frame::runtime::testing_prelude::RuntimeVersion;
use frame::traits::IdentityLookup;

use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature,
};

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them.
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = frame::deps::sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem;
/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

pub type Address = sp_runtime::MultiAddress<AccountId, ()>;

pub type UncheckedExtrinsic =
generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

pub type Block = generic::Block<Header, UncheckedExtrinsic>;

type Migrations = ();

/// Block ID.
pub type BlockId = generic::BlockId<Block>;

pub type RuntimeExecutive = frame::deps::frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrations,
>;

/*
pub type RuntimeExecutive =
    frame::runtime::testing_prelude::Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;
*/

use frame::arithmetic::Perbill;
use frame::deps::sp_runtime::{create_runtime_str, traits::Bounded, FixedPointNumber, Perquintill};
use frame::deps::sp_std::prelude::*;
use frame::deps::sp_version::NativeVersion;
use sp_core::ConstBool;

use frame_support::{
    construct_runtime, derive_impl,
    instances::Instance1,
    parameter_types,
    traits::{AsEnsureOriginWithArg, ConstU128, ConstU16, ConstU32},
    weights::{constants::RocksDbWeight, ConstantMultiplier, IdentityFee},
};
use frame_system::{EnsureRoot, EnsureSigned};
pub use pallet_transaction_payment::{CurrencyAdapter, Multiplier, TargetedFeeAdjustment};

pub const MILLICENTS: Balance = 1_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;
// assume this is worth about a cent.
pub const DOLLARS: Balance = 100 * CENTS;

pub const MILLISECS_PER_BLOCK: Moment = 3000;
pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

// 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
//       Attempting to do so will brick block production.
pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 10 * MINUTES;
pub const EPOCH_DURATION_IN_SLOTS: u64 = {
    const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

    (EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
};

// These time units are defined in number of blocks.
pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// Runtime version.
#[frame::deps::sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("node"),
    impl_name: create_runtime_str!("substrate-node"),
    authoring_version: 10,
    // Per convention: if the runtime behavior changes, increment spec_version
    // and set impl_version to 0. If only runtime
    // implementation changes and behavior does not, then leave spec_version as
    // is and increment impl_version.
    spec_version: 268,
    impl_version: 0,
    apis: frame::deps::sp_version::create_apis_vec!([]),
    transaction_version: 2,
    state_version: 1,
};

/// Native version.
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
pub const BlockHashCount: BlockNumber = 100;
}

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
    type BaseCallFilter = frame::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = RocksDbWeight;
    type Nonce = Nonce;
    type Hash = Hash;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type AccountData = pallet_balances::AccountData<Balance>;
    type SystemWeightInfo = frame_system::weights::SubstrateWeight<Runtime>;
    type SS58Prefix = ConstU16<42>;
    type MaxConsumers = ConstU32<16>;
}

parameter_types! {
         pub static ExistentialDeposit: u64 = 1;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Pallet<Runtime>;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
    type FreezeIdentifier = RuntimeFreezeReason;
    type MaxFreezes = ConstU32<1>;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 10 * MILLICENTS;
    pub const OperationalFeeMultiplier: u8 = 5;
    pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
    pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 100_000);
    pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128);
    pub MaximumMultiplier: Multiplier = Bounded::max_value();
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = TargetedFeeAdjustment<
        Self,
        TargetBlockFullness,
        AdjustmentVariable,
        MinimumMultiplier,
        MaximumMultiplier,
    >;
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
}

parameter_types! {
    pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    type Moment = Moment;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = pallet_timestamp::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const AssetDeposit: Balance = 100 * DOLLARS;
    pub const ApprovalDeposit: Balance = DOLLARS;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 10 * DOLLARS;
    pub const MetadataDepositPerByte: Balance = DOLLARS;
}

impl pallet_assets::Config<Instance1> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u128;
    type AssetId = u32;
    type AssetIdParameter = parity_scale_codec::Compact<u32>;
    type Currency = Balances;
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = ConstU128<DOLLARS>;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type Freezer = ();
    type Extra = ();
    type CallbackHandle = ();
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
    type RemoveItemsLimit = ConstU32<1000>;
}

pub const AST: Balance = 1_000 * 1_000 * 1_000_000_000_000;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 1 * AST + (bytes as Balance) * 100 * 1_000_000_000_000
}

impl pallet_insecure_randomness_collective_flip::Config for Runtime {}
parameter_types! {
    pub const DepositPerItem: Balance = deposit(1, 0);
    pub const DepositPerByte: Balance = deposit(0, 1);
    // Fallback value if storage deposit limit not set by the user
    pub const DefaultDepositLimit: Balance = deposit(16, 16 * 1024);
    pub const MaxDelegateDependencies: u32 = 32;
    pub const CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(10);
    pub Schedule: pallet_contracts::Schedule<Runtime> = Default::default();
}
impl pallet_contracts::Config for Runtime {
    type Time = Timestamp;
    type Randomness = Randomness;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    /// The safest default is to allow no calls at all.
    ///
    /// Runtimes should whitelist dispatchables that are allowed to be called from contracts
    /// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
    /// change because that would break already deployed contracts. The `Call` structure itself
    /// is not allowed to change the indices of existing pallets, too.
    type CallFilter = frame_support::traits::Nothing;
    type DepositPerItem = DepositPerItem;
    type DepositPerByte = DepositPerByte;
    type DefaultDepositLimit = DefaultDepositLimit;
    type CallStack = [pallet_contracts::Frame<Self>; 5];
    type WeightPrice = pallet_transaction_payment::Pallet<Self>;
    type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
    // type ChainExtension = LocalChainExtensions<Self, UnifiedAccounts, Xvm>;
    type ChainExtension = ();
    type Schedule = Schedule;
    type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
    type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
    type MaxStorageKeyLen = ConstU32<128>;
    type UnsafeUnstableInterface = ConstBool<true>;
    type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
    type MaxDelegateDependencies = MaxDelegateDependencies;
    type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
    type RuntimeHoldReason = RuntimeHoldReason;
    type Debug = ();
    type Environment = ();
    type Migrations = ();
    type ApiVersion = ();
    type Xcm = ();
}

construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,

        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Randomness: pallet_insecure_randomness_collective_flip,

        Assets: pallet_assets::<Instance1>,
        Contracts: pallet_contracts
    }
);
