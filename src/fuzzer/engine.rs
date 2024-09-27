use crate::{
    contract::{
        remote::FullContractResponse,
        runtime::{
            AllPalletsWithSystem,
            BlockNumber,
            RuntimeOrigin,
            Timestamp,
            SLOT_DURATION,
        },
    },
    fuzzer::parser::OneInput,
};
use frame_support::traits::{
    OnFinalize,
    OnInitialize,
};
use prettytable::{
    Cell,
    Row,
    Table,
};
use std::println;

/// Pretty print the result of `OneInput`
#[allow(dead_code)]
pub fn pretty_print(responses: Vec<FullContractResponse>, one_input: OneInput) {
    println!("\n🌱 Executing new seed");
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Message"), Cell::new("Details")]));

    for (response, message) in responses.iter().zip(&one_input.messages) {
        let call_description = message.message_metadata.to_string();
        let debug = message.display_with_reply(response.get());

        table.add_row(Row::new(vec![
            Cell::new(&call_description),
            Cell::new(&debug),
        ]));
    }

    table.printstd();
}

/// We need to instantiate a proper timestamp on each call
pub fn timestamp(lapse: u32) {
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
