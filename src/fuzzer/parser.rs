use crate::contract::remote::{BalanceOf, Test};
use crate::contract::runtime::RuntimeCall;
use contract_transcode::{ContractMessageTranscoder, Transcoder};
use parity_scale_codec::DecodeLimit;
use std::sync::Mutex;

// Call delimiter: `********`
pub const DELIMITER: [u8; 8] = [42; 8];
pub const MIN_SEED_LEN: usize = 12; //Origin + Value + Selector
pub const MAX_MESSAGES_PER_EXEC: usize = 4;

pub struct Data<'a> {
    pub data: &'a [u8],
    pub pointer: usize,
    pub size: usize,
}

#[derive(Debug)]
pub struct Message {
    pub origin: usize,
    pub call: Vec<u8>,
    pub value_token: BalanceOf<Test>,
    pub description: String,
}

#[derive(Debug)]
pub struct OneInput {
    pub messages: Vec<Message>,
}

impl<'a> Data<'a> {
    fn size_limit_reached(&self) -> bool {
        !(MAX_MESSAGES_PER_EXEC == 0) && self.size >= MAX_MESSAGES_PER_EXEC
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

pub fn parse_input(data: &[u8], transcoder: &mut Mutex<ContractMessageTranscoder>) -> OneInput {
    let iterable = Data {
        data,
        pointer: 0,
        size: 0,
    };
    let mut input = OneInput { messages: vec![] };
    for extrinsic in iterable {
        let value_token: u32 =
            u32::from_ne_bytes(extrinsic[0..4].try_into().expect("missing lapse bytes"));
        let origin: usize =
            u16::from_ne_bytes(extrinsic[4..6].try_into().expect("missing origin bytes")) as usize;
        let mut encoded_extrinsic: &[u8] = &extrinsic[6..];

        let decoded_msg = transcoder
            .lock()
            .unwrap()
            .decode_contract_message(&mut &*encoded_extrinsic);

        // println!("{:?}", decoded_msg.clone().unwrap().to_string());

        match &decoded_msg {
            Ok(_) => {
                if MAX_MESSAGES_PER_EXEC != 0 && input.messages.len() <= MAX_MESSAGES_PER_EXEC {
                    input.messages.push(Message {
                        origin,
                        call: encoded_extrinsic.into(),
                        value_token: value_token.into(),
                        description: decoded_msg.unwrap().to_string(),
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
