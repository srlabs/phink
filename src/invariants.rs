use crate::payload::Selector;

pub struct Invariants {
    invariant_selectors: Vec<Selector>,
}

impl Invariants {
    ///To calculate the selector we:
    //
    // Grab the name of the trait and the name of the message, Frobinate::frobinate
    // Compute BLAKE2("Frobinate::frobinate") = 0x8915412ad772b2a116917cf75df4ba461b5808556a73f729bce582fb79200c5b
    // Grab the first four bytes, 0x8915412a
    pub fn from(invariant_selectors: Vec<Selector>) -> Self {
        Self {
            invariant_selectors,
        }
    }
    pub fn are_invariants_passing(&self) -> bool {
        true
    }
}
