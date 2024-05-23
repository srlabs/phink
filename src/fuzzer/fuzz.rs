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
use crate::fuzzer::message::{parse_input, OneInput};
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
    fn fuzz(self) {
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
    // let binding = client.clone();
    // let mut raw_call = binding.parse_args(input, selectors.clone());
    //

    if input.len() > 500 || input.len() <= 6 {
        return;
    }

    let decoded_msgs: OneInput = parse_input(input, transcoder_loader);

    if decoded_msgs.messages.len() == 0 {
        return;
    }

    let mut chain = BasicExternalities::new(client.setup.genesis.clone());
    chain.execute_with(|| <Fuzzer as FuzzerEngine>::timestamp());

    chain.execute_with(|| {
        for decoded_msg in decoded_msgs.messages {
            let mut results: Vec<Vec<u8>> = Vec::new();

            let result = client.setup.clone().call(
                &decoded_msg.call,
                decoded_msg.origin as u8,
                decoded_msg.value_token,
            );
            results.push(result.debug_message);

            #[cfg(not(fuzzing))]
            <Fuzzer as FuzzerEngine>::pretty_print(
                result.result.clone(),
                decoded_msg.description,
                decoded_msg.call,
            );

            let flatten_cov: &[u8] = &*results.into_iter().flatten().collect::<Vec<_>>();
            let mut coverage_str = utils::deduplicate(&*String::from_utf8_lossy(flatten_cov));

            // seq!(x in 0..=300 {
            //    if coverage_str.contains(&format!("COV={}", x)) {
            //         let _ = 1 + 1;
            //         println!("We've passed {:?}", x);
            //         let _ = 1 + 1;
            //     }
            // });
            if coverage_str.contains(&format!("COV={}", 0)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 0);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 1)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 1);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 2)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 2);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 3)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 3);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 4)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 4);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 5)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 5);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 6)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 6);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 7)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 7);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 8)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 8);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 9)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 9);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 10)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 10);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 11)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 11);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 12)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 12);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 13)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 13);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 14)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 14);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 15)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 15);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 16)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 16);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 17)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 17);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 18)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 18);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 19)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 19);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 20)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 20);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 21)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 21);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 22)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 22);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 23)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 23);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 24)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 24);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 25)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 25);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 26)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 26);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 27)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 27);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 28)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 28);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 29)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 29);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 30)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 30);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 31)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 31);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 32)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 32);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 33)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 33);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 34)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 34);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 35)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 35);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 36)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 36);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 37)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 37);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 38)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 38);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 39)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 39);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 40)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 40);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 41)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 41);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 42)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 42);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 43)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 43);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 44)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 44);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 45)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 45);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 46)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 46);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 47)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 47);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 48)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 48);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 49)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 49);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 50)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 50);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 51)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 51);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 52)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 52);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 53)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 53);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 54)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 54);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 55)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 55);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 56)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 56);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 57)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 57);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 58)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 58);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 59)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 59);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 60)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 60);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 61)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 61);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 62)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 62);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 63)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 63);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 64)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 64);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 65)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 65);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 66)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 66);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 67)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 67);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 68)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 68);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 69)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 69);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 70)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 70);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 71)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 71);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 72)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 72);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 73)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 73);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 74)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 74);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 75)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 75);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 76)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 76);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 77)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 77);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 78)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 78);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 79)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 79);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 80)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 80);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 81)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 81);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 82)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 82);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 83)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 83);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 84)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 84);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 85)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 85);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 86)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 86);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 87)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 87);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 88)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 88);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 89)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 89);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 90)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 90);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 91)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 91);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 92)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 92);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 93)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 93);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 94)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 94);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 95)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 95);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 96)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 96);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 97)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 97);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 98)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 98);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 99)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 99);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 100)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 100);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 101)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 101);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 102)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 102);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 103)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 103);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 104)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 104);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 105)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 105);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 106)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 106);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 107)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 107);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 108)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 108);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 109)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 109);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 110)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 110);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 111)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 111);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 112)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 112);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 113)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 113);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 114)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 114);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 115)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 115);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 116)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 116);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 117)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 117);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 118)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 118);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 119)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 119);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 120)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 120);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 121)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 121);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 122)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 122);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 123)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 123);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 124)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 124);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 125)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 125);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 126)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 126);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 127)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 127);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 128)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 128);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 129)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 129);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 130)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 130);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 131)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 131);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 132)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 132);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 133)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 133);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 134)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 134);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 135)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 135);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 136)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 136);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 137)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 137);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 138)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 138);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 139)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 139);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 140)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 140);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 141)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 141);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 142)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 142);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 143)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 143);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 144)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 144);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 145)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 145);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 146)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 146);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 147)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 147);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 148)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 148);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 149)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 149);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 150)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 150);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 151)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 151);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 152)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 152);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 153)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 153);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 154)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 154);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 155)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 155);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 156)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 156);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 157)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 157);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 158)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 158);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 159)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 159);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 160)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 160);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 161)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 161);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 162)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 162);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 163)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 163);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 164)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 164);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 165)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 165);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 166)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 166);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 167)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 167);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 168)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 168);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 169)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 169);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 170)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 170);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 171)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 171);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 172)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 172);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 173)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 173);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 174)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 174);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 175)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 175);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 176)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 176);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 177)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 177);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 178)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 178);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 179)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 179);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 180)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 180);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 181)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 181);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 182)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 182);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 183)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 183);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 184)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 184);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 185)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 185);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 186)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 186);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 187)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 187);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 188)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 188);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 189)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 189);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 190)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 190);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 191)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 191);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 192)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 192);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 193)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 193);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 194)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 194);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 195)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 195);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 196)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 196);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 197)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 197);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 198)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 198);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 199)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 199);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 200)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 200);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 201)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 201);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 202)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 202);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 203)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 203);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 204)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 204);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 205)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 205);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 206)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 206);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 207)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 207);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 208)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 208);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 209)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 209);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 210)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 210);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 211)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 211);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 212)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 212);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 213)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 213);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 214)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 214);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 215)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 215);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 216)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 216);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 217)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 217);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 218)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 218);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 219)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 219);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 220)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 220);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 221)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 221);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 222)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 222);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 223)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 223);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 224)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 224);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 225)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 225);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 226)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 226);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 227)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 227);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 228)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 228);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 229)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 229);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 230)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 230);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 231)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 231);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 232)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 232);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 233)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 233);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 234)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 234);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 235)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 235);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 236)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 236);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 237)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 237);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 238)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 238);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 239)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 239);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 240)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 240);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 241)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 241);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 242)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 242);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 243)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 243);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 244)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 244);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 245)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 245);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 246)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 246);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 247)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 247);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 248)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 248);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 249)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 249);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 250)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 250);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 251)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 251);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 252)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 252);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 253)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 253);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 254)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 254);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 255)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 255);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 256)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 256);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 257)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 257);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 258)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 258);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 259)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 259);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 260)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 260);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 261)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 261);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 262)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 262);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 263)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 263);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 264)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 264);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 265)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 265);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 266)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 266);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 267)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 267);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 268)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 268);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 269)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 269);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 270)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 270);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 271)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 271);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 272)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 272);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 273)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 273);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 274)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 274);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 275)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 275);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 276)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 276);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 277)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 277);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 278)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 278);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 279)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 279);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 280)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 280);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 281)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 281);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 282)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 282);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 283)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 283);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 284)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 284);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 285)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 285);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 286)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 286);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 287)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 287);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 288)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 288);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 289)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 289);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 290)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 290);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 291)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 291);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 292)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 292);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 293)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 293);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 294)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 294);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 295)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 295);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 296)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 296);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 297)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 297);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 298)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 298);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 299)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 299);
                let _ = 1 + 1;
            }
            if coverage_str.contains(&format!("COV={}", 300)) {
                let _ = 1 + 1;
                println!("We've passed {:?}", 300);
                let _ = 1 + 1;
            }
        }
        // For each call, we verify that invariants aren't broken
        if !invariant_manager.are_invariants_passing() {
            panic!("Invariant triggered!");
        }
        println!("========= NEXT EXECUTION!!!!!!!!!===========");
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::sync::Mutex;

    #[test]
    fn test_parse_input() {
        // Input data
        let metadata_path = Path::new("sample/dns/target/ink/dns.json");

        let mut transcoder = Mutex::new(ContractMessageTranscoder::load(metadata_path).unwrap());

        let encoded_bytes =
            hex::decode("229b553f9400000000000000000027272727272727272700002727272727272727272727")
                .unwrap();
        let hex = transcoder
            .lock()
            .unwrap()
            .decode_contract_message(&mut &encoded_bytes[..]);
        assert_eq!(
            hex.unwrap().to_string(),
            "register { name: 0x9400000000000000000027272727272727272700002727272727272727272727 }"
        );

        /// 00000000 : money
        /// 0001 : alice
        /// 229b553f: selector
        /// 9400000000000000000027272727272727272700002727272727272727272727: params
        /// 2a2a2a2a2a2a2a2a: delimiter
        /// ...
        let double_call =
            hex::decode("000000000001229b553f94000000000000000000272727272727272727000027272727272727272727272a2a2a2a2a2a2a2a000000000001229b553f9400000000000000000027272727272727272700002727272727272727272727")
                .unwrap();

        let result = parse_input(double_call.as_slice(), &mut transcoder);
        assert_eq!(result.messages.len(), 2);
    }
}
