use crate::contract::remote::{BalanceOf, Test};
use contract_transcode::{ContractMessageTranscoder, Value};
use ink_metadata::{InkProject, Selector};
use std::sync::Mutex;

pub const DELIMITER: [u8; 8] = [42; 8]; // call delimiter for each message
                                        // Minimum size for the seed
pub const MIN_SEED_LEN: usize = 5;
/// 4 covers index 4. (origin)
/// 0..4 covers indices 0, 1, 2, and 3. (value)
/// 5..starts from index 5 and goes to the end of the array.

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
    pub value_token: BalanceOf<Test>,
    pub message_metadata: Value,
    pub origin: u8,
}

#[derive(Debug, Clone)]
pub struct OneInput {
    pub messages: Vec<Message>,
    pub origin: u8,
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

pub fn parse_input(
    data: &[u8],
    transcoder: &mut Mutex<ContractMessageTranscoder>,
    max_messages_per_exec: usize,
) -> OneInput {
    let iterable = Data {
        data,
        pointer: 0,
        size: 0,
        max_messages_per_exec,
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

        input.origin = decoded_payloads[4];

        let encoded_message: &[u8] = &decoded_payloads[5..];
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
