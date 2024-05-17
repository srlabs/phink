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
use frame_support::traits::Len;
use itertools::Itertools;
use libafl::corpus::Corpus;
use libafl::inputs::{GeneralizedInputMetadata, HasBytesVec};
#[cfg(feature = "tui")]
use libafl::monitors::tui::{ui::TuiUI, TuiMonitor};
#[cfg(not(feature = "tui"))]
use libafl::monitors::SimpleMonitor;
use libafl::mutators::{havoc_mutations, GrimoireStringReplacementMutator, Mutator};
use libafl::prelude::HasRand;
use libafl::prelude::*;
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
use libafl_bolts::{rands::StdRand, tuples::tuple_list, AsSlice, Error, Named};
use pallet_contracts::ExecReturnValue;
use parity_scale_codec::Encode;
use sp_runtime::DispatchError;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::{fs, path::Path, sync::Mutex};
use std::{path::PathBuf, ptr::write};

#[derive(Clone)]
pub struct LibAFLFuzzer {
    setup: ContractBridge,
}

/// Coverage map with explicit assignments due to the lack of instrumentation
static mut SIGNALS: [u16; 65535] = [0; 65535];
static mut SIGNALS_PTR: *mut u16 = unsafe { SIGNALS.as_mut_ptr() };

/// Assign a signal to the signals map
fn coverage(idx: u16) {
    unsafe { write(SIGNALS_PTR.add(idx as usize), 1) };
}

impl LibAFLFuzzer {
    pub fn new(setup: ContractBridge) -> LibAFLFuzzer {
        LibAFLFuzzer { setup }
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
    #[no_mangle]
    fn fuzz(self) {
        let mut transcoder_loader = Mutex::new(
            ContractMessageTranscoder::load(Path::new(&self.setup.path_to_specs)).unwrap(),
        );

        let specs = &self.setup.json_specs;
        let mut selectors: Vec<Selector> = PayloadCrafter::extract_all(specs);
        let inv = PayloadCrafter::extract_invariants(specs)
            .expect("No invariants found, check your contract");

        let mut invariant_manager = Invariants::from(inv, self.setup.clone());

        {
            // Create the directory if it doesn't exist
            fs::create_dir_all("./output/phink/corpus").unwrap();

            // Iterate over the selectors and write each one to a separate file
            for (i, selector) in selectors.iter().enumerate() {
                let file_path = format!("{}/selector_{}.bin", "./output/phink/corpus", i);
                let mut file = File::create(&file_path).unwrap();

                // Write the u8 array to the file
                file.write_all(selector).unwrap();
            }
        }

        let mut harness = |input: &BytesInput| {
            harness(
                self.clone(),
                &mut transcoder_loader,
                &mut selectors.clone(),
                &mut invariant_manager,
                input,
            )
        };

        // Create an observation channel using the signals map
        let observer =
            unsafe { StdMapObserver::from_mut_ptr("signals", SIGNALS_PTR, SIGNALS.len()) };

        // Feedback to rate the interestingness of an input
        let mut feedback = MaxMapFeedback::new(&observer);

        // A feedback to choose if an input is a solution or not
        let mut objective = CrashFeedback::new();
        let corpus_dirs = vec![PathBuf::from("./output/phink/corpus_old")];

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

        #[cfg(not(feature = "tui"))]
        let mon = SimpleMonitor::new(|s| println!("{s}"));
        #[cfg(feature = "tui")]
        let ui = TuiUI::with_version(String::from("Phink"), String::from("0.1"), true);
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

        // In case the corpus is empty (i.e. on first run), load existing test cases from on-disk
        // corpus

        if state.corpus().count() < 1 {
            state
                .load_initial_inputs(&mut fuzzer, &mut executor, &mut mgr, &corpus_dirs)
                .unwrap();
        }

        println!("Selectors {:?}", selectors);
        // let mutator = StdMutationalStage::new(StdScheduledMutator::new(ValidSelectorMutator::new(
        //     selectors.clone(),
        // )));
        //
        // let mut2 = StdMutationalStage::new(StdScheduledMutator::new(havoc_mutations()));
        //
        // let mut stages = tuple_list!(mutator, mut2);

        let mut stages = tuple_list!(StdMutationalStage::new(StdScheduledMutator::new(
            havoc_mutations()
        )));

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
    coverage(1);
    let binding = client.clone();
    let raw_call = binding.parse_args(payload, selectors.clone());
    coverage(2);
    if raw_call.is_none() {
        return ExitKind::Ok;
    }
    coverage(3);
    let call = raw_call.expect("`raw_call` wasn't `None`");
    let mut chain = BasicExternalities::new(client.setup.genesis.clone());
    chain.execute_with(|| {
        <LibAFLFuzzer as FuzzerEngine>::timestamp();
    });

    match <LibAFLFuzzer as FuzzerEngine>::create_call(call.0, call.1) {
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
            chain.execute_with(|| {
                let result = client.setup.clone().call(&full_call);

                coverage(5);

                let mut i = 6;

                let mut coverage_str =
                    deduplicate(&*String::from_utf8_lossy(&*result.debug_message));

                for line in coverage_str.lines() {
                    if line.starts_with("COV=") {
                        #[cfg(not(feature = "tui"))]
                        println!("We found the coverage for the line: {:?}", line);
                        i += 1;
                        coverage(i);
                    }
                }

                // We pretty-print all information that we need to debug
                #[cfg(not(feature = "tui"))]
                <LibAFLFuzzer as FuzzerEngine>::pretty_print(
                    result.result.clone(),
                    decoded_msg.unwrap().to_string(),
                    full_call,
                );

                // For each call, we verify that invariants aren't broken
                if !invariant_manager.are_invariants_passing() {
                    panic!("Invariant triggered!")
                }
            });
        }

        None => return ExitKind::Ok,
    }

    ExitKind::Ok
}

/// A simple helper to remove some duplicated lines from a `&str`
/// This is used mainly to remove coverage returns being inserted many times in the debug vector
/// in case of any `iter()`, `for` loop and so on
/// # Arguments
/// * `input`: The string to deduplicate

fn deduplicate(input: &str) -> String {
    let mut unique_lines = HashSet::new();
    input
        .lines()
        .filter(|&line| unique_lines.insert(line))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_deduplicate() {
        // Test case: input with duplicate lines
        let input = "line1\nline2\nline1\nline3\nline2";
        let expected = "line1\nline2\nline3";
        let cow_input = Cow::Borrowed(input);
        assert_eq!(deduplicate(&cow_input), Cow::Owned(expected.to_string()));

        // Test case: input without duplicate lines
        let input = "line1\nline2\nline3";
        let expected = "line1\nline2\nline3";
        let cow_input = Cow::Borrowed(input);
        assert_eq!(deduplicate(&cow_input), Cow::Owned(expected.to_string()));

        // Test case: empty input
        let input = "";
        let expected = "";
        let cow_input = Cow::Borrowed(input);
        assert_eq!(deduplicate(&cow_input), Cow::Owned(expected.to_string()));

        // Test case: input with consecutive duplicate lines
        let input = "line1\nline1\nline2\nline2\nline3\nline3";
        let expected = "line1\nline2\nline3";
        let cow_input = Cow::Borrowed(input);
        assert_eq!(deduplicate(&cow_input), Cow::Owned(expected.to_string()));

        // Test case: input with non-consecutive duplicate lines
        let input = "line1\nline2\nline3\nline1\nline2";
        let expected = "line1\nline2\nline3";
        let cow_input = Cow::Borrowed(input);
        assert_eq!(deduplicate(&cow_input), Cow::Owned(expected.to_string()));
    }
}
