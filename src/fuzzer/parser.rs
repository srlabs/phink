use crate::{
    cli::config::OriginFuzzingOption,
    contract::{
        remote::BalanceOf,
        runtime::Runtime,
        selectors::selector::Selector,
    },
    fuzzer::manager::CampaignManager,
};
use contract_transcode::Value;
use OriginFuzzingOption::{
    DisableOriginFuzzing,
    EnableOriginFuzzing,
};

pub const DELIMITER: [u8; 8] = [42; 8]; // call delimiter for each message
pub const MIN_SEED_LEN: usize = 9;
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

pub fn parse_input(data: &[u8], manager: CampaignManager) -> OneInput {
    let config = manager.clone().config();

    let fuzzdata = Data {
        data,
        pointer: 0,
        size: 0,
        max_messages_per_exec: config.max_messages_per_exec.unwrap_or_default(),
    };

    let mut input = OneInput {
        messages: vec![],
        fuzz_option: config.should_fuzz_origin(),
    };

    for inkpayload in fuzzdata {
        let encoded_message: &[u8] = &inkpayload[5..];
        let selector: [u8; 4] = encoded_message[0..4]
            .try_into()
            .expect("Slice conversion failed");
        let sec = Selector::from(selector);

        if !manager.database().messages().unwrap().contains(&sec) {
            continue;
        }

        let value: u32 = u32::from_ne_bytes(inkpayload[0..4].try_into().unwrap()); // todo: it's actually 16 not 4
        let origin = match input.fuzz_option {
            EnableOriginFuzzing => Origin(inkpayload[4]),
            DisableOriginFuzzing => Origin::default(),
        };

        let decoded_msg = manager
            .transcoder()
            .lock()
            .unwrap()
            .decode_contract_message(&mut &*encoded_message);

        match &decoded_msg {
            Ok(metadata) => {
                if fuzzdata.max_messages_per_exec != 0
                    && input.messages.len() <= fuzzdata.max_messages_per_exec
                {
                    println!("{:?}", metadata);

                    input.messages.push(Message {
                        is_payable: manager.is_payable(&sec),
                        payload: encoded_message.into(),
                        value_token: value as u128, // todo: fix this
                        message_metadata: metadata.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_iterator() {
        let input = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 42, 42, 42, 42, 42, 42, 42, 42, 5, 6, 7, 23, 123, 1, 8,
            12, 13, 14,
        ];
        let data = Data {
            data: &input,
            pointer: 0,
            size: 0,
            max_messages_per_exec: 2,
        };

        let result: Vec<&[u8]> = data.collect();
        assert_eq!(
            result,
            vec![
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                &[5, 6, 7, 23, 123, 1, 8, 12, 13, 14]
            ]
        );
    }

    #[test]
    fn test_data_size_limit() {
        let input = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 42, 42, 42, 42, 42, 42, 42, 42, 5, 6, 7, 8, 8,
        ];
        let mut data = Data {
            data: &input,
            pointer: 0,
            size: 0,
            max_messages_per_exec: 1,
        };

        assert_eq!(data.next(), Some(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10][..]));
        assert_eq!(data.next(), None);
    }

    #[test]
    fn test_origin_default() {
        assert_eq!(Origin::default(), Origin(1));
    }

    #[test]
    fn test_origin_from_u8() {
        assert_eq!(Origin::from(5), Origin(5));
    }

    #[test]
    fn test_u8_from_origin() {
        assert_eq!(u8::from(Origin(3)), 3);
    }
}
