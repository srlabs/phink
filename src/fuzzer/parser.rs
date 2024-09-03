use crate::{
    cli::config::{
        Configuration,
        OriginFuzzingOption,
    },
    contract::{
        remote::BalanceOf,
        runtime::Runtime,
    },
    fuzzer::fuzz::MAX_MESSAGES_PER_EXEC,
};
use contract_transcode::{
    ContractMessageTranscoder,
    Value,
};
use ink_metadata::{
    InkProject,
    Selector,
};
use std::sync::Mutex;
use OriginFuzzingOption::{
    DisableOriginFuzzing,
    EnableOriginFuzzing,
};

pub const DELIMITER: [u8; 8] = [42; 8]; // call delimiter for each message
pub const MIN_SEED_LEN: usize = 4;
/// 0..4 covers indices 0, 1, 2, and 3. (value to be transfered)
/// 4 covers index 4. (origin) (optionnal)
/// 5.. starts from index 5 and goes to the end of the array.
#[derive(Clone, Copy)]
pub struct Data<'a> {
    pub data: &'a [u8],
    pub pointer: usize,
    pub size: usize,
    pub max_messages_per_exec: usize,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub is_payable: bool,
    pub payload: Vec<u8>,
    pub value_token: BalanceOf<Runtime>,
    pub message_metadata: Value,
    pub origin: Origin,
}

#[derive(Debug, Clone)]
pub struct OneInput {
    pub messages: Vec<Message>,
    pub fuzz_option: OriginFuzzingOption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Origin(pub u8);
impl Default for Origin {
    fn default() -> Self {
        Origin(1)
    }
}
impl From<u8> for Origin {
    fn from(value: u8) -> Self {
        Origin(value)
    }
}
impl From<Origin> for u8 {
    fn from(origin: Origin) -> Self {
        origin.0
    }
}

impl<'a> Data<'a> {
    fn size_limit_reached(&self) -> bool {
        self.size >= self.max_messages_per_exec
    }
}

impl<'a> Iterator for Data<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.data.len() <= self.pointer || self.size_limit_reached() {
                return None;
            }
            let next_delimiter = self.data[self.pointer..]
                .windows(DELIMITER.len())
                .position(|window| window == DELIMITER);
            let next_pointer = match next_delimiter {
                Some(delimiter) => self.pointer + delimiter,
                None => self.data.len(),
            };
            let res = &self.data[self.pointer..next_pointer];
            self.pointer = next_pointer + DELIMITER.len();
            if res.len() >= MIN_SEED_LEN {
                self.size += 1;
                return Option::from(res);
            }
        }
    }
}
fn is_message_payable(selector: &Selector, metadata: &InkProject) -> bool {
    metadata
        .spec()
        .messages()
        .iter()
        .find(|msg| msg.selector().eq(selector))
        .map(|msg| msg.payable())
        .unwrap_or(false)
}

pub fn parse_input(
    data: &[u8],
    transcoder: &mut Mutex<ContractMessageTranscoder>,
    config: Configuration,
) -> OneInput {
    let max_messages_per_exec = config
        .max_messages_per_exec
        .unwrap_or(MAX_MESSAGES_PER_EXEC);

    let iterable = Data {
        data,
        pointer: 0,
        size: 0,
        max_messages_per_exec,
    };

    let mut input = OneInput {
        messages: vec![],
        fuzz_option: config.should_fuzz_origin(),
    };

    for decoded_payloads in iterable {
        let value_token: u32 = u32::from_ne_bytes(
            decoded_payloads[0..4]
                .try_into()
                .expect("missing transfer value bytes"),
        );

        let encoded_message: &[u8];
        let mut origin = Origin::default();
        match input.fuzz_option {
            EnableOriginFuzzing => {
                origin = Origin(decoded_payloads[4]);
                encoded_message = &decoded_payloads[5..];
            }
            DisableOriginFuzzing => encoded_message = &decoded_payloads[4..],
        }

        let binding = transcoder.get_mut().unwrap();
        let decoded_msg = binding.decode_contract_message(&mut &*encoded_message);

        match &decoded_msg {
            Ok(_) => {
                if iterable.max_messages_per_exec != 0
                    && input.messages.len() <= iterable.max_messages_per_exec
                {
                    let is_payable: bool = is_message_payable(
                        &Selector::from(
                            <&[u8] as TryInto<[u8; 4]>>::try_into(&encoded_message[0..4]).unwrap(),
                        ),
                        transcoder.get_mut().unwrap().metadata(),
                    );

                    input.messages.push(Message {
                        is_payable,
                        payload: encoded_message.into(),
                        value_token: value_token.into(),
                        message_metadata: decoded_msg.unwrap(),
                        origin,
                    });
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
    input
}
