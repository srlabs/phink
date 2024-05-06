use crate::contract::payload::{PayloadCrafter, Selector};
use crate::contract::remote::ContractBridge;
use crate::contract::runtime::{
    AllPalletsWithSystem, BlockNumber, RuntimeOrigin, Timestamp, SLOT_DURATION,
};
use crate::fuzzer::invariants::Invariants;
use contract_transcode::ContractMessageTranscoder;
use frame_support::__private::BasicExternalities;
use frame_support::traits::{OnFinalize, OnInitialize};
use pallet_contracts::ExecReturnValue;
use parity_scale_codec::Encode;
use prettytable::{row, Table};
use sp_runtime::DispatchError;
use std::path::Path;
use std::sync::Mutex;

pub trait FuzzerEngine {
    fn fuzz(self);

    /// Return the scale_encoded version of a call
    fn create_call(func: Selector, args: &[u8]) -> Option<Vec<u8>> {
        Some((func, args.to_vec()).encode())
    }

    /// Accept some raw bytes `[u8]` and return the appropriate
    fn parse_args<'a>(
        &'a self,
        data: &'a [u8],
        selectors: Vec<Selector>,
    ) -> Option<Box<(Selector, &[u8])>> {
        // TODO! 1500 shouldn't be static
        // Our payload must be at least `1_500` sized, and min `4`
        let selector_size = 4; //where 4 is *at least* the selector size
        if data.len() > 1_500 || data.len() <= selector_size {
            return None;
        }
        let selector_slice: usize =
            u32::from_ne_bytes(data[0..selector_size].try_into().unwrap()) as usize;
        if selector_slice < selectors.len() {
            let fuzzed_func = selectors[selector_slice];
            let arguments = &data[selector_size..];
            return Some(Box::new((fuzzed_func, arguments)));
        }
        None
    }

    /// Pretty print the result of a call
    /// Used for debug purposed... (yes, we still make it fancy :/)
    fn pretty_print(
        result: Result<ExecReturnValue, DispatchError>,
        decoded_msg: String,
        full_call: Vec<u8>,
    ) {
        let mut table = Table::new();
        let result: String = format!("{:?}", result.unwrap());
        table.add_row(row!["Decoded call", "Encoded call", "Result"]);
        table.add_row(row![decoded_msg, hex::encode(full_call), result]);
        table.printstd();
    }

    /// We need to instantiate a proper timestamp on each call
    /// TODO! Lapse should be fuzzed, so if the contract depends on a block number,
    /// TODO! the fuzzer will correctly find the block
    fn timestamp() {
        let mut block: u32 = 1;
        Timestamp::set(RuntimeOrigin::none(), block as u64 * SLOT_DURATION).unwrap();
        let lapse: u32 = 0; //for now, we set lapse always to zero
        if lapse > 0 {
            <AllPalletsWithSystem as OnFinalize<BlockNumber>>::on_finalize(block);
            block += u32::from(lapse);
            <AllPalletsWithSystem as OnInitialize<BlockNumber>>::on_initialize(block);
            Timestamp::set(RuntimeOrigin::none(), SLOT_DURATION * block as u64).unwrap();
        }
    }
}
