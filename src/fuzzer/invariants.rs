use crate::contract::payload::Selector;
use crate::contract::remote::ContractBridge;
use contract_transcode::ContractMessageTranscoder;
use pallet_contracts::ExecReturnValue;
use parity_scale_codec::Decode;
use sp_runtime::{DispatchError, ModuleError};
use std::sync::MutexGuard;

pub struct Invariants {
    contract_bridge: ContractBridge,
    invariant_selectors: Vec<Selector>,
}

impl Invariants {
    pub fn from(invariant_selectors: Vec<Selector>, contract_bridge: ContractBridge) -> Self {
        Self {
            contract_bridge,
            invariant_selectors,
        }
    }

    // TODO! Refactor this, it's basically return to type & human readable :)
    // println!(
    //     "{:?}",
    //     self.transcoder.decode_message_return(
    //         "phink_assert_abc_dot_com_cant_be_registered",
    //         &mut toz.data.as_slice()
    //     )
    // );

    /// This function aims to call every invariant function via `invariant_selectors`.
    pub fn are_invariants_passing(&self) -> bool {
        for invariant in &self.invariant_selectors {
            let toz = self.contract_bridge.clone().call(&invariant.to_vec());
            if let Err(_) = toz.result {
                println!(
                    "Invariant Debug Message {:?}",
                    String::from_utf8_lossy(&*toz.debug_message)
                );
                return false;
            }
        }
        true
    }
}
