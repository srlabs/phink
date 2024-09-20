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
    fuzzer::{
        engine::pretty_print,
        parser::{
            Message,
            OneInput,
            Origin,
        },
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
    contract_bridge: ContractSetup,
    database: SelectorDatabase,
    configuration: Configuration,
    transcoder: Arc<Mutex<ContractMessageTranscoder>>,
}

impl CampaignManager {
    pub fn new(
        database: SelectorDatabase,
        contract_bridge: ContractSetup,
        configuration: Configuration,
    ) -> anyhow::Result<Self> {
        let transcoder = Arc::new(Mutex::new(
            ContractMessageTranscoder::load(Path::new(&contract_bridge.path_to_specs))
                .context("Cannot instantiante the `ContractMessageTranscoder`")?,
        ));

        Ok(Self {
            contract_bridge,
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

    pub fn is_payable(&self, selector: &Selector) -> bool {
        self.transcoder
            .lock()
            .unwrap()
            .metadata()
            .spec()
            .messages()
            .iter()
            .find(|msg| msg.selector().to_bytes().eq(selector.as_ref()))
            .map(|msg| msg.payable())
            .unwrap_or(false)
    }

    pub(crate) fn check_invariants(
        &self,
        all_msg_responses: &[FullContractResponse],
        decoded_msgs: &OneInput,
    ) {
        let first = decoded_msgs.messages[0].to_owned();
        all_msg_responses
            .iter()
            .filter(|response| CampaignManager::is_trapped(response))
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
        // We print the details only when we don't fuzz, so when we run a seed
        // for instance, otherwise this will pollute the AFL logs
        #[cfg(not(fuzzing))]
        {
            println!("\n🤯 A trapped contract got caught! Let's dive into it");

            println!(
                "\n🐛 IMPORTANT STACKTRACE : {}\n",
                String::from_utf8_lossy(&InputCoverage::remove_cov_from_trace(
                    response.clone().debug_message
                ))
                .replace("\n", " ")
            );

            println!("🎉 Find below the trace that caused that trapped contract");

            pretty_print(
                vec![response],
                OneInput {
                    messages: vec![message.clone()],
                    fuzz_option: self.configuration.should_fuzz_origin(),
                },
            );
        }

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
            pretty_print(responses, decoded_msg);
        }
        // Artificially trigger a bug for AFL
        panic!("\n🫡   Job is done! Please, don't mind the backtrace below/above.\n\n");
    }

    /// This function aims to call every invariants via `invariant_selectors`.
    pub fn are_invariants_failing(&self, origin: Origin) -> anyhow::Result<Selector> {
        for invariant in &self.database.to_owned().invariants()? {
            let invariant_call: FullContractResponse = self.contract_bridge.clone().call(
                invariant.as_ref(),
                origin.into(),
                0,
                self.configuration.clone(),
            );
            if invariant_call.result.is_err() {
                return Ok(invariant.clone());
            }
        }
        bail!("All invariants passed")
    }

    pub fn is_trapped(contract_response: &FullContractResponse) -> bool {
        if let Err(DispatchError::Module(ModuleError { message, .. })) = contract_response.result {
            if message == Some("ContractTrapped") {
                return true;
            }
        }
        false
    }
}
