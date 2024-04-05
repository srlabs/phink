#![recursion_limit = "1024"]

use frame::prelude::Weight;
use pallet_contracts::{Code, CollectEvents, DebugInfo};
use sp_core::crypto::AccountId32;
use crate::mocks::{Balances, Contracts};
use frame::traits::Currency;

mod mocks;


pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);
pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

fn main() {

}