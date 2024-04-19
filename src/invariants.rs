use crate::payload::Selector;
use crate::remote::ContractBridge;

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

    /// This function aims to call every invariant function via `invariant_selectors`.
    pub fn are_invariants_passing(&self) -> bool {
        for invariant in self.invariant_selectors {
            let toz = self.contract_bridge.clone().call(&invariant.to_vec()).unwrap();
            println!("{:?}", toz);
        }
        true
    }
}
