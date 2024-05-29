use contract_transcode::ContractMessageTranscoder;
use core::panic;
use frame_support::__private::BasicExternalities;
use frame_support::traits::{OnFinalize, OnInitialize};
use pallet_contracts::ExecReturnValue;
use prettytable::{row, Table};
use sp_runtime::DispatchError;
use std::format;
use std::sync::Mutex;

use crate::contract::payload::Selector;
use crate::contract::runtime::{
    AllPalletsWithSystem, BlockNumber, RuntimeOrigin, Timestamp, SLOT_DURATION,
};
use crate::fuzzer::fuzz::Fuzzer;
use crate::fuzzer::invariants::Invariants;
use crate::fuzzer::parser::{parse_input, OneInput};
use crate::utils;

pub trait FuzzerEngine {
    fn fuzz(self);
    fn redirect_coverage(coverages_vec: Vec<Vec<u8>>);
    fn harness(
        client: Fuzzer,
        transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
        _selectors: &mut Vec<Selector>,
        bug_manager: &mut Invariants,
        input: &[u8],
    );

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
                hex::encode(&message.payload),
                result
            ]);
        }
        table.printstd();
    }

    /// We need to instantiate a proper timestamp on each call
    fn timestamp(lapse: u32) {
        let mut block: u32 = 1;
        Timestamp::set(
            RuntimeOrigin::none(),
            (block as u64).saturating_mul(SLOT_DURATION),
        )
        .unwrap();
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
