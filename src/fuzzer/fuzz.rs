use std::{fs, fs::File, io::Write, path::Path, sync::Mutex};

use contract_transcode::ContractMessageTranscoder;
use frame_support::__private::BasicExternalities;

use crate::{
    contract::payload::{PayloadCrafter, Selector},
    contract::remote::ContractBridge,
    contract::remote::FullContractResponse,
    fuzzer::bug::BugManager,
    fuzzer::coverage::Coverage,
    fuzzer::engine::FuzzerEngine,
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

    fn build_corpus_and_dict(selectors: &mut [Selector]) -> Result<(), std::io::Error> {
        // Create the output directory for the corpus
        fs::create_dir_all("./output/phink/corpus")?;

        // Create or truncate the dictionary file
        let mut dict_file = File::create("./output/phink/selectors.dict")?;

        // Write the dictionary header comments
        writeln!(dict_file, "# Dictionary file for selectors")?;
        writeln!(
            dict_file,
            "# Lines starting with '#' and empty lines are ignored."
        )?;
        writeln!(dict_file)?;

        for (i, selector) in selectors.iter().enumerate() {
            // Create the corpus file path
            let file_path = format!("{}/selector_{}.bin", "./output/phink/corpus", i);
            let mut file = File::create(&file_path)?;

            // Write the selector to the corpus file
            file.write_all(selector)?;

            // Write the selector to the dictionary file in the required format
            let selector_string = selector
                .iter()
                .map(|b| format!("\\x{:02X}", b))
                .collect::<String>();
            writeln!(dict_file, "\"{}\"", selector_string)?;
        }

        Ok(())
    }

    // This function handles edge cases where the fuzzer should understand that some conditions are
    // required in order to continue
    fn should_stop_now(bug_manager: &mut BugManager, decoded_msgs: &OneInput) -> bool {
        // Condition 1: we break if we cannot decode any message
        if decoded_msgs.messages.len() == 0 {
            return true;
        }

        // Condition 2: we break if the fuzzed message is an invariant message
        if decoded_msgs.messages.iter().any(|payload| {
            payload
                .payload
                .get(0..4)
                .and_then(|slice| slice.try_into().ok())
                .map_or(false, |slice: &[u8; 4]| {
                    bug_manager.contains_selector(slice)
                })
        }) {
            return true;
        }
        false
    }
}

impl FuzzerEngine for Fuzzer {
    fn fuzz(self) {
        let mut transcoder_loader = Mutex::new(
            ContractMessageTranscoder::load(Path::new(&self.setup.path_to_specs)).unwrap(),
        );

        let specs = &self.setup.json_specs;

        let selectors: Vec<Selector> = PayloadCrafter::extract_all(specs);
        let invariants: Vec<Selector> = PayloadCrafter::extract_invariants(specs)
            .expect("No invariants found, check your contract");

        let mut selectors_without_invariants: Vec<Selector> = selectors
            .clone()
            .into_iter()
            .filter(|s| !invariants.clone().contains(s))
            .collect();

        let mut invariant_manager = BugManager::from(invariants, self.setup.clone());

        Self::build_corpus_and_dict(&mut selectors_without_invariants)
            .expect("🙅 Failed to create initial corpus");

        println!(
            "\n\n🚀  Now fuzzing `{}` ({})!\n",
            self.setup.path_to_specs.as_os_str().to_str().unwrap(),
            self.setup.contract_address
        );

        ziggy::fuzz!(|data: &[u8]| {
            Self::harness(
                self.clone(),
                &mut transcoder_loader,
                &mut invariant_manager,
                data,
            );
        });
    }

    fn harness(
        client: Fuzzer,
        transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
        bug_manager: &mut BugManager,
        input: &[u8],
    ) {
        let decoded_msgs: OneInput = parse_input(input, transcoder_loader);

        if Self::should_stop_now(bug_manager, &decoded_msgs) {
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
                    bug_manager.display_trap(message.clone(), result.clone());
                }

                coverage.add_cov(&result.debug_message);
                all_msg_responses.push(result);
            }

            // For each group of call, we verify that invariants aren't broken
            if let Err(invariant_tested) = bug_manager.are_invariants_passing(decoded_msgs.origin) {
                bug_manager.display_invariant(
                    all_msg_responses.clone(),
                    decoded_msgs.clone(),
                    invariant_tested,
                    transcoder_loader,
                );
            }
        });

        // Pretty print all the calls of the current input
        // #[cfg(not(fuzzing))] //TODO: investigate why this doesn't work
        <Fuzzer as FuzzerEngine>::pretty_print(all_msg_responses, decoded_msgs);

        // We now fake the coverage
        coverage.redirect_coverage();
    }
}

#[cfg(test)]
mod tests {
    use crate::fuzzer::parser::Message;
    use ink_metadata::Selector;
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
