use contract_transcode::ContractMessageTranscoder;
use prettytable::{row, Table};
use sp_runtime::{DispatchError, ModuleError};
use std::sync::Mutex;

use crate::{
    contract::{
        payload::Selector,
        remote::{ContractBridge, FullContractResponse},
    },
    fuzzer::{engine::FuzzerEngine, fuzz::Fuzzer, parser::Message, parser::OneInput},
};

pub type FailedInvariantTrace = (Selector, FullContractResponse);
pub struct BugManager {
    pub contract_bridge: ContractBridge,
    pub invariant_selectors: Vec<Selector>,
}

impl BugManager {
    pub fn from(invariant_selectors: Vec<Selector>, contract_bridge: ContractBridge) -> Self {
        Self {
            contract_bridge,
            invariant_selectors,
        }
    }

    pub fn display_trap(&self, message: Message, response: FullContractResponse) {
        println!("🤯 A *trap contract* got caught! Let's dive down.");
        println!(
            "🐛 IMPORTANT STACKTRACE : {:?}",
            String::from_utf8_lossy(response.debug_message.as_ref())
        );

        println!("🎉 Find below the trace that caused that *trapped contract*");

        <Fuzzer as FuzzerEngine>::pretty_print(
            vec![response],
            OneInput {
                messages: vec![message.clone()],
                origin: message.origin,
            },
        );

        panic!("Good luck ser! 🫡"); //Artificially trigger a bug for AFL
    }

    pub fn display_invariant(
        &self,
        responses: Vec<FullContractResponse>,
        decoded_msg: OneInput,
        invariant_tested: FailedInvariantTrace,
        transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
    ) {
        println!("🤯 An *invariant* got caught! Let's dive down.");

        // Convert the array to a slice and then take a mutable reference to the slice
        let mut invariant_slice: &[u8] = &invariant_tested.0;

        let hex = transcoder_loader
            .lock()
            .unwrap()
            .decode_contract_message(&mut invariant_slice)
            .unwrap();

        let mut table = Table::new();
        table.add_row(row!["Invariant", "Debug trace"]);

        table.add_row(row![
            hex,
            String::from_utf8_lossy(invariant_tested.0.as_ref())
        ]);

        table.printstd();

        println!("🎉 Find below the trace that caused that *invariant*");
        <Fuzzer as FuzzerEngine>::pretty_print(responses, decoded_msg);
        panic!("Good luck ser! 🫡"); //Artificially trigger a bug for AFL
    }

    /// This function aims to call every invariant function via `invariant_selectors`.
    pub fn are_invariants_passing(&self, origin: usize) -> Result<(), FailedInvariantTrace> {
        for invariant in &self.invariant_selectors {
            let invariant_call: FullContractResponse =
                self.contract_bridge
                    .clone()
                    .call(&invariant.to_vec(), origin as u8, 0);
            if let Err(_) = invariant_call.result {
                return Err((*invariant, invariant_call));
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
