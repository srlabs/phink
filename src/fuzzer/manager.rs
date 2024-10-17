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
            database,
            configuration,
            transcoder,
        })
    }

    pub fn config(&self) -> Configuration {
        self.configuration.clone()
    }

    pub fn database(&self) -> &SelectorDatabase {
        &self.database
    }

    pub fn transcoder(&self) -> Arc<Mutex<ContractMessageTranscoder>> {
        Arc::clone(&self.transcoder)
    }

    pub fn check_invariants(
        &self,
        responses: &[FullContractResponse],
        decoded_msgs: &OneInput,
        catch_trapped_contract: bool,
    ) {
        let mut trapped = responses.iter().filter(|response| response.is_trapped());

        // We print the details only when we don't fuzz, so when we run a seed for instance
        #[cfg(not(fuzzing))]
        {
            trapped.clone().for_each(|response| {
                self.display_trap(decoded_msgs, response);
            });
        }
        if let Ok(invariant) = self.are_invariants_failing(decoded_msgs.messages[0].origin) {
            // TODO: We only try to run the invariants as the first message's origin here
            self.display_invariant(responses.to_vec(), decoded_msgs, invariant);
        }

        // If we are running the seeds or that we want the fuzzer to catch the trapped contract AND
        // that we have a trapped contract, we panic artificially trigger a bug for AFL
        if (cfg!(not(fuzzing)) || !catch_trapped_contract) && trapped.next().is_some() {
            panic!("Contract Trapped !");
        }
    }

    pub fn display_trap(&self, message: &OneInput, response: &FullContractResponse) {
        println!("\n🤯 A trapped contract got caught! Let's dive into it");
        println!("\n🐛 IMPORTANT STACKTRACE : {response}\n");
        println!("🎉 Find below the trace that caused that trapped contract");

        message.pretty_print(vec![response.clone()]);

        // Artificially trigger a bug for AFL
        panic!("\n🫡  Job is done! Please, don't mind the backtrace below/above.\n\n");
    }

    pub fn display_invariant(
        &self,
        responses: Vec<FullContractResponse>,
        decoded_msg: &OneInput,
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
                return Ok(*invariant);
            }
        }
        bail!("All invariants passed")
    }
}
