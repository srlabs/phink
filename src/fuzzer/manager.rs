#![allow(unused_imports, unused_variables)]
use crate::{
    cli::config::Configuration,
    contract::{
        remote::{
            ContractSetup,
            FullContractResponse,
        },
        selectors::{
            database::SelectorDatabase,
            selector::Selector,
        },
    },
    cover::coverage::InputCoverage,
    fuzzer::parser::{
        Message,
        OneInput,
        Origin,
    },
};
use anyhow::{
    bail,
    Context,
};
use contract_transcode::ContractMessageTranscoder;
use ink_metadata::InkProject;
use sp_runtime::{
    DispatchError,
    ModuleError,
};
use std::{
    panic,
    path::Path,
    sync::{
        Arc,
        Mutex,
    },
};

#[derive(Clone)]
pub struct CampaignManager {
    setup: ContractSetup,
    database: SelectorDatabase,
    configuration: Configuration,
    transcoder: Arc<Mutex<ContractMessageTranscoder>>,
}

impl CampaignManager {
    pub fn new(
        database: SelectorDatabase,
        setup: ContractSetup,
        configuration: Configuration,
    ) -> anyhow::Result<Self> {
        let transcoder = Arc::new(Mutex::new(
            ContractMessageTranscoder::load(Path::new(&setup.path_to_specs))
                .context("Cannot instantiante the `ContractMessageTranscoder`")?,
        ));

        Ok(Self {
            setup,
            database: database.clone(),
            configuration,
            transcoder,
        })
    }

    pub fn config(&self) -> Configuration {
        self.configuration.clone()
    }

    pub fn database(&self) -> SelectorDatabase {
        self.database.clone()
    }

    pub fn transcoder(&self) -> Arc<Mutex<ContractMessageTranscoder>> {
        Arc::clone(&self.transcoder)
    }

    pub fn check_invariants(
        &self,
        all_msg_responses: &[FullContractResponse],
        decoded_msgs: &OneInput,
    ) {
        let first = decoded_msgs.messages[0].to_owned();

        // We print the details only when we don't fuzz, so when we run a seed for instance,
        // otherwise this will pollute the AFL logs
        #[cfg(not(fuzzing))]
        all_msg_responses
            .iter()
            .filter(|response| response.is_trapped())
            .for_each(|response| {
                self.display_trap(first.clone(), response.clone());
            });

        if let Ok(invariant_tested) = self.are_invariants_failing(first.origin) {
            self.display_invariant(
                all_msg_responses.to_vec(),
                decoded_msgs.to_owned(),
                invariant_tested,
            );
        }
    }

    pub fn display_trap(&self, message: Message, response: FullContractResponse) {
        println!("\n🤯 A trapped contract got caught! Let's dive into it");
        println!("\n🐛 IMPORTANT STACKTRACE : {}\n", response);
        println!("🎉 Find below the trace that caused that trapped contract");

        let input = OneInput {
            messages: vec![message.clone()],
            fuzz_option: self.configuration.should_fuzz_origin(),
        };
        input.pretty_print(vec![response.clone()]);

        // Artificially trigger a bug for AFL
        panic!("\n🫡  Job is done! Please, don't mind the backtrace below/above.\n\n");
    }

    pub fn display_invariant(
        &self,
        responses: Vec<FullContractResponse>,
        decoded_msg: OneInput,
        mut invariant_tested: Selector,
    ) {
        let hex = self
            .transcoder()
            .lock()
            .unwrap()
            .decode_contract_message(&mut &*invariant_tested.as_mut())
            .unwrap();

        #[cfg(not(fuzzing))]
        {
            println!("\n🤯 An invariant got caught! Let's dive into it");
            println!("\n🫵  This was caused by `{hex}`\n");
            println!("🎉 Find below the trace that caused that invariant");
            decoded_msg.pretty_print(responses);
        }
        // Artificially trigger a bug for AFL
        panic!("\n🫡   Job is done! Please, don't mind the backtrace below/above.\n\n");
    }

    /// This function aims to call every invariants via `invariant_selectors`.
    pub fn are_invariants_failing(&self, origin: Origin) -> anyhow::Result<Selector> {
        for invariant in &self.database.to_owned().invariants()? {
            let invariant_call: FullContractResponse = self.setup.clone().call(
                invariant.as_ref(),
                origin.into(),
                0,
                self.configuration.clone(),
            );
            if invariant_call.failed() {
                return Ok(invariant.clone());
            }
        }
        bail!("All invariants passed")
    }
}
