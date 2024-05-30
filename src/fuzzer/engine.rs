use std::format;
use std::sync::Mutex;

use contract_transcode::ContractMessageTranscoder;
use frame_support::traits::{OnFinalize, OnInitialize};
use prettytable::{row, Table};

use crate::{
    contract::payload::Selector,
    contract::remote::FullContractResponse,
    contract::runtime::{
        AllPalletsWithSystem, BlockNumber, RuntimeOrigin, Timestamp, SLOT_DURATION,
    },
    fuzzer::fuzz::Fuzzer,
    fuzzer::invariants::BugManager,
    fuzzer::parser::OneInput,
};

pub trait FuzzerEngine {
    fn fuzz(self);
    fn harness(
        client: Fuzzer,
        transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
        _selectors: &mut Vec<Selector>,
        bug_manager: &mut BugManager,
        input: &[u8],
    );

    /// Pretty print the result of `OneInput`
    fn pretty_print(results: Vec<FullContractResponse>, decoded_msg: OneInput) {
        assert_eq!(results.len(), decoded_msg.messages.len());

        println!("\n\n🌱  Executing new seed");
        let mut table = Table::new();
        table.add_row(row!["Message", "Result", "Debug"]);

        for i in 0..results.len() {
            let curr_result = results.get(i).unwrap();

            let result: String = format!("{:?}", curr_result.result.clone().unwrap());
            let description = decoded_msg
                .messages
                .get(i)
                .clone()
                .unwrap()
                .message_metadata
                .to_string();
            let debug = &curr_result.debug_message;

            table.add_row(row![
                description,
                result,
                format!("{}", String::from_utf8_lossy(&*debug).replace('\n', " "))
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
