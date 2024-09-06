#![allow(unused_imports, unused_variables)]
use crate::{
    cli::config::Configuration,
    contract::{
        payload::Selector,
        remote::{
            ContractSetup,
            FullContractResponse,
        },
    },
    cover::coverage::InputCoverage,
    fuzzer::{
        engine::pretty_print,
        parser::{
            Message,
            OneInput,
            Origin,
        },
    },
};
use contract_transcode::ContractMessageTranscoder;
use sp_runtime::{
    DispatchError,
    ModuleError,
};
use std::{
    panic,
    sync::Mutex,
};

#[derive(Clone)]
pub struct BugManager {
    pub contract_bridge: ContractSetup,
    pub invariant_selectors: Vec<Selector>,
    pub configuration: Configuration,
}

impl BugManager {
    pub fn new(
        invariant_selectors: Vec<Selector>,
        contract_bridge: ContractSetup,
        configuration: Configuration,
    ) -> Self {
        Self {
            contract_bridge,
            invariant_selectors,
            configuration,
        }
    }

    pub fn contains_selector(&self, selector: Selector) -> bool {
        self.invariant_selectors.contains(&selector)
    }

    pub fn display_trap(&self, message: Message, response: FullContractResponse) {
        // We print the details only when we don't fuzz, so when we run a seed
        // for instance, otherwise this will pollute the AFL logs
        #[cfg(not(fuzzing))]
        {
            println!("\n🤯 A trapped contract got caught! Let's dive into it");

            println!(
                "\n🐛 IMPORTANT STACKTRACE : {}\n",
                String::from_utf8_lossy(&InputCoverage::remove_cov_from_trace(
                    response.clone().debug_message
                ))
                .replace("\n", " ")
            );

            println!("🎉 Find below the trace that caused that trapped contract");

            pretty_print(
                vec![response],
                OneInput {
                    messages: vec![message.clone()],
                    fuzz_option: self.configuration.should_fuzz_origin(),
                },
            );
        }

        // Artificially trigger a bug for AFL
        panic!("\n🫡  Job is done! Please, don't mind the backtrace below/above.\n\n");
    }

    pub fn display_invariant(
        &self,
        responses: Vec<FullContractResponse>,
        decoded_msg: OneInput,
        mut invariant_tested: Selector,
        transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
    ) {
        let hex = transcoder_loader
            .get_mut()
            .unwrap()
            .decode_contract_message(&mut &*invariant_tested.as_mut())
            .unwrap();

        #[cfg(not(fuzzing))]
        {
            println!("\n🤯 An invariant got caught! Let's dive into it");
            println!("\n🫵  This was caused by `{hex}`\n");
            println!("🎉 Find below the trace that caused that invariant");
            pretty_print(responses, decoded_msg);
        }
        // Artificially trigger a bug for AFL
        panic!("\n🫡   Job is done! Please, don't mind the backtrace below/above.\n\n");
    }

    /// This function aims to call every invariants via `invariant_selectors`.
    pub fn are_invariants_passing(&self, origin: Origin) -> Result<(), Selector> {
        for invariant in &self.invariant_selectors {
            let invariant_call: FullContractResponse = self.contract_bridge.clone().call(
                invariant.as_ref(),
                origin.into(),
                0,
                self.configuration.clone(),
            );
            if invariant_call.result.is_err() {
                return Err(invariant.clone());
            }
        }
        Ok(())
    }

    pub fn is_contract_trapped(&self, contract_response: &FullContractResponse) -> bool {
        if let Err(DispatchError::Module(ModuleError { message, .. })) = contract_response.result {
            if message == Some("ContractTrapped") {
                return true;
            }
        }
        false
    }
}
