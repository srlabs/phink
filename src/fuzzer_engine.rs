use crate::invariants::Invariants;
use crate::payload::{PayloadCrafter, Selector};
use contract_transcode::ContractMessageTranscoder;
use std::path::Path;
use std::sync::Mutex;

pub trait FuzzerEngine {
    #[warn(unused_variables)]
    fn fuzz(self);
}
