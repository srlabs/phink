use prettytable::{row, Table};

use contract_transcode::ContractMessageTranscoder;
use frame_support::{
    __private::BasicExternalities,
    traits::{OnFinalize, OnInitialize},
};

use crate::contract::payload::{PayloadCrafter, Selector};
use crate::contract::remote::ContractBridge;
use crate::contract::runtime::{
    AllPalletsWithSystem, BlockNumber, RuntimeOrigin, Timestamp, SLOT_DURATION,
};
use crate::fuzzer::engine::FuzzerEngine;
use crate::fuzzer::invariants::Invariants;
use pallet_contracts::ExecReturnValue;
use parity_scale_codec::Encode;
use sp_runtime::DispatchError;
use std::{path::Path, sync::Mutex};
use std::{path::PathBuf, ptr::write};

use crate::fuzzer::ziggy::ZiggyFuzzer;
#[cfg(feature = "tui")]
use libafl::monitors::tui::{ui::TuiUI, TuiMonitor};
#[cfg(not(feature = "tui"))]
use libafl::monitors::SimpleMonitor;
use libafl::mutators::havoc_mutations;
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
    Evaluator,
};
use libafl_bolts::rands::RandomSeed;
use libafl_bolts::{rands::StdRand, tuples::tuple_list, AsSlice};

#[derive(Clone)]
pub struct LibAFLFuzzer {
    setup: ContractBridge,
}

/// Coverage map with explicit assignments due to the lack of instrumentation
static mut SIGNALS: [u8; 64] = [0; 64];
static mut SIGNALS_PTR: *mut u8 = unsafe { SIGNALS.as_mut_ptr() };

/// Assign a signal to the signals map
fn signals_set(idx: usize) {
    unsafe { write(SIGNALS_PTR.add(idx), 1) };
}
impl LibAFLFuzzer {
    fn new(setup: ContractBridge) -> crate::fuzzer::libafl::LibAFLFuzzer {
        Self { setup }
    }
}
impl FuzzerEngine for LibAFLFuzzer {
    /// This is the main fuzzing function. Here, we fuzz ink!, and the planet
    fn fuzz(self) {
        let transcoder_loader = Mutex::new(
            ContractMessageTranscoder::load(Path::new(&self.setup.path_to_specs)).unwrap(),
        );

        let specs = &self.setup.json_specs;
        let selectors: Vec<Selector> = PayloadCrafter::extract_all(specs);
        let invariants = PayloadCrafter::extract_invariants(specs);
        let invariant_manager: Invariants = Invariants::from(invariants, self.setup.clone());

        // The closure that we want to fuzz
        let mut harness = |input: &BytesInput| {
            let target = input.target_bytes();
            let payload = target.as_slice();
            let binding = self.clone();
            let raw_call = binding.parse_args(payload, selectors.clone());
            if raw_call.is_none() {
                return ExitKind::Ok;
            }
            let call = raw_call.expect("`raw_call` wasn't `None`; QED");

            match ZiggyFuzzer::create_call(call.0, call.1) {
                // Successfully encoded
                Some(full_call) => {
                    let decoded_msg = transcoder_loader
                        .lock()
                        .unwrap()
                        .decode_contract_message(&mut &*full_call);
                    if let Err(_) = decoded_msg {
                        return ExitKind::Ok;
                    }
                    let mut chain = BasicExternalities::new(self.setup.genesis.clone());
                    chain.execute_with(|| {
                        //todo
                        Self::timestamp();
                        let result = self.setup.clone().call(&full_call);

                        // We pretty-print all information that we need to debug
                        #[cfg(not(fuzzing))]
                        Self::pretty_print(result, decoded_msg.unwrap().to_string(), full_call);

                        // For each call, we verify that invariants aren't broken
                        if !invariant_manager.are_invariants_passing() {
                            panic!("Invariant triggered! ")
                        }
                    });
                }

                // Encoding failed, we try again
                None => return ExitKind::Ok,
            }
            //signals_set(i);

            ExitKind::Ok
        };

        // Create an observation channel using the signals map
        let observer =
            unsafe { StdMapObserver::from_mut_ptr("signals", SIGNALS_PTR, SIGNALS.len()) };

        // Feedback to rate the interestingness of an input
        let mut feedback = MaxMapFeedback::new(&observer);

        // A feedback to choose if an input is a solution or not
        let mut objective = CrashFeedback::new();

        // create a State from scratch
        let mut state = StdState::new(
            // RNG
            StdRand::new(),
            // Corpus that will be evolved, we keep it in memory for performance
            InMemoryCorpus::new(),
            // Corpus in which we store solutions (crashes in this example),
            // on disk so the user can get them after stopping the fuzzer
            OnDiskCorpus::new(PathBuf::from("./crashes")).unwrap(),
            // States of the feedbacks.
            // The feedbacks can report the data that should persist in the State.
            &mut feedback,
            // Same for objective feedbacks
            &mut objective,
        )
        .unwrap();

        // The Monitor trait define how the fuzzer stats are displayed to the user
        #[cfg(not(feature = "tui"))]
        let mon = SimpleMonitor::new(|s| println!("{s}"));
        #[cfg(feature = "tui")]
        let ui = TuiUI::with_version(String::from("Baby Fuzzer"), String::from("0.0.1"), false);
        #[cfg(feature = "tui")]
        let mon = TuiMonitor::new(ui);

        // The event manager handle the various events generated during the fuzzing loop
        // such as the notification of the addition of a new item to the corpus
        let mut mgr = SimpleEventManager::new(mon);

        // A queue policy to get testcasess from the corpus
        let scheduler = QueueScheduler::new();

        // A fuzzer with feedbacks and a corpus scheduler
        let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

        // Create the executor for an in-process function with just one observer
        let mut executor = InProcessExecutor::new(
            &mut harness,
            tuple_list!(observer),
            &mut fuzzer,
            &mut state,
            &mut mgr,
        )
        .expect("Failed to create the Executor");

        // Generate 8 initial inputs
        fuzzer
            .evaluate_input(
                &mut state,
                &mut executor,
                &mut mgr,
                BytesInput::new(vec![b'a']),
            )
            .unwrap();

        // Setup a mutational stage with a basic bytes mutator
        let mutator = StdScheduledMutator::new(havoc_mutations());
        let mut stages = tuple_list!(StdMutationalStage::new(mutator));
        fuzzer
            .fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)
            .expect("Error in the fuzzing loop");
    }
}
