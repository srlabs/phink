use prettytable::{row, Table};
use sp_runtime::{DispatchError, ModuleError};

use crate::fuzzer::engine::FuzzerEngine;
use crate::fuzzer::fuzz::Fuzzer;
use crate::fuzzer::parser::OneInput;
use crate::{
    contract::payload::Selector,
    contract::remote::{ContractBridge, FullContractResponse},
    fuzzer::parser::Message,
};

pub type FailedInvariantTrace = (Vec<u8>, FullContractResponse);
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
        let mut table = Table::new();
        table.add_row(row!["Description", "SCALE", "Result"]);

        table.printstd();

        <Fuzzer as FuzzerEngine>::pretty_print(
            vec![response],
            OneInput {
                messages: vec![message.clone()],
                origin: message.origin,
            },
        );

        panic!("😲");
    }

    pub fn display_invariant(
        &self,
        responses: Vec<FullContractResponse>,
        decoded_msg: OneInput,
        trace: FailedInvariantTrace,
    ) {
        println!(
            "Broken invariant message : {:?}",
            String::from_utf8_lossy(trace.0.as_ref())
        );
        <Fuzzer as FuzzerEngine>::pretty_print(responses, decoded_msg);
        panic!("😲");
    }

    /// This function aims to call every invariant function via `invariant_selectors`.
    pub fn are_invariants_passing(&self, origin: usize) -> Result<(), FailedInvariantTrace> {
        for invariant in &self.invariant_selectors {
            let invariant_call: FullContractResponse =
                self.contract_bridge
                    .clone()
                    .call(&invariant.to_vec(), origin as u8, 0);
            if let Err(_) = invariant_call.result {
                return Err((invariant.to_vec(), invariant_call));
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
