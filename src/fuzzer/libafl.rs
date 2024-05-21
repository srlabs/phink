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
use parity_scale_codec::Encode;
use std::borrow::Cow;
use std::fs::File;
use std::io::Write;
use std::{fs, path::Path, sync::Mutex};
use std::{path::PathBuf, ptr::write};

#[derive(Clone)]
pub struct LibAFLFuzzer {
    setup: ContractBridge,
}

/// Coverage map with explicit assignments due to the lack of instrumentation
static mut SIGNALS: [u8; 65535] = [0; 65535];
static mut SIGNALS_PTR: *mut u8 = unsafe { SIGNALS.as_mut_ptr() };
pub static mut IDX: u8 = 0;
/// Assign a signal to the signals map
/// We basically signal to LibAFL that coverage has been increased
pub fn inc_cov() {
    unsafe {
        IDX += 1;
        write(SIGNALS_PTR.add(IDX as usize), 1)
    };
}

pub fn reset_cov() {
    unsafe {
        IDX = 0;
    }
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

            // Write the u8 array to the file
            file.write_all(selector).unwrap();
        }
    }
}

struct ValidSelectorMutator {
    selectors: Vec<Selector>,
}

impl ValidSelectorMutator {
    pub fn new(selectors: Vec<Selector>) -> Self {
        Self { selectors }
    }
}

impl<S> Mutator<BytesInput, S> for ValidSelectorMutator
where
    S: HasRand,
{
    fn mutate(&mut self, state: &mut S, input: &mut BytesInput) -> Result<MutationResult, Error> {
        let data = input.bytes_mut();

        if data.len() < 4 {
            // Ensure we have at least 4 bytes to mutate
            return Ok(MutationResult::Skipped);
        }

        // Choose a random selector from the provided selectors
        let selector = state.rand_mut().choose(&self.selectors);

        // Replace the first 4 bytes of the input with the chosen selector
        data[0..4].copy_from_slice(&selector[..4]);
        println!("{:?}", data);
        Ok(MutationResult::Mutated)
    }
}

impl Named for ValidSelectorMutator {
    fn name(&self) -> &str {
        "ValidSelectorRandomPayloadMutator"
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

        let mut harness = |input: &BytesInput| {
            let exec: ExitKind = harness(
                self.clone(),
                &mut transcoder_loader,
                &mut selectors.clone(),
                &mut invariant_manager,
                input,
            );
            reset_cov();
            return exec;
        };

        let observer =
            unsafe { StdMapObserver::from_mut_ptr("signals", SIGNALS_PTR, SIGNALS.len()) };

        let mut feedback = MaxMapFeedback::new(&observer);

        let mut objective = CrashFeedback::new();
        let corpus_dirs = vec![PathBuf::from("./output/phink/corpus")];

        let mut state = StdState::new(
            StdRand::new(),
            InMemoryCorpus::new(),
            OnDiskCorpus::new(PathBuf::from("./crashes")).unwrap(),
            &mut feedback,
            &mut objective,
        )
        .unwrap();

        #[cfg(not(feature = "tui"))]
        let mon = SimpleMonitor::new(|s| println!("{s}"));
        #[cfg(feature = "tui")]
        let ui = TuiUI::with_version(String::from("Phink"), String::from("0.1"), true);
        #[cfg(feature = "tui")]
        let mon = TuiMonitor::new(ui);

        let mut mgr = SimpleEventManager::new(mon);

        let mutator = StdMOptMutator::new(
            &mut state,
            havoc_mutations().merge(tokens_mutations()),
            7,
            5,
        )
        .unwrap();

        let power = StdMutationalStage::new(mutator);

        let scheduler = QueueScheduler::new();

        let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

        let mut executor = InProcessExecutor::new(
            &mut harness,
            tuple_list!(observer),
            &mut fuzzer,
            &mut state,
            &mut mgr,
        )
        .expect("Failed to create the Executor");

        if state.corpus().count() < 1 {
            state
                .load_initial_inputs_forced(&mut fuzzer, &mut executor, &mut mgr, &corpus_dirs)
                .unwrap();
        }

        println!("Selectors {:?}", selectors);

        let mut stages = tuple_list!(power);

        fuzzer
            .fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)
            .unwrap()
    }
}

fn harness(
    client: LibAFLFuzzer,
    transcoder_loader: &mut Mutex<ContractMessageTranscoder>,
    selectors: &mut Vec<Selector>,
    invariant_manager: &mut Invariants,
    input: &BytesInput,
) -> ExitKind {
    let target = input.target_bytes();
    let payload = target.as_slice();
    inc_cov();
    let binding = client.clone();
    let raw_call = binding.parse_args(payload, selectors.clone());
    inc_cov();
    if raw_call.is_none() {
        return ExitKind::Ok;
    }
    inc_cov();
    let call = raw_call.expect("`raw_call` wasn't `None`");
    let mut chain = BasicExternalities::new(client.setup.genesis.clone()).execute_with(<LibAFLFuzzer as FuzzerEngine>::timestamp());


    let mut calls: Vec<Vec<u8>> = Vec::new();

    for i in 0..3 {
        let one_call = <LibAFLFuzzer as FuzzerEngine>::create_call(call.0, call.1);
        if one_call.is_none() {
            return ExitKind::Ok;
        }
        calls.push(one_call.unwrap());
    }

    inc_cov();

    chain.execute_with(|| {
        let mut results: Vec<Vec<u8>> = Vec::new();
        for call in calls {
            let decoded_msg = transcoder_loader
                .lock()
                .unwrap()
                .decode_contract_message(&mut &*call.clone());
            inc_cov();

            if let Err(_) = decoded_msg {
                return ExitKind::Ok;
            }
            inc_cov();

            let result = client.setup.clone().call(&call.clone());
            results.push(result.debug_message);

            inc_cov();

            // We pretty-print all information that we need to debug
            #[cfg(not(feature = "tui"))]
            <LibAFLFuzzer as FuzzerEngine>::pretty_print(
                result.result.clone(),
                decoded_msg.unwrap().to_string(),
                call,
            );
        }

        let mut coverage_str = utils::deduplicate(&*String::from_utf8_lossy(
            &*results.into_iter().flatten().collect::<Vec<_>>(),
        ));
        for line in coverage_str.lines() {
            if line.starts_with("COV=") {
                #[cfg(not(feature = "tui"))]
                println!("We found the coverage for the line: {:?}", line);
                inc_cov();
            }
        }
        // For each call, we verify that invariants aren't broken
        if !invariant_manager.are_invariants_passing() {
            panic!("Invariant triggered!");
        } else {
            return ExitKind::Ok;
        }
    })
}
