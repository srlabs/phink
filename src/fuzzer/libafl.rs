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
use crate::fuzzer::ziggy::ZiggyFuzzer;
use frame_support::traits::Len;
use libafl::corpus::Corpus;
#[cfg(feature = "tui")]
use libafl::monitors::tui::{ui::TuiUI, TuiMonitor};
#[cfg(not(feature = "tui"))]
use libafl::monitors::SimpleMonitor;
use libafl::mutators::havoc_mutations;
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
    Evaluator,
};
use libafl_bolts::rands::RandomSeed;
use libafl_bolts::{rands::StdRand, tuples::tuple_list, AsSlice};
use pallet_contracts::ExecReturnValue;
use parity_scale_codec::Encode;
use sp_runtime::DispatchError;
use std::{path::Path, sync::Mutex};
use std::{path::PathBuf, ptr::write};

#[derive(Clone)]
pub struct LibAFLFuzzer {
    setup: ContractBridge,
}

/// Coverage map with explicit assignments due to the lack of instrumentation
static mut SIGNALS: [u8; 255] = [0; 255];
static mut SIGNALS_PTR: *mut u8 = unsafe { SIGNALS.as_mut_ptr() };

/// Assign a signal to the signals map
fn coverage(idx: usize) {
    unsafe { write(SIGNALS_PTR.add(idx), 1) };
}

impl LibAFLFuzzer {
    pub fn new(setup: ContractBridge) -> LibAFLFuzzer {
        LibAFLFuzzer { setup }
    }


}

impl FuzzerEngine for LibAFLFuzzer {
    /// This is the main fuzzing function. Here, we fuzz ink!, and the planet
    #[no_mangle]
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
            harness(self.clone(), transcoder_loader, selectors, invariant_manager, input)
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
        // In case the corpus is empty (i.e. on first run), load existing test cases from on-disk
        // corpus
        let corpus_dirs = vec![PathBuf::from("./output/phink/corpus")];

        if state.corpus().count() < 1 {
            state
                .load_initial_inputs(&mut fuzzer, &mut executor, &mut mgr, &corpus_dirs)
                .unwrap();
        }

        // Setup a mutational stage with a basic bytes mutator
        let mutator = StdScheduledMutator::new(havoc_mutations());
        let mut stages = tuple_list!(StdMutationalStage::new(mutator));
        fuzzer
            .fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)
            .unwrap()
    }
}

fn harness(
    client: LibAFLFuzzer,
    transcoder_loader: Mutex<ContractMessageTranscoder>,
    selectors: Vec<Selector>,
    invariant_manager: Invariants,
    input: &BytesInput,
) -> ExitKind {
    let target = input.target_bytes();
    let payload = target.as_slice();
    coverage(1);
    let binding = client.clone();
    let raw_call = binding.parse_args(payload, selectors.clone());
    coverage(2);
    if raw_call.is_none() {
        return ExitKind::Ok;
    }
    coverage(3);
    let call = raw_call.expect("`raw_call` wasn't `None`; QED");
    match ZiggyFuzzer::create_call(call.0, call.1) {
        // Successfully encoded
        Some(full_call) => {
            coverage(3);

            let decoded_msg = transcoder_loader
                .lock()
                .unwrap()
                .decode_contract_message(&mut &*full_call);
            coverage(4);

            if let Err(_) = decoded_msg {
                return ExitKind::Ok;
            }
            coverage(5);
            let mut chain = BasicExternalities::new(client.setup.genesis.clone());
            chain.execute_with(|| {
                //todo
                client.timestamp();
                let result = client.setup.clone().call(&full_call);
                coverage(5);

                let mut i = 6;
                #[cfg(not(feature = "tui"))]
                println!("Events -- {:?}", result.events.len());

                for event in result.events.unwrap_or_default() {
                    #[cfg(not(feature = "tui"))]

                    println!("{:?}", event);
                    i += 1;
                    coverage(i);
                }

                // We pretty-print all information that we need to debug
                #[cfg(not(feature = "tui"))]
                client.pretty_print(result.result, decoded_msg.unwrap().to_string(), full_call);

                // For each call, we verify that invariants aren't broken
                if !invariant_manager.are_invariants_passing() {
                    panic!("Invariant triggered! ")
                }
            });
        }

        None => return ExitKind::Ok,
    }

    ExitKind::Ok
}