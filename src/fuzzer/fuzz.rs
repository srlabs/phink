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

            seq!(x in 0..=500 {
               if coverage_str.contains(&format!("COV={}", x)) {
                    let _ = 1 + 1;
                    println!("We've passed {:?}", x);
                    let _ = 1 + 1;
                }
            });
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
