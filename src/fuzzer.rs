use crate::deploy::DeployedSetup;
use crate::payload;
use crate::payload::{PayloadCrafter, Selector};
use crate::runtime::{AllPalletsWithSystem, BlockNumber, RuntimeOrigin, Timestamp, SLOT_DURATION};
use contract_transcode::ContractMessageTranscoder;
use frame_support::__private::BasicExternalities;
use frame_support::traits::{OnFinalize, OnInitialize};
use pallet_contracts::{DebugInfo, Determinism};
use parity_scale_codec::Encode;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ContractFuzzer {
    setup: DeployedSetup,
    payload: PayloadCrafter,
}

impl ContractFuzzer {
    pub fn new(setup: DeployedSetup) -> ContractFuzzer {
        Self {
            setup,
            payload: Default::default(),
        }
    }

    /// Return the scale_encoded version of a call
    pub fn create_call(func: Selector, args: &[u8]) -> Option<Vec<u8>> {
        Some((func, args.to_vec()).encode())
    }

    fn parse_args(self, data: &[u8]) -> Option<Box<(Selector, &[u8])>> {
        // Our payload must be at least `1_000` sized, and min `4`
        // TODO! Figure out what's the minimum size
        if data.len() > 1_000 || data.len() < 4 {
            return None;
        }

        let selectors: Vec<Selector> = PayloadCrafter::extract(self.setup.json_specs);

        // 4 bytes are allocated to the selector fuzz
        if let selector_slice =
            u32::from_ne_bytes(data[0..4].try_into().unwrap()) as usize >= selectors.len()
        {
            let fuzzed_func = selectors[selector_slice as usize];
            let arguments = &data[4..];
         return   Some(Box::new((fuzzed_func, arguments)))
        }
        None
    }

    pub fn fuzz(self) {
        let transcoder_loader = Arc::new(Mutex::new(
            ContractMessageTranscoder::load(Path::new("sample/dns/target/ink/dns.json")).unwrap(),
        ));

        ziggy::fuzz!(|data: &[u8]| {
            let raw_call = self.clone().parse_args(data);
            if raw_call.is_none() {
                return;
            }

            match ContractFuzzer::create_call(raw_call.clone().unwrap().0, raw_call.clone().unwrap().1) {
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
                        //TODO!
                        // let result = extrinsics::bare_call(self.setup.contract.clone())
                        //     .debug(DebugInfo::UnsafeDebug)
                        //     .determinism(Determinism::Relaxed)
                        //     .data(full_call.clone())
                        //     .build();

                        // check_invariants();

                        #[cfg(not(fuzzing))]
                        {
                            println!(
                                "{}          {}",
                                decoded_msg.unwrap().to_string(),
                                hex::encode(full_call.clone())
                            );
                            // println!("{result:?}\n");
                        }
                    });
                }

                // Encoding failed, we try agian
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
