use crate::contract::remote::{BalanceOf, Test};
use contract_transcode::{ContractMessageTranscoder, Value};
use ink_metadata::{InkProject, Selector};
use std::sync::Mutex;

pub const DELIMITER: [u8; 8] = [42; 8]; // call delimiter for each message
// Minimum size for the seed
pub const MIN_SEED_LEN: usize = 0 + 4 + 2 + 4;
pub const MAX_MESSAGES_PER_EXEC: usize = 4; // One execution contains maximum 4 messages. Todo: make it as a User parameter

pub struct Data<'a> {
    pub data: &'a [u8],
    pub pointer: usize,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub is_payable: bool,
    pub payload: Vec<u8>,
    pub value_token: BalanceOf<Test>,
    pub message_metadata: Value,
    pub origin: usize,
}

#[derive(Debug, Clone)]
pub struct OneInput {
    pub messages: Vec<Message>,
    pub origin: usize,
}

impl<'a> Data<'a> {
    fn size_limit_reached(&self) -> bool {
        self.size >= MAX_MESSAGES_PER_EXEC
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
            let res = Some(&self.data[self.pointer..next_pointer]);
            self.pointer = next_pointer + DELIMITER.len();
            if res.unwrap().len() >= MIN_SEED_LEN {
                self.size += 1;
                return res;
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

pub fn parse_input(data: &[u8], transcoder: &mut Mutex<ContractMessageTranscoder>) -> OneInput {
    let iterable = Data {
        data,
        pointer: 0,
        size: 0,
    };
    let mut input = OneInput {
        messages: vec![],
        origin: 1,
    };
    for decoded_payloads in iterable {
        let value_token: u32 = u32::from_ne_bytes(
            decoded_payloads[0..4]
                .try_into()
                .expect("missing transfer value bytes"),
        );

        input.origin = u16::from_ne_bytes(
            decoded_payloads[4..6]
                .try_into()
                .expect("missing origin bytes"),
        ) as usize;

        let encoded_message: &[u8] = &decoded_payloads[6..];
        let binding = transcoder.get_mut().unwrap();
        let decoded_msg = binding.decode_contract_message(&mut &*encoded_message);

        match &decoded_msg {
            Ok(_) => {
                if MAX_MESSAGES_PER_EXEC != 0 && input.messages.len() <= MAX_MESSAGES_PER_EXEC {
                    let is_payable: bool = is_message_payable(
                        &Selector::from(
                            <&[u8] as TryInto<[u8; 4]>>::try_into(&encoded_message[0..4]).unwrap(),
                        ),
                        transcoder.lock().unwrap().metadata(),
                    );

                    input.messages.push(Message {
                        is_payable,
                        payload: encoded_message.into(),
                        value_token: value_token.into(),
                        message_metadata: decoded_msg.unwrap(),
                        origin: input.origin,
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
