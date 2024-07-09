use std::sync::Mutex;

use crate::{
    contract::remote::FullContractResponse,
    contract::runtime::{
        AllPalletsWithSystem, BlockNumber, RuntimeOrigin, Timestamp, SLOT_DURATION,
    },
    fuzzer::bug::BugManager,
    fuzzer::fuzz::Fuzzer,
    fuzzer::parser::OneInput,
};
use contract_transcode::ContractMessageTranscoder;
use frame_support::traits::{OnFinalize, OnInitialize};
use pallet_contracts::ContractResult;
use prettytable::{Cell, Row, Table};
use sp_core::crypto::AccountId32;

pub trait FuzzerEngine {
    fn fuzz(self);
    fn harness(
        client: Fuzzer,
        transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
        bug_manager: &mut BugManager,
        input: &[u8],
    );

    /// Pretty print the result of `OneInput`
    fn pretty_print(responses: Vec<FullContractResponse>, one_input: OneInput) {
        println!("\n🌱 Executing new seed\n");
        let mut table = Table::new();
        table.add_row(Row::new(vec![Cell::new("Message"), Cell::new("Details")]));

        for (response, message) in responses.iter().zip(&one_input.messages) {
            let call_description = message.message_metadata.to_string();

            let debug = match response {
                ContractResult {
                    result: _result, ..
                } => format!(
                    "⛽️ Gas required: {}\n\
                 🔥 Gas consumed: {}\n\
                 🧑 Origin: {}({})\n\
                 💾 Storage deposit: {:?}{}",
                    response.gas_required,
                    response.gas_consumed,
                    message.origin,
                    AccountId32::new([message.origin.try_into().unwrap(); 32]).to_string(),
                    response.storage_deposit,
                    if message.is_payable {
                        format!(
                            "\n💸 Message was payable and {} units were transferred",
                            message.value_token
                        )
                    } else {
                        String::new()
                    }
                ),
            };

            table.add_row(Row::new(vec![
                Cell::new(&call_description),
                Cell::new(&debug),
            ]));
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
            block = block.saturating_add(lapse);
            <AllPalletsWithSystem as OnInitialize<BlockNumber>>::on_initialize(block);
            Timestamp::set(
                RuntimeOrigin::none(),
                SLOT_DURATION.saturating_mul(block as u64),
            )
            .unwrap();
        }
    }
    fn exec_seed(self, seed: &[u8]);
}
