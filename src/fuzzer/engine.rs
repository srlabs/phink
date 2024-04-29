use crate::fuzzer::invariants::Invariants;
use contract_transcode::ContractMessageTranscoder;
use std::path::Path;
use std::sync::Mutex;

pub trait FuzzerEngine {
    fn fuzz(self);
}
