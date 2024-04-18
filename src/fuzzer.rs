use crate::deploy::DeployedSetup;
use crate::selectors;
use crate::selectors::{PayloadCrafter, Selector};
use contract_transcode::ContractMessageTranscoder;
use frame_support::__private::BasicExternalities;
use pallet_contracts::{DebugInfo, Determinism};
use std::path::Path;
use std::sync::{Arc, Mutex};

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
    pub fn fuzz(self) {
        let selectors: Vec<Selector> = self.payload.extract_all(self.setup.json_specs);
        let transcoder_loader = Arc::new(Mutex::new(
            ContractMessageTranscoder::load(Path::new("sample/dns/target/ink/dns.json")).unwrap(),
        ));

        ziggy::fuzz!(|data: &[u8]| {
            if data.len() > 1_000 || data.len() < 4 {
                return;
            }

            match create_call(&data, &selectors) {
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
                        // timestamp();
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
                                "{} ......... {}",
                                decoded_msg.unwrap().to_string(),
                                hex::encode(full_call.clone())
                            );
                            // println!("{result:?}\n");
                        }
                    });
                }
                None => return,
            }
        });
    }
}
