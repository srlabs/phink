use crate::contract::payload::Selector;
use crate::contract::remote::ContractBridge;

pub struct Invariants {
    pub contract_bridge: ContractBridge,
    pub invariant_selectors: Vec<Selector>,
}

impl Invariants {
    pub fn from(invariant_selectors: Vec<Selector>, contract_bridge: ContractBridge) -> Self {
        Self {
            contract_bridge,
            invariant_selectors,
        }
    }

    /// This function aims to call every invariant function via `invariant_selectors`.
    pub fn are_invariants_passing(&self, origin: u8) -> bool {
        for invariant in &self.invariant_selectors {
            let invariant_call = self
                .contract_bridge
                .clone()
                .call(&invariant.to_vec(), origin, 0);
            if let Err(_) = invariant_call.result {
                println!(
                    "Invariant Debug Message {:?}",
                    String::from_utf8_lossy(&*invariant_call.debug_message)
                );
                return false;
            }
        }
        true
    }
}
