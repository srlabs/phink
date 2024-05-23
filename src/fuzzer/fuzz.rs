use contract_transcode::ContractMessageTranscoder;
use frame_support::{
    __private::BasicExternalities,
    traits::{OnFinalize, OnInitialize},
};

use crate::contract::payload::{PayloadCrafter, Selector};
use crate::contract::remote::ContractBridge;
use crate::fuzzer::engine::FuzzerEngine;
use crate::fuzzer::invariants::Invariants;
use crate::utils;
use frame_support::traits::Len;
use itertools::Itertools;
use libafl::corpus::Corpus;
use libafl::inputs::HasBytesVec;
#[cfg(feature = "tui")]
use libafl::monitors::tui::{ui::TuiUI, TuiMonitor};
#[cfg(not(feature = "tui"))]
use libafl::monitors::SimpleMonitor;
use libafl::mutators::{havoc_mutations, Mutator};
use libafl::prelude::HasRand;
use libafl::prelude::*;
use libafl::schedulers::powersched::PowerSchedule;
use libafl::state::HasCorpus;
use libafl::{
    corpus::{InMemoryCorpus, OnDiskCorpus},
    events::SimpleEventManager,
    executors::{inprocess::InProcessExecutor, ExitKind},
    feedbacks::{CrashFeedback, MaxMapFeedback},
    fuzzer::{Fuzzer, StdFuzzer},
    inputs::{BytesInput, HasTargetBytes},
    mutators::StdScheduledMutator,
    observers::StdMapObserver,
    schedulers::QueueScheduler,
    stages::mutational::StdMutationalStage,
    state::StdState,
    Evaluator, HasMetadata,
};
use libafl_bolts::rands::{Rand, RandomSeed};
use libafl_bolts::tuples::Merge;
use libafl_bolts::{rands::StdRand, tuples::tuple_list, AsSlice, Error, Named};
use pallet_contracts::runtime_decl_for_contracts_api::ID;
use parity_scale_codec::{DecodeLimit, Encode};
use std::borrow::Cow;
use std::fs::File;
use std::io::Write;
use std::{fs, path::Path, sync::Mutex};
use std::{path::PathBuf, ptr::write};

#[derive(Clone)]
pub struct LibAFLFuzzer {
    setup: ContractBridge,
}

impl LibAFLFuzzer {
    pub fn new(setup: ContractBridge) -> LibAFLFuzzer {
        LibAFLFuzzer { setup }
    }

    fn make_initial_corpus(selectors: &mut Vec<Selector>) {
        fs::create_dir_all("./output/phink/corpus").unwrap();

        // Iterate over the selectors and write each one to a separate file
        for (i, selector) in selectors.iter().enumerate() {
            let file_path = format!("{}/selector_{}.bin", "./output/phink/corpus", i);
            let mut file = File::create(&file_path).unwrap();

            file.write_all(selector).unwrap();
        }
    }
}

impl FuzzerEngine for LibAFLFuzzer {
    /// This is the main fuzzing function. Here, we fuzz ink!, and the planet
    // #[no_mangle]
    fn setup(self) {
        let mut transcoder_loader = Mutex::new(
            ContractMessageTranscoder::load(Path::new(&self.setup.path_to_specs)).unwrap(),
        );

        let specs = &self.setup.json_specs;
        let mut selectors: Vec<Selector> = PayloadCrafter::extract_all(specs);
        let inv = PayloadCrafter::extract_invariants(specs)
            .expect("No invariants found, check your contract");

        let mut invariant_manager = Invariants::from(inv, self.setup.clone());

        Self::make_initial_corpus(&mut selectors);

        ziggy::fuzz!(|data: &[u8]| {
            harness(
                self.clone(),
                &mut transcoder_loader,
                &mut selectors.clone(),
                &mut invariant_manager,
                data,
            );
        });
    }
}

use seq_macro::seq;
use crate::contract::runtime::RuntimeCall;

fn harness(
    client: LibAFLFuzzer,
    transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
    selectors: &mut Vec<Selector>,
    invariant_manager: &mut Invariants,
    input: &[u8],
) {
    let binding = client.clone();
    let raw_call = binding.parse_args(input, selectors.clone());

    if raw_call.is_none() {
        return;
    }

    let decoded_msg = transcoder_loader
        .lock()
        .unwrap()
        .decode_contract_message(&mut &*raw_call.clone().unwrap());

    if let Err(_) = decoded_msg {
        return;
    }

    let mut chain = BasicExternalities::new(client.setup.genesis.clone());
    chain.execute_with(|| <LibAFLFuzzer as FuzzerEngine>::timestamp());

    // let mut calls: Vec<Vec<u8>> = Vec::new();

    // calls.push(raw_call.clone().unwrap());



    chain.execute_with(|| {
        let mut results: Vec<Vec<u8>> = Vec::new();
        // for call in calls {


            let result = client.setup.clone().call(&raw_call.clone().unwrap());
            results.push(result.debug_message);

            // We pretty-print all information that we need to debug
            #[cfg(not(fuzzing))]
            <LibAFLFuzzer as FuzzerEngine>::pretty_print(
                result.result.clone(),
                decoded_msg.unwrap().to_string(),
                raw_call.unwrap(),
            );
        // }

        let flatten_cov: &[u8] = &*results.into_iter().flatten().collect::<Vec<_>>();
        let mut coverage_str = utils::deduplicate(&*String::from_utf8_lossy(flatten_cov));

        // seq!(x in 0..=500 {
        //    if coverage_str.contains(&format!("COV={}", x)) {
        //     let _ = 1 + 1;
        //     println!("We've passed {:?}", x);
        // }
        // });

        if coverage_str.contains(&format!("COV={}", 0)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 0);
        }
        if coverage_str.contains(&format!("COV={}", 1)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 1);
        }
        if coverage_str.contains(&format!("COV={}", 2)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 2);
        }
        if coverage_str.contains(&format!("COV={}", 3)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 3);
        }
        if coverage_str.contains(&format!("COV={}", 4)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 4);
        }
        if coverage_str.contains(&format!("COV={}", 5)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 5);
        }
        if coverage_str.contains(&format!("COV={}", 6)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 6);
        }
        if coverage_str.contains(&format!("COV={}", 7)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 7);
        }
        if coverage_str.contains(&format!("COV={}", 8)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 8);
        }
        if coverage_str.contains(&format!("COV={}", 9)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 9);
        }
        if coverage_str.contains(&format!("COV={}", 10)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 10);
        }
        if coverage_str.contains(&format!("COV={}", 11)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 11);
        }
        if coverage_str.contains(&format!("COV={}", 12)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 12);
        }
        if coverage_str.contains(&format!("COV={}", 13)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 13);
        }
        if coverage_str.contains(&format!("COV={}", 14)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 14);
        }
        if coverage_str.contains(&format!("COV={}", 15)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 15);
        }
        if coverage_str.contains(&format!("COV={}", 16)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 16);
        }
        if coverage_str.contains(&format!("COV={}", 17)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 17);
        }
        if coverage_str.contains(&format!("COV={}", 18)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 18);
        }
        if coverage_str.contains(&format!("COV={}", 19)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 19);
        }
        if coverage_str.contains(&format!("COV={}", 20)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 20);
        }
        if coverage_str.contains(&format!("COV={}", 21)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 21);
        }
        if coverage_str.contains(&format!("COV={}", 22)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 22);
        }
        if coverage_str.contains(&format!("COV={}", 23)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 23);
        }
        if coverage_str.contains(&format!("COV={}", 24)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 24);
        }
        if coverage_str.contains(&format!("COV={}", 25)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 25);
        }
        if coverage_str.contains(&format!("COV={}", 26)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 26);
        }
        if coverage_str.contains(&format!("COV={}", 27)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 27);
        }
        if coverage_str.contains(&format!("COV={}", 28)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 28);
        }
        if coverage_str.contains(&format!("COV={}", 29)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 29);
        }
        if coverage_str.contains(&format!("COV={}", 30)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 30);
        }
        if coverage_str.contains(&format!("COV={}", 31)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 31);
        }
        if coverage_str.contains(&format!("COV={}", 32)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 32);
        }
        if coverage_str.contains(&format!("COV={}", 33)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 33);
        }
        if coverage_str.contains(&format!("COV={}", 34)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 34);
        }
        if coverage_str.contains(&format!("COV={}", 35)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 35);
        }
        if coverage_str.contains(&format!("COV={}", 36)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 36);
        }
        if coverage_str.contains(&format!("COV={}", 37)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 37);
        }
        if coverage_str.contains(&format!("COV={}", 38)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 38);
        }
        if coverage_str.contains(&format!("COV={}", 39)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 39);
        }
        if coverage_str.contains(&format!("COV={}", 40)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 40);
        }
        if coverage_str.contains(&format!("COV={}", 41)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 41);
        }
        if coverage_str.contains(&format!("COV={}", 42)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 42);
        }
        if coverage_str.contains(&format!("COV={}", 43)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 43);
        }
        if coverage_str.contains(&format!("COV={}", 44)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 44);
        }
        if coverage_str.contains(&format!("COV={}", 45)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 45);
        }
        if coverage_str.contains(&format!("COV={}", 46)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 46);
        }
        if coverage_str.contains(&format!("COV={}", 47)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 47);
        }
        if coverage_str.contains(&format!("COV={}", 48)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 48);
        }
        if coverage_str.contains(&format!("COV={}", 49)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 49);
        }
        if coverage_str.contains(&format!("COV={}", 50)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 50);
        }
        if coverage_str.contains(&format!("COV={}", 51)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 51);
        }
        if coverage_str.contains(&format!("COV={}", 52)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 52);
        }
        if coverage_str.contains(&format!("COV={}", 53)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 53);
        }
        if coverage_str.contains(&format!("COV={}", 54)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 54);
        }
        if coverage_str.contains(&format!("COV={}", 55)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 55);
        }
        if coverage_str.contains(&format!("COV={}", 56)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 56);
        }
        if coverage_str.contains(&format!("COV={}", 57)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 57);
        }
        if coverage_str.contains(&format!("COV={}", 58)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 58);
        }
        if coverage_str.contains(&format!("COV={}", 59)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 59);
        }
        if coverage_str.contains(&format!("COV={}", 60)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 60);
        }
        if coverage_str.contains(&format!("COV={}", 61)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 61);
        }
        if coverage_str.contains(&format!("COV={}", 62)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 62);
        }
        if coverage_str.contains(&format!("COV={}", 63)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 63);
        }
        if coverage_str.contains(&format!("COV={}", 64)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 64);
        }
        if coverage_str.contains(&format!("COV={}", 65)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 65);
        }
        if coverage_str.contains(&format!("COV={}", 66)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 66);
        }
        if coverage_str.contains(&format!("COV={}", 67)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 67);
        }
        if coverage_str.contains(&format!("COV={}", 68)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 68);
        }
        if coverage_str.contains(&format!("COV={}", 69)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 69);
        }
        if coverage_str.contains(&format!("COV={}", 70)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 70);
        }
        if coverage_str.contains(&format!("COV={}", 71)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 71);
        }
        if coverage_str.contains(&format!("COV={}", 72)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 72);
        }
        if coverage_str.contains(&format!("COV={}", 73)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 73);
        }
        if coverage_str.contains(&format!("COV={}", 74)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 74);
        }
        if coverage_str.contains(&format!("COV={}", 75)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 75);
        }
        if coverage_str.contains(&format!("COV={}", 76)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 76);
        }
        if coverage_str.contains(&format!("COV={}", 77)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 77);
        }
        if coverage_str.contains(&format!("COV={}", 78)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 78);
        }
        if coverage_str.contains(&format!("COV={}", 79)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 79);
        }
        if coverage_str.contains(&format!("COV={}", 80)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 80);
        }
        if coverage_str.contains(&format!("COV={}", 81)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 81);
        }
        if coverage_str.contains(&format!("COV={}", 82)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 82);
        }
        if coverage_str.contains(&format!("COV={}", 83)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 83);
        }
        if coverage_str.contains(&format!("COV={}", 84)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 84);
        }
        if coverage_str.contains(&format!("COV={}", 85)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 85);
        }
        if coverage_str.contains(&format!("COV={}", 86)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 86);
        }
        if coverage_str.contains(&format!("COV={}", 87)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 87);
        }
        if coverage_str.contains(&format!("COV={}", 88)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 88);
        }
        if coverage_str.contains(&format!("COV={}", 89)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 89);
        }
        if coverage_str.contains(&format!("COV={}", 90)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 90);
        }
        if coverage_str.contains(&format!("COV={}", 91)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 91);
        }
        if coverage_str.contains(&format!("COV={}", 92)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 92);
        }
        if coverage_str.contains(&format!("COV={}", 93)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 93);
        }
        if coverage_str.contains(&format!("COV={}", 94)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 94);
        }
        if coverage_str.contains(&format!("COV={}", 95)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 95);
        }
        if coverage_str.contains(&format!("COV={}", 96)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 96);
        }
        if coverage_str.contains(&format!("COV={}", 97)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 97);
        }
        if coverage_str.contains(&format!("COV={}", 98)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 98);
        }
        if coverage_str.contains(&format!("COV={}", 99)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 99);
        }
        if coverage_str.contains(&format!("COV={}", 100)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 100);
        }
        if coverage_str.contains(&format!("COV={}", 101)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 101);
        }
        if coverage_str.contains(&format!("COV={}", 102)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 102);
        }
        if coverage_str.contains(&format!("COV={}", 103)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 103);
        }
        if coverage_str.contains(&format!("COV={}", 104)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 104);
        }
        if coverage_str.contains(&format!("COV={}", 105)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 105);
        }
        if coverage_str.contains(&format!("COV={}", 106)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 106);
        }
        if coverage_str.contains(&format!("COV={}", 107)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 107);
        }
        if coverage_str.contains(&format!("COV={}", 108)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 108);
        }
        if coverage_str.contains(&format!("COV={}", 109)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 109);
        }
        if coverage_str.contains(&format!("COV={}", 110)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 110);
        }
        if coverage_str.contains(&format!("COV={}", 111)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 111);
        }
        if coverage_str.contains(&format!("COV={}", 112)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 112);
        }
        if coverage_str.contains(&format!("COV={}", 113)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 113);
        }
        if coverage_str.contains(&format!("COV={}", 114)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 114);
        }
        if coverage_str.contains(&format!("COV={}", 115)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 115);
        }
        if coverage_str.contains(&format!("COV={}", 116)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 116);
        }
        if coverage_str.contains(&format!("COV={}", 117)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 117);
        }
        if coverage_str.contains(&format!("COV={}", 118)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 118);
        }
        if coverage_str.contains(&format!("COV={}", 119)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 119);
        }
        if coverage_str.contains(&format!("COV={}", 120)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 120);
        }
        if coverage_str.contains(&format!("COV={}", 121)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 121);
        }
        if coverage_str.contains(&format!("COV={}", 122)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 122);
        }
        if coverage_str.contains(&format!("COV={}", 123)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 123);
        }
        if coverage_str.contains(&format!("COV={}", 124)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 124);
        }
        if coverage_str.contains(&format!("COV={}", 125)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 125);
        }
        if coverage_str.contains(&format!("COV={}", 126)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 126);
        }
        if coverage_str.contains(&format!("COV={}", 127)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 127);
        }
        if coverage_str.contains(&format!("COV={}", 128)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 128);
        }
        if coverage_str.contains(&format!("COV={}", 129)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 129);
        }
        if coverage_str.contains(&format!("COV={}", 130)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 130);
        }
        if coverage_str.contains(&format!("COV={}", 131)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 131);
        }
        if coverage_str.contains(&format!("COV={}", 132)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 132);
        }
        if coverage_str.contains(&format!("COV={}", 133)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 133);
        }
        if coverage_str.contains(&format!("COV={}", 134)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 134);
        }
        if coverage_str.contains(&format!("COV={}", 135)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 135);
        }
        if coverage_str.contains(&format!("COV={}", 136)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 136);
        }
        if coverage_str.contains(&format!("COV={}", 137)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 137);
        }
        if coverage_str.contains(&format!("COV={}", 138)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 138);
        }
        if coverage_str.contains(&format!("COV={}", 139)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 139);
        }
        if coverage_str.contains(&format!("COV={}", 140)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 140);
        }
        if coverage_str.contains(&format!("COV={}", 141)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 141);
        }
        if coverage_str.contains(&format!("COV={}", 142)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 142);
        }
        if coverage_str.contains(&format!("COV={}", 143)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 143);
        }
        if coverage_str.contains(&format!("COV={}", 144)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 144);
        }
        if coverage_str.contains(&format!("COV={}", 145)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 145);
        }
        if coverage_str.contains(&format!("COV={}", 146)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 146);
        }
        if coverage_str.contains(&format!("COV={}", 147)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 147);
        }
        if coverage_str.contains(&format!("COV={}", 148)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 148);
        }
        if coverage_str.contains(&format!("COV={}", 149)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 149);
        }
        if coverage_str.contains(&format!("COV={}", 150)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 150);
        }
        if coverage_str.contains(&format!("COV={}", 151)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 151);
        }
        if coverage_str.contains(&format!("COV={}", 152)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 152);
        }
        if coverage_str.contains(&format!("COV={}", 153)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 153);
        }
        if coverage_str.contains(&format!("COV={}", 154)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 154);
        }
        if coverage_str.contains(&format!("COV={}", 155)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 155);
        }
        if coverage_str.contains(&format!("COV={}", 156)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 156);
        }
        if coverage_str.contains(&format!("COV={}", 157)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 157);
        }
        if coverage_str.contains(&format!("COV={}", 158)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 158);
        }
        if coverage_str.contains(&format!("COV={}", 159)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 159);
        }
        if coverage_str.contains(&format!("COV={}", 160)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 160);
        }
        if coverage_str.contains(&format!("COV={}", 161)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 161);
        }
        if coverage_str.contains(&format!("COV={}", 162)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 162);
        }
        if coverage_str.contains(&format!("COV={}", 163)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 163);
        }
        if coverage_str.contains(&format!("COV={}", 164)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 164);
        }
        if coverage_str.contains(&format!("COV={}", 165)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 165);
        }
        if coverage_str.contains(&format!("COV={}", 166)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 166);
        }
        if coverage_str.contains(&format!("COV={}", 167)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 167);
        }
        if coverage_str.contains(&format!("COV={}", 168)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 168);
        }
        if coverage_str.contains(&format!("COV={}", 169)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 169);
        }
        if coverage_str.contains(&format!("COV={}", 170)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 170);
        }
        if coverage_str.contains(&format!("COV={}", 171)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 171);
        }
        if coverage_str.contains(&format!("COV={}", 172)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 172);
        }
        if coverage_str.contains(&format!("COV={}", 173)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 173);
        }
        if coverage_str.contains(&format!("COV={}", 174)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 174);
        }
        if coverage_str.contains(&format!("COV={}", 175)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 175);
        }
        if coverage_str.contains(&format!("COV={}", 176)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 176);
        }
        if coverage_str.contains(&format!("COV={}", 177)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 177);
        }
        if coverage_str.contains(&format!("COV={}", 178)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 178);
        }
        if coverage_str.contains(&format!("COV={}", 179)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 179);
        }
        if coverage_str.contains(&format!("COV={}", 180)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 180);
        }
        if coverage_str.contains(&format!("COV={}", 181)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 181);
        }
        if coverage_str.contains(&format!("COV={}", 182)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 182);
        }
        if coverage_str.contains(&format!("COV={}", 183)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 183);
        }
        if coverage_str.contains(&format!("COV={}", 184)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 184);
        }
        if coverage_str.contains(&format!("COV={}", 185)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 185);
        }
        if coverage_str.contains(&format!("COV={}", 186)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 186);
        }
        if coverage_str.contains(&format!("COV={}", 187)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 187);
        }
        if coverage_str.contains(&format!("COV={}", 188)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 188);
        }
        if coverage_str.contains(&format!("COV={}", 189)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 189);
        }
        if coverage_str.contains(&format!("COV={}", 190)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 190);
        }
        if coverage_str.contains(&format!("COV={}", 191)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 191);
        }
        if coverage_str.contains(&format!("COV={}", 192)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 192);
        }
        if coverage_str.contains(&format!("COV={}", 193)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 193);
        }
        if coverage_str.contains(&format!("COV={}", 194)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 194);
        }
        if coverage_str.contains(&format!("COV={}", 195)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 195);
        }
        if coverage_str.contains(&format!("COV={}", 196)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 196);
        }
        if coverage_str.contains(&format!("COV={}", 197)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 197);
        }
        if coverage_str.contains(&format!("COV={}", 198)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 198);
        }
        if coverage_str.contains(&format!("COV={}", 199)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 199);
        }
        if coverage_str.contains(&format!("COV={}", 200)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 200);
        }
        if coverage_str.contains(&format!("COV={}", 201)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 201);
        }
        if coverage_str.contains(&format!("COV={}", 202)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 202);
        }
        if coverage_str.contains(&format!("COV={}", 203)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 203);
        }
        if coverage_str.contains(&format!("COV={}", 204)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 204);
        }
        if coverage_str.contains(&format!("COV={}", 205)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 205);
        }
        if coverage_str.contains(&format!("COV={}", 206)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 206);
        }
        if coverage_str.contains(&format!("COV={}", 207)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 207);
        }
        if coverage_str.contains(&format!("COV={}", 208)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 208);
        }
        if coverage_str.contains(&format!("COV={}", 209)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 209);
        }
        if coverage_str.contains(&format!("COV={}", 210)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 210);
        }
        if coverage_str.contains(&format!("COV={}", 211)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 211);
        }
        if coverage_str.contains(&format!("COV={}", 212)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 212);
        }
        if coverage_str.contains(&format!("COV={}", 213)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 213);
        }
        if coverage_str.contains(&format!("COV={}", 214)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 214);
        }
        if coverage_str.contains(&format!("COV={}", 215)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 215);
        }
        if coverage_str.contains(&format!("COV={}", 216)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 216);
        }
        if coverage_str.contains(&format!("COV={}", 217)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 217);
        }
        if coverage_str.contains(&format!("COV={}", 218)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 218);
        }
        if coverage_str.contains(&format!("COV={}", 219)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 219);
        }
        if coverage_str.contains(&format!("COV={}", 220)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 220);
        }
        if coverage_str.contains(&format!("COV={}", 221)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 221);
        }
        if coverage_str.contains(&format!("COV={}", 222)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 222);
        }
        if coverage_str.contains(&format!("COV={}", 223)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 223);
        }
        if coverage_str.contains(&format!("COV={}", 224)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 224);
        }
        if coverage_str.contains(&format!("COV={}", 225)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 225);
        }
        if coverage_str.contains(&format!("COV={}", 226)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 226);
        }
        if coverage_str.contains(&format!("COV={}", 227)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 227);
        }
        if coverage_str.contains(&format!("COV={}", 228)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 228);
        }
        if coverage_str.contains(&format!("COV={}", 229)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 229);
        }
        if coverage_str.contains(&format!("COV={}", 230)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 230);
        }
        if coverage_str.contains(&format!("COV={}", 231)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 231);
        }
        if coverage_str.contains(&format!("COV={}", 232)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 232);
        }
        if coverage_str.contains(&format!("COV={}", 233)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 233);
        }
        if coverage_str.contains(&format!("COV={}", 234)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 234);
        }
        if coverage_str.contains(&format!("COV={}", 235)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 235);
        }
        if coverage_str.contains(&format!("COV={}", 236)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 236);
        }
        if coverage_str.contains(&format!("COV={}", 237)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 237);
        }
        if coverage_str.contains(&format!("COV={}", 238)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 238);
        }
        if coverage_str.contains(&format!("COV={}", 239)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 239);
        }
        if coverage_str.contains(&format!("COV={}", 240)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 240);
        }
        if coverage_str.contains(&format!("COV={}", 241)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 241);
        }
        if coverage_str.contains(&format!("COV={}", 242)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 242);
        }
        if coverage_str.contains(&format!("COV={}", 243)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 243);
        }
        if coverage_str.contains(&format!("COV={}", 244)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 244);
        }
        if coverage_str.contains(&format!("COV={}", 245)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 245);
        }
        if coverage_str.contains(&format!("COV={}", 246)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 246);
        }
        if coverage_str.contains(&format!("COV={}", 247)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 247);
        }
        if coverage_str.contains(&format!("COV={}", 248)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 248);
        }
        if coverage_str.contains(&format!("COV={}", 249)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 249);
        }
        if coverage_str.contains(&format!("COV={}", 250)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 250);
        }
        if coverage_str.contains(&format!("COV={}", 251)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 251);
        }
        if coverage_str.contains(&format!("COV={}", 252)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 252);
        }
        if coverage_str.contains(&format!("COV={}", 253)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 253);
        }
        if coverage_str.contains(&format!("COV={}", 254)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 254);
        }
        if coverage_str.contains(&format!("COV={}", 255)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 255);
        }
        if coverage_str.contains(&format!("COV={}", 256)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 256);
        }
        if coverage_str.contains(&format!("COV={}", 257)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 257);
        }
        if coverage_str.contains(&format!("COV={}", 258)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 258);
        }
        if coverage_str.contains(&format!("COV={}", 259)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 259);
        }
        if coverage_str.contains(&format!("COV={}", 260)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 260);
        }
        if coverage_str.contains(&format!("COV={}", 261)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 261);
        }
        if coverage_str.contains(&format!("COV={}", 262)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 262);
        }
        if coverage_str.contains(&format!("COV={}", 263)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 263);
        }
        if coverage_str.contains(&format!("COV={}", 264)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 264);
        }
        if coverage_str.contains(&format!("COV={}", 265)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 265);
        }
        if coverage_str.contains(&format!("COV={}", 266)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 266);
        }
        if coverage_str.contains(&format!("COV={}", 267)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 267);
        }
        if coverage_str.contains(&format!("COV={}", 268)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 268);
        }
        if coverage_str.contains(&format!("COV={}", 269)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 269);
        }
        if coverage_str.contains(&format!("COV={}", 270)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 270);
        }
        if coverage_str.contains(&format!("COV={}", 271)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 271);
        }
        if coverage_str.contains(&format!("COV={}", 272)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 272);
        }
        if coverage_str.contains(&format!("COV={}", 273)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 273);
        }
        if coverage_str.contains(&format!("COV={}", 274)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 274);
        }
        if coverage_str.contains(&format!("COV={}", 275)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 275);
        }
        if coverage_str.contains(&format!("COV={}", 276)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 276);
        }
        if coverage_str.contains(&format!("COV={}", 277)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 277);
        }
        if coverage_str.contains(&format!("COV={}", 278)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 278);
        }
        if coverage_str.contains(&format!("COV={}", 279)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 279);
        }
        if coverage_str.contains(&format!("COV={}", 280)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 280);
        }
        if coverage_str.contains(&format!("COV={}", 281)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 281);
        }
        if coverage_str.contains(&format!("COV={}", 282)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 282);
        }
        if coverage_str.contains(&format!("COV={}", 283)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 283);
        }
        if coverage_str.contains(&format!("COV={}", 284)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 284);
        }
        if coverage_str.contains(&format!("COV={}", 285)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 285);
        }
        if coverage_str.contains(&format!("COV={}", 286)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 286);
        }
        if coverage_str.contains(&format!("COV={}", 287)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 287);
        }
        if coverage_str.contains(&format!("COV={}", 288)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 288);
        }
        if coverage_str.contains(&format!("COV={}", 289)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 289);
        }
        if coverage_str.contains(&format!("COV={}", 290)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 290);
        }
        if coverage_str.contains(&format!("COV={}", 291)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 291);
        }
        if coverage_str.contains(&format!("COV={}", 292)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 292);
        }
        if coverage_str.contains(&format!("COV={}", 293)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 293);
        }
        if coverage_str.contains(&format!("COV={}", 294)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 294);
        }
        if coverage_str.contains(&format!("COV={}", 295)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 295);
        }
        if coverage_str.contains(&format!("COV={}", 296)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 296);
        }
        if coverage_str.contains(&format!("COV={}", 297)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 297);
        }
        if coverage_str.contains(&format!("COV={}", 298)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 298);
        }
        if coverage_str.contains(&format!("COV={}", 299)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 299);
        }
        if coverage_str.contains(&format!("COV={}", 300)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 300);
        }
        if coverage_str.contains(&format!("COV={}", 301)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 301);
        }
        if coverage_str.contains(&format!("COV={}", 302)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 302);
        }
        if coverage_str.contains(&format!("COV={}", 303)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 303);
        }
        if coverage_str.contains(&format!("COV={}", 304)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 304);
        }
        if coverage_str.contains(&format!("COV={}", 305)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 305);
        }
        if coverage_str.contains(&format!("COV={}", 306)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 306);
        }
        if coverage_str.contains(&format!("COV={}", 307)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 307);
        }
        if coverage_str.contains(&format!("COV={}", 308)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 308);
        }
        if coverage_str.contains(&format!("COV={}", 309)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 309);
        }
        if coverage_str.contains(&format!("COV={}", 310)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 310);
        }
        if coverage_str.contains(&format!("COV={}", 311)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 311);
        }
        if coverage_str.contains(&format!("COV={}", 312)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 312);
        }
        if coverage_str.contains(&format!("COV={}", 313)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 313);
        }
        if coverage_str.contains(&format!("COV={}", 314)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 314);
        }
        if coverage_str.contains(&format!("COV={}", 315)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 315);
        }
        if coverage_str.contains(&format!("COV={}", 316)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 316);
        }
        if coverage_str.contains(&format!("COV={}", 317)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 317);
        }
        if coverage_str.contains(&format!("COV={}", 318)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 318);
        }
        if coverage_str.contains(&format!("COV={}", 319)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 319);
        }
        if coverage_str.contains(&format!("COV={}", 320)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 320);
        }
        if coverage_str.contains(&format!("COV={}", 321)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 321);
        }
        if coverage_str.contains(&format!("COV={}", 322)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 322);
        }
        if coverage_str.contains(&format!("COV={}", 323)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 323);
        }
        if coverage_str.contains(&format!("COV={}", 324)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 324);
        }
        if coverage_str.contains(&format!("COV={}", 325)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 325);
        }
        if coverage_str.contains(&format!("COV={}", 326)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 326);
        }
        if coverage_str.contains(&format!("COV={}", 327)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 327);
        }
        if coverage_str.contains(&format!("COV={}", 328)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 328);
        }
        if coverage_str.contains(&format!("COV={}", 329)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 329);
        }
        if coverage_str.contains(&format!("COV={}", 330)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 330);
        }
        if coverage_str.contains(&format!("COV={}", 331)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 331);
        }
        if coverage_str.contains(&format!("COV={}", 332)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 332);
        }
        if coverage_str.contains(&format!("COV={}", 333)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 333);
        }
        if coverage_str.contains(&format!("COV={}", 334)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 334);
        }
        if coverage_str.contains(&format!("COV={}", 335)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 335);
        }
        if coverage_str.contains(&format!("COV={}", 336)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 336);
        }
        if coverage_str.contains(&format!("COV={}", 337)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 337);
        }
        if coverage_str.contains(&format!("COV={}", 338)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 338);
        }
        if coverage_str.contains(&format!("COV={}", 339)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 339);
        }
        if coverage_str.contains(&format!("COV={}", 340)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 340);
        }
        if coverage_str.contains(&format!("COV={}", 341)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 341);
        }
        if coverage_str.contains(&format!("COV={}", 342)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 342);
        }
        if coverage_str.contains(&format!("COV={}", 343)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 343);
        }
        if coverage_str.contains(&format!("COV={}", 344)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 344);
        }
        if coverage_str.contains(&format!("COV={}", 345)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 345);
        }
        if coverage_str.contains(&format!("COV={}", 346)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 346);
        }
        if coverage_str.contains(&format!("COV={}", 347)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 347);
        }
        if coverage_str.contains(&format!("COV={}", 348)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 348);
        }
        if coverage_str.contains(&format!("COV={}", 349)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 349);
        }
        if coverage_str.contains(&format!("COV={}", 350)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 350);
        }
        if coverage_str.contains(&format!("COV={}", 351)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 351);
        }
        if coverage_str.contains(&format!("COV={}", 352)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 352);
        }
        if coverage_str.contains(&format!("COV={}", 353)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 353);
        }
        if coverage_str.contains(&format!("COV={}", 354)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 354);
        }
        if coverage_str.contains(&format!("COV={}", 355)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 355);
        }
        if coverage_str.contains(&format!("COV={}", 356)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 356);
        }
        if coverage_str.contains(&format!("COV={}", 357)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 357);
        }
        if coverage_str.contains(&format!("COV={}", 358)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 358);
        }
        if coverage_str.contains(&format!("COV={}", 359)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 359);
        }
        if coverage_str.contains(&format!("COV={}", 360)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 360);
        }
        if coverage_str.contains(&format!("COV={}", 361)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 361);
        }
        if coverage_str.contains(&format!("COV={}", 362)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 362);
        }
        if coverage_str.contains(&format!("COV={}", 363)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 363);
        }
        if coverage_str.contains(&format!("COV={}", 364)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 364);
        }
        if coverage_str.contains(&format!("COV={}", 365)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 365);
        }
        if coverage_str.contains(&format!("COV={}", 366)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 366);
        }
        if coverage_str.contains(&format!("COV={}", 367)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 367);
        }
        if coverage_str.contains(&format!("COV={}", 368)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 368);
        }
        if coverage_str.contains(&format!("COV={}", 369)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 369);
        }
        if coverage_str.contains(&format!("COV={}", 370)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 370);
        }
        if coverage_str.contains(&format!("COV={}", 371)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 371);
        }
        if coverage_str.contains(&format!("COV={}", 372)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 372);
        }
        if coverage_str.contains(&format!("COV={}", 373)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 373);
        }
        if coverage_str.contains(&format!("COV={}", 374)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 374);
        }
        if coverage_str.contains(&format!("COV={}", 375)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 375);
        }
        if coverage_str.contains(&format!("COV={}", 376)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 376);
        }
        if coverage_str.contains(&format!("COV={}", 377)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 377);
        }
        if coverage_str.contains(&format!("COV={}", 378)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 378);
        }
        if coverage_str.contains(&format!("COV={}", 379)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 379);
        }
        if coverage_str.contains(&format!("COV={}", 380)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 380);
        }
        if coverage_str.contains(&format!("COV={}", 381)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 381);
        }
        if coverage_str.contains(&format!("COV={}", 382)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 382);
        }
        if coverage_str.contains(&format!("COV={}", 383)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 383);
        }
        if coverage_str.contains(&format!("COV={}", 384)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 384);
        }
        if coverage_str.contains(&format!("COV={}", 385)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 385);
        }
        if coverage_str.contains(&format!("COV={}", 386)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 386);
        }
        if coverage_str.contains(&format!("COV={}", 387)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 387);
        }
        if coverage_str.contains(&format!("COV={}", 388)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 388);
        }
        if coverage_str.contains(&format!("COV={}", 389)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 389);
        }
        if coverage_str.contains(&format!("COV={}", 390)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 390);
        }
        if coverage_str.contains(&format!("COV={}", 391)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 391);
        }
        if coverage_str.contains(&format!("COV={}", 392)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 392);
        }
        if coverage_str.contains(&format!("COV={}", 393)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 393);
        }
        if coverage_str.contains(&format!("COV={}", 394)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 394);
        }
        if coverage_str.contains(&format!("COV={}", 395)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 395);
        }
        if coverage_str.contains(&format!("COV={}", 396)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 396);
        }
        if coverage_str.contains(&format!("COV={}", 397)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 397);
        }
        if coverage_str.contains(&format!("COV={}", 398)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 398);
        }
        if coverage_str.contains(&format!("COV={}", 399)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 399);
        }
        if coverage_str.contains(&format!("COV={}", 400)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 400);
        }
        if coverage_str.contains(&format!("COV={}", 401)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 401);
        }
        if coverage_str.contains(&format!("COV={}", 402)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 402);
        }
        if coverage_str.contains(&format!("COV={}", 403)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 403);
        }
        if coverage_str.contains(&format!("COV={}", 404)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 404);
        }
        if coverage_str.contains(&format!("COV={}", 405)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 405);
        }
        if coverage_str.contains(&format!("COV={}", 406)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 406);
        }
        if coverage_str.contains(&format!("COV={}", 407)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 407);
        }
        if coverage_str.contains(&format!("COV={}", 408)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 408);
        }
        if coverage_str.contains(&format!("COV={}", 409)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 409);
        }
        if coverage_str.contains(&format!("COV={}", 410)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 410);
        }
        if coverage_str.contains(&format!("COV={}", 411)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 411);
        }
        if coverage_str.contains(&format!("COV={}", 412)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 412);
        }
        if coverage_str.contains(&format!("COV={}", 413)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 413);
        }
        if coverage_str.contains(&format!("COV={}", 414)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 414);
        }
        if coverage_str.contains(&format!("COV={}", 415)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 415);
        }
        if coverage_str.contains(&format!("COV={}", 416)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 416);
        }
        if coverage_str.contains(&format!("COV={}", 417)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 417);
        }
        if coverage_str.contains(&format!("COV={}", 418)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 418);
        }
        if coverage_str.contains(&format!("COV={}", 419)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 419);
        }
        if coverage_str.contains(&format!("COV={}", 420)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 420);
        }
        if coverage_str.contains(&format!("COV={}", 421)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 421);
        }
        if coverage_str.contains(&format!("COV={}", 422)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 422);
        }
        if coverage_str.contains(&format!("COV={}", 423)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 423);
        }
        if coverage_str.contains(&format!("COV={}", 424)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 424);
        }
        if coverage_str.contains(&format!("COV={}", 425)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 425);
        }
        if coverage_str.contains(&format!("COV={}", 426)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 426);
        }
        if coverage_str.contains(&format!("COV={}", 427)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 427);
        }
        if coverage_str.contains(&format!("COV={}", 428)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 428);
        }
        if coverage_str.contains(&format!("COV={}", 429)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 429);
        }
        if coverage_str.contains(&format!("COV={}", 430)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 430);
        }
        if coverage_str.contains(&format!("COV={}", 431)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 431);
        }
        if coverage_str.contains(&format!("COV={}", 432)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 432);
        }
        if coverage_str.contains(&format!("COV={}", 433)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 433);
        }
        if coverage_str.contains(&format!("COV={}", 434)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 434);
        }
        if coverage_str.contains(&format!("COV={}", 435)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 435);
        }
        if coverage_str.contains(&format!("COV={}", 436)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 436);
        }
        if coverage_str.contains(&format!("COV={}", 437)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 437);
        }
        if coverage_str.contains(&format!("COV={}", 438)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 438);
        }
        if coverage_str.contains(&format!("COV={}", 439)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 439);
        }
        if coverage_str.contains(&format!("COV={}", 440)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 440);
        }
        if coverage_str.contains(&format!("COV={}", 441)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 441);
        }
        if coverage_str.contains(&format!("COV={}", 442)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 442);
        }
        if coverage_str.contains(&format!("COV={}", 443)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 443);
        }
        if coverage_str.contains(&format!("COV={}", 444)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 444);
        }
        if coverage_str.contains(&format!("COV={}", 445)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 445);
        }
        if coverage_str.contains(&format!("COV={}", 446)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 446);
        }
        if coverage_str.contains(&format!("COV={}", 447)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 447);
        }
        if coverage_str.contains(&format!("COV={}", 448)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 448);
        }
        if coverage_str.contains(&format!("COV={}", 449)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 449);
        }
        if coverage_str.contains(&format!("COV={}", 450)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 450);
        }
        if coverage_str.contains(&format!("COV={}", 451)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 451);
        }
        if coverage_str.contains(&format!("COV={}", 452)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 452);
        }
        if coverage_str.contains(&format!("COV={}", 453)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 453);
        }
        if coverage_str.contains(&format!("COV={}", 454)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 454);
        }
        if coverage_str.contains(&format!("COV={}", 455)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 455);
        }
        if coverage_str.contains(&format!("COV={}", 456)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 456);
        }
        if coverage_str.contains(&format!("COV={}", 457)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 457);
        }
        if coverage_str.contains(&format!("COV={}", 458)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 458);
        }
        if coverage_str.contains(&format!("COV={}", 459)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 459);
        }
        if coverage_str.contains(&format!("COV={}", 460)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 460);
        }
        if coverage_str.contains(&format!("COV={}", 461)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 461);
        }
        if coverage_str.contains(&format!("COV={}", 462)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 462);
        }
        if coverage_str.contains(&format!("COV={}", 463)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 463);
        }
        if coverage_str.contains(&format!("COV={}", 464)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 464);
        }
        if coverage_str.contains(&format!("COV={}", 465)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 465);
        }
        if coverage_str.contains(&format!("COV={}", 466)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 466);
        }
        if coverage_str.contains(&format!("COV={}", 467)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 467);
        }
        if coverage_str.contains(&format!("COV={}", 468)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 468);
        }
        if coverage_str.contains(&format!("COV={}", 469)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 469);
        }
        if coverage_str.contains(&format!("COV={}", 470)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 470);
        }
        if coverage_str.contains(&format!("COV={}", 471)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 471);
        }
        if coverage_str.contains(&format!("COV={}", 472)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 472);
        }
        if coverage_str.contains(&format!("COV={}", 473)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 473);
        }
        if coverage_str.contains(&format!("COV={}", 474)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 474);
        }
        if coverage_str.contains(&format!("COV={}", 475)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 475);
        }
        if coverage_str.contains(&format!("COV={}", 476)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 476);
        }
        if coverage_str.contains(&format!("COV={}", 477)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 477);
        }
        if coverage_str.contains(&format!("COV={}", 478)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 478);
        }
        if coverage_str.contains(&format!("COV={}", 479)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 479);
        }
        if coverage_str.contains(&format!("COV={}", 480)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 480);
        }
        if coverage_str.contains(&format!("COV={}", 481)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 481);
        }
        if coverage_str.contains(&format!("COV={}", 482)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 482);
        }
        if coverage_str.contains(&format!("COV={}", 483)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 483);
        }
        if coverage_str.contains(&format!("COV={}", 484)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 484);
        }
        if coverage_str.contains(&format!("COV={}", 485)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 485);
        }
        if coverage_str.contains(&format!("COV={}", 486)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 486);
        }
        if coverage_str.contains(&format!("COV={}", 487)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 487);
        }
        if coverage_str.contains(&format!("COV={}", 488)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 488);
        }
        if coverage_str.contains(&format!("COV={}", 489)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 489);
        }
        if coverage_str.contains(&format!("COV={}", 490)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 490);
        }
        if coverage_str.contains(&format!("COV={}", 491)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 491);
        }
        if coverage_str.contains(&format!("COV={}", 492)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 492);
        }
        if coverage_str.contains(&format!("COV={}", 493)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 493);
        }
        if coverage_str.contains(&format!("COV={}", 494)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 494);
        }
        if coverage_str.contains(&format!("COV={}", 495)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 495);
        }
        if coverage_str.contains(&format!("COV={}", 496)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 496);
        }
        if coverage_str.contains(&format!("COV={}", 497)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 497);
        }
        if coverage_str.contains(&format!("COV={}", 498)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 498);
        }
        if coverage_str.contains(&format!("COV={}", 499)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 499);
        }
        if coverage_str.contains(&format!("COV={}", 500)) {
            let _ = 1 + 1;
            println!("We've passed {:?}", 500);
        }


        #[cfg(not(fuzzing))]
        for line in coverage_str.lines() {
            if line.starts_with("COV=") {
                println!("We found the coverage for the line: {:?}", line);
            }
        }
        // For each call, we verify that invariants aren't broken
        if !invariant_manager.are_invariants_passing() {
            panic!("Invariant triggered!");
        }
    })
}
