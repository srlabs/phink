use std::{fs, fs::File, io::Write, path::Path, sync::Mutex};

use contract_transcode::ContractMessageTranscoder;
use frame_support::__private::BasicExternalities;

use crate::{
    contract::payload::{PayloadCrafter, Selector},
    contract::remote::ContractBridge,
    contract::remote::FullContractResponse,
    fuzzer::coverage::Coverage,
    fuzzer::engine::FuzzerEngine,
    fuzzer::invariants::BugManager,
    fuzzer::parser::{parse_input, OneInput},
};

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

        let mut invariant_manager = BugManager::from(inv, self.setup.clone());

        Self::make_initial_corpus(&mut selectors).expect("Failed to create initial corpus");
        println!(
            "\n\n🚀  Now fuzzing `{}` ({})!\n\n",
            self.setup.path_to_specs.as_os_str().to_str().unwrap(),
            self.setup.contract_address
        );

        ziggy::fuzz!(|data: &[u8]| {
            Self::harness(
                self.clone(),
                &mut transcoder_loader,
                &mut selectors.clone(),
                &mut invariant_manager,
                data,
            );
        });
    }

    fn harness(
        client: Fuzzer,
        transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
        _selectors: &mut Vec<Selector>,
        bug_manager: &mut BugManager,
        input: &[u8],
    ) {
        let decoded_msgs: OneInput = parse_input(input, transcoder_loader);

        if decoded_msgs.messages.len() == 0 {
            return;
        }

        let mut chain = BasicExternalities::new(client.setup.genesis.clone());
        chain.execute_with(|| <Fuzzer as FuzzerEngine>::timestamp(0));

        let mut coverage: Coverage = Coverage::new();
        let mut all_msg_responses: Vec<FullContractResponse> = Vec::new();

        chain.execute_with(|| {
            for message in &decoded_msgs.messages {
                let transfer_value = if message.is_payable {
                    message.value_token
                } else {
                    0
                };

                let result: FullContractResponse = client.setup.clone().call(
                    &message.payload,
                    decoded_msgs.origin as u8,
                    transfer_value,
                );

                // For each call, we verify that it isn't trapped
                if bug_manager.is_contract_trapped(&result) {
                    bug_manager.display_trap(message, &result);
                }

                coverage.add_cov(&result.debug_message);
                all_msg_responses.push(result);
            }

            // For each group of call, we verify that invariants aren't broken
            if let Err(trace) = bug_manager.are_invariants_passing(decoded_msgs.origin) {
                bug_manager.display_invariant(
                    all_msg_responses.clone(),
                    decoded_msgs.clone(),
                    trace,
                );
            }
        });

        #[cfg(not(fuzzing))]
        <Fuzzer as FuzzerEngine>::pretty_print(all_msg_responses, decoded_msgs);

        // We now fake the coverage
        coverage.redirect_coverage();
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Mutex;

    use super::*;

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
            .decode_contract_message(&mut &encoded_bytes[..])
            .unwrap();
        assert_eq!(
            hex.to_string(),
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
