use std::fs::File;
use std::io::Write;
use std::{fs, path::Path, sync::Mutex};

use contract_transcode::ContractMessageTranscoder;
use frame_support::__private::BasicExternalities;
use itertools::Itertools;

use seq_macro::seq;

use crate::contract::payload::{PayloadCrafter, Selector};
use crate::contract::remote::ContractBridge;
use crate::fuzzer::engine::FuzzerEngine;
use crate::fuzzer::invariants::Invariants;
use crate::utils;

#[derive(Clone)]
pub struct Fuzzer {
    setup: ContractBridge,
}

impl Fuzzer {
    pub fn new(setup: ContractBridge) -> Fuzzer {
        Fuzzer { setup }
    }


    fn make_initial_corpus(selectors: &mut [Selector]) -> Result<(), std::io::Error> {
        fs::create_dir_all("./output/phink/corpus")?;

        for (i, selector) in selectors.iter().enumerate() {
            let file_path = format!("{}/selector_{}.bin", "./output/phink/corpus", i);
            let mut file = File::create(&file_path)?;
            file.write_all(selector)?;
        }

        Ok(())
    }
}

impl FuzzerEngine for Fuzzer {
    fn setup(self) {
        let mut transcoder_loader = Mutex::new(
            ContractMessageTranscoder::load(Path::new(&self.setup.path_to_specs)).unwrap(),
        );

        let specs = &self.setup.json_specs;
        let mut selectors: Vec<Selector> = PayloadCrafter::extract_all(specs);
        let inv = PayloadCrafter::extract_invariants(specs)
            .expect("No invariants found, check your contract");

        let mut invariant_manager = Invariants::from(inv, self.setup.clone());

        Self::make_initial_corpus(&mut selectors).expect("Failed to create initial corpus");

        ziggy::fuzz!(|data: &[u8]| {
            harness(
                self.clone(),
                &mut transcoder_loader,
                &mut selectors.clone(),
                &mut invariant_manager,
                data,
            );
        });
    }
}

fn harness(
    client: Fuzzer,
    transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
    selectors: &mut Vec<Selector>,
    invariant_manager: &mut Invariants,
    input: &[u8],
) {
    let binding = client.clone();
    let mut raw_call = binding.parse_args(input, selectors.clone());

    if raw_call.is_none() {
        return;
    }

    raw_call = Some(raw_call.unwrap());


    // let decoded_msg = raw_call.decode();

     let decoded_msg = transcoder_loader
         .lock()
         .unwrap()
         .decode_contract_message(&mut &*raw_call.clone().unwrap());

    if let Err(_) = decoded_msg {
        return;
    }

    let mut chain = BasicExternalities::new(client.setup.genesis.clone());
    chain.execute_with(|| <Fuzzer as FuzzerEngine>::timestamp());

    chain.execute_with(|| {
        let mut results: Vec<Vec<u8>> = Vec::new();
        // for call in calls {

        let result = client.setup.clone().call(&raw_call.clone().unwrap(), 1, 0);
        results.push(result.debug_message);

        #[cfg(not(fuzzing))]
        <Fuzzer as FuzzerEngine>::pretty_print(
            result.result.clone(),
            decoded_msg.unwrap().to_string(),
            raw_call.unwrap(),
        );

        let flatten_cov: &[u8] = &*results.into_iter().flatten().collect::<Vec<_>>();
        let mut coverage_str = utils::deduplicate(&*String::from_utf8_lossy(flatten_cov));

        seq!(x in 0..=500 {
           if coverage_str.contains(&format!("COV={}", x)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", x);
                            let _ = 1 + 1;

        }
        });

        // For each call, we verify that invariants aren't broken
        if !invariant_manager.are_invariants_passing() {
            panic!("Invariant triggered!");
        }
    })
}
