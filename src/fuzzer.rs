use crate::{
    deploy::ContractBridge,
    invariants,
    payload::{PayloadCrafter, Selector},
    runtime::{AllPalletsWithSystem, BlockNumber, RuntimeOrigin, Timestamp, SLOT_DURATION},
};

use prettytable::{row, Cell, Row, Table};

use contract_transcode::ContractMessageTranscoder;
use frame_support::{
    __private::BasicExternalities,
    traits::{OnFinalize, OnInitialize},
};

use crate::invariants::Invariants;
use anyhow::Context;
use ink_metadata::InkProject;
use parity_scale_codec::Encode;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct ContractFuzzer {
    setup: ContractBridge,
}

impl ContractFuzzer {
    pub fn new(setup: ContractBridge) -> ContractFuzzer {
        Self { setup }
    }

    /// Return the scale_encoded version of a call
    pub fn create_call(func: Selector, args: &[u8]) -> Option<Vec<u8>> {
        Some((func, args.to_vec()).encode())
    }

    /// Accept some raw bytes `[u8]` and return the appropriate
    fn parse_args(self, data: &[u8], selectors: Vec<Selector>) -> Option<Box<(Selector, &[u8])>> {
        // Our payload must be at least `1_500` sized, and min `4`
        if data.len() > 1_500 || data.len() <= 4 {
            return None;
        }
        // 4 bytes are allocated to the selector fuzz
        let selector_slice: usize = u32::from_ne_bytes(data[0..4].try_into().unwrap()) as usize;
        if selector_slice < selectors.len() {
            let fuzzed_func = selectors[selector_slice];
            let arguments = &data[4..];
            return Some(Box::new((fuzzed_func, arguments)));
        }
        None
    }
    #[warn(unused_variables)]
    pub fn fuzz(self) {
        let transcoder_loader = Arc::new(Mutex::new(
            //TODO! That's not supposed to be hardcoded
            ContractMessageTranscoder::load(Path::new("sample/dns/target/ink/dns.json")).unwrap(),
        ));

        let selectors: Vec<Selector> = PayloadCrafter::extract_all(&self.setup.json_specs);
        // let invariant_manager: Invariants =
        //     Invariants::from(PayloadCrafter::extract_invariants(&self.setup.json_specs));

        ziggy::fuzz!(|data: &[u8]| {
            let raw_call = self.clone().parse_args(data, selectors.clone());
            if raw_call.is_none() {
                return;
            }
            let call = raw_call.expect("`raw_call` wasn't None; QED");
            match ContractFuzzer::create_call(call.0, call.1) {
                // Successfully encoded
                Some(full_call) => {
                    let decoded_msg = transcoder_loader
                        .lock()
                        .unwrap()
                        .decode_contract_message(&mut &*full_call);
                    if let Err(_) = decoded_msg {
                        return;
                    }
                    let mut chain = BasicExternalities::new(self.setup.genesis.clone());
                    chain.execute_with(|| {
                        timestamp();
                        let result = self.setup.clone().call(&full_call);

                        // For each call, we verify that invariants aren't broken
                        // if !&invariant_manager.are_invariants_passing() {
                        //     panic!("Invariant triggered -- oupsi")
                        // }

                        // We pretty-print all information that we need to debug
                        #[cfg(not(fuzzing))]
                        {
                            let mut table = Table::new();
                            let result: String = format!("{:?}", result.unwrap());
                            table.add_row(row!["Decoded call", "Encoded call", "Result"]);
                            table.add_row(row![
                                decoded_msg.unwrap().to_string(),
                                hex::encode(full_call),
                                result
                            ]);
                            table.printstd();
                        }
                    });
                }

                // Encoding failed, we try again
                None => return,
            }
        });
    }
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
