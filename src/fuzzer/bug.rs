#[allow(unused_imports, unused_variables)]
use crate::{
    cli::config::Configuration,
    contract::{
        payload::Selector,
        remote::{
            ContractBridge,
            FullContractResponse,
        },
    },
    cover::coverage::Coverage,
    fuzzer::{
        engine::FuzzerEngine,
        fuzz::Fuzzer,
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
    pub contract_bridge: ContractBridge,
    pub invariant_selectors: Vec<Selector>,
    pub configuration: Configuration,
}

impl BugManager {
    pub fn from(
        invariant_selectors: Vec<Selector>,
        contract_bridge: ContractBridge,
        configuration: Configuration,
    ) -> Self {
        Self { contract_bridge, invariant_selectors, configuration }
    }

    pub fn contains_selector(&self, selector: &Selector) -> bool {
        self.invariant_selectors.contains(selector)
    }

    pub fn display_trap(
        &self,
        message: Message,
        response: FullContractResponse,
    ) {
        // We print the details only when we don't fuzz, so when we run a seed
        // for instance, otherwise this will pollute the AFL logs
        #[cfg(not(fuzzing))]
        {
            println!("\n🤯 A trapped contract got caught! Let's dive into it");

            println!(
                "\n🐛 IMPORTANT STACKTRACE : {}\n",
                String::from_utf8_lossy(&Coverage::remove_cov_from_trace(
                    response.clone().debug_message
                ))
                .replace("\n", " ")
            );

            println!(
                "🎉 Find below the trace that caused that trapped contract"
            );

            <Fuzzer as FuzzerEngine>::pretty_print(
                vec![response],
                OneInput {
                    messages: vec![message.clone()],
                    origin: message.origin,
                    fuzz_option: self.configuration.should_fuzz_origin(),
                },
            );
        }

        // Artificially trigger a bug for AFL
        panic!("\nJob is done! Please, don't mind the backtrace below/above 🫡\n\n");
    }

    pub fn display_invariant(
        &self,
        responses: Vec<FullContractResponse>,
        decoded_msg: OneInput,
        invariant_tested: Selector,
        transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
    ) {
        let mut invariant_slice: &[u8] = &invariant_tested;

        let hex = transcoder_loader
            .get_mut()
            .unwrap()
            .decode_contract_message(&mut invariant_slice)
            .unwrap();

        #[cfg(not(fuzzing))]
        {
            println!("\n🤯 An invariant got caught! Let's dive into it");

            println!("\n🫵  This was caused by `{}`\n", hex);

            println!("🎉 Find below the trace that caused that invariant");
            <Fuzzer as FuzzerEngine>::pretty_print(responses, decoded_msg);
        }
        // Artificially trigger a bug for AFL
        panic!("\nJob is done! Please, don't mind the backtrace below/above 🫡\n\n");
    }

    /// This function aims to call every invariant function via
    /// `invariant_selectors`.
    pub fn are_invariants_passing(
        &self,
        origin: Origin,
    ) -> Result<(), Selector> {
        for invariant in &self.invariant_selectors {
            let invariant_call: FullContractResponse =
                self.contract_bridge.clone().call(
                    invariant.as_ref(),
                    origin.into(),
                    0,
                    self.configuration.clone(),
                );
            if invariant_call.result.is_err() {
                return Err(*invariant)
            }
        }
        Ok(())
    }

    pub fn is_contract_trapped(
        &self,
        contract_response: &FullContractResponse,
    ) -> bool {
        if let Err(DispatchError::Module(ModuleError { message, .. })) =
            contract_response.result
        {
            if message == Some("ContractTrapped") {
                return true;
            }
        }
        false
    }
}
