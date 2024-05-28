use frame_support::traits::{OnFinalize, OnInitialize};
use pallet_contracts::ExecReturnValue;
use prettytable::{row, Table};
use sp_runtime::DispatchError;

use crate::contract::payload::Selector;
use crate::contract::runtime::{
    AllPalletsWithSystem, BlockNumber, RuntimeOrigin, Timestamp, SLOT_DURATION,
};
use crate::fuzzer::parser::OneInput;

pub trait FuzzerEngine {
    fn fuzz(self);

    /// Takes some raw bytes `[u8]` and returns the good code
    #[deprecated]
    fn parse_args<'a>(&'a self, data: &'a [u8], selectors: Vec<Selector>) -> Option<Vec<u8>> {
        // TODO! 1500 shouldn't be static
        // Our payload must be at least `1_500` sized, and min `4`
        let selector_size = 4; //where 4 is *at least* the selector size
        if data.len() > 500 || data.len() <= 6 {
            return None;
        } //This is passed to Ziggy

        let selector_slice: usize = u32::from_ne_bytes(data[0..4].try_into().unwrap()) as usize;
        if selector_slice < selectors.len() {
            let fuzzed_func = selectors[selector_slice];

            let arguments = &data[4..];
            let mut result = Vec::with_capacity(fuzzed_func.len() + arguments.len());
            result.extend_from_slice(&fuzzed_func);
            result.extend_from_slice(arguments);
            return Some(result);
        }
        None
    }

    /// Pretty print the result of OneInput
    fn pretty_print(results: Vec<Result<ExecReturnValue, DispatchError>>, decoded_msg: OneInput) {
        assert_eq!(results.len(), decoded_msg.messages.len());
        let mut table = Table::new();
        table.add_row(row!["Description", "SCALE", "Result"]);

        for i in 0..results.len() {
            let result: String = format!("{:?}", results.get(i).unwrap());
            let message = decoded_msg.messages.get(i).clone().unwrap();
            table.add_row(row![
                message.description,
                hex::encode(&message.call),
                result
            ]);
        }
        table.printstd();
    }

    /// We need to instantiate a proper timestamp on each call
    /// TODO! Lapse should be fuzzed, so if the contract depends on a block number,
    /// TODO! the fuzzer will correctly find the block
    fn timestamp() {
        let mut block: u32 = 1;
        Timestamp::set(
            RuntimeOrigin::none(),
            (block as u64).saturating_mul(SLOT_DURATION),
        )
        .unwrap();
        let lapse: u32 = 0; //for now, we set lapse always to zero
        if lapse > 0 {
            <AllPalletsWithSystem as OnFinalize<BlockNumber>>::on_finalize(block);
            block = block.saturating_add(u32::from(lapse));
            <AllPalletsWithSystem as OnInitialize<BlockNumber>>::on_initialize(block);
            Timestamp::set(
                RuntimeOrigin::none(),
                SLOT_DURATION.saturating_mul(block as u64),
            )
            .unwrap();
        }
    }
}
