use crate::{
    cli::config::OriginFuzzingOption,
    contract::{
        remote::{
            BalanceOf,
            ContractResponse,
            FullContractResponse,
        },
        runtime::Runtime,
        selectors::selector::Selector,
    },
    fuzzer::manager::CampaignManager,
};
use contract_transcode::{
    ContractMessageTranscoder,
    Value,
};
use prettytable::{
    Cell,
    Row,
    Table,
};
use serde_derive::Serialize;
use sp_core::crypto::AccountId32;
use std::fmt::{
    Display,
    Formatter,
};
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

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub is_payable: bool,
    pub payload: Vec<u8>,
    pub value_token: BalanceOf<Runtime>,
    pub message_metadata: Value,
    pub origin: Origin,
}

impl Message {
    pub fn display_with_reply(&self, reply: &ContractResponse) -> String {
        format!(
            "⛽️ Gas required: {}\n\
             🔥 Gas consumed: {}\n\
             🧑 Origin: {:?} ({})\n\
             💾 Storage deposit: {:?}{}",
            reply.gas_required,
            reply.gas_consumed,
            self.origin,
            AccountId32::new([self.origin.into(); 32]),
            reply.storage_deposit,
            if self.is_payable {
                format!(
                    "\n💸 Message was payable and {} units were transferred",
                    self.value_token
                )
            } else {
                String::new()
            }
        )
    }
    pub fn print(&self) -> String {
        format!(
            "Payload:\t0x{}\n\
             Origin:\t{:?} (identifier: {})\n\
             {}\
             Message:\t{}\n\n",
            hex::encode(&self.payload),
            AccountId32::new([self.origin.into(); 32]),
            self.origin.0,
            if self.is_payable {
                format!("Transfered: {}\n", self.value_token)
            } else {
                String::new()
            },
            self.message_metadata
        )
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message_metadata.to_string().as_str())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OneInput {
    pub messages: Vec<Message>,
    pub fuzz_option: OriginFuzzingOption,
    pub raw_binary: Vec<u8>,
}

impl OneInput {
    /// Pretty print the result of `OneInput`
    #[allow(dead_code)]
    pub fn pretty_print(&self, responses: Vec<FullContractResponse>) {
        println!("\n🌱 Executing new seed");
        let mut table = Table::new();
        table.add_row(Row::new(vec![Cell::new("Message"), Cell::new("Details")]));

        for (response, message) in responses.iter().zip(&self.messages) {
            let call_description = message.message_metadata.to_string();
            let debug = message.display_with_reply(response.get());

            table.add_row(Row::new(vec![
                Cell::new(&call_description),
                Cell::new(&debug),
            ]));
        }

        table.printstd();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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

impl Data<'_> {
    fn size_limit_reached(&self) -> bool {
        self.size >= self.max_messages_per_exec
    }
}

impl<'a> Iterator for Data<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.size_limit_reached() {
            return None;
        }
        // If `max_messages_per_exec` is 1, return the entire remaining data
        if self.max_messages_per_exec == 1 {
            let res = &self.data[self.pointer..];
            self.pointer = self.data.len();
            self.size += 1;
            return if res.len() >= MIN_SEED_LEN {
                Some(res)
            } else {
                None
            };
        }

        loop {
            if self.data.len() <= self.pointer {
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
                return Some(res);
            }
        }
    }
}

pub fn parse_input(bytes: &[u8], manager: CampaignManager) -> OneInput {
    let config = manager.clone().config();

    let max_msg = config.max_messages_per_exec.unwrap_or_default();
    let data = Data {
        data: bytes,
        pointer: 0,
        size: 0,
        max_messages_per_exec: max_msg,
    };

    let mut input = OneInput {
        messages: vec![],
        fuzz_option: config.should_fuzz_origin(),
        raw_binary: Vec::new(),
    };

    let arc = manager.transcoder();
    let guard = arc.try_lock().unwrap();

    for payload in data {
        let mut encoded_message = vec![0u8; payload.len() - 5];
        encoded_message.copy_from_slice(&payload[5..]);

        let selector: [u8; 4] = encoded_message[0..4].try_into().expect("[0..4] to u8 fail");
        let slctr = Selector::from(selector);
        let db = manager.database();

        // If we see a message being an invariant or our selector isn't a proper message we stop
        if db.contains_invariant(&slctr) || !db.contains_message(&slctr) {
            break;
        }

        let mut encoded_cloned = encoded_message.clone();

        match decode_contract_message(&guard, &mut encoded_cloned) {
            Ok(message_metadata) => {
                if data.max_messages_per_exec != 0
                    && input.messages.len() <= data.max_messages_per_exec
                {
                    let origin = match input.fuzz_option {
                        EnableOriginFuzzing => {
                            panic!("wtf");
                            Origin(payload[4])
                        }
                        DisableOriginFuzzing => Origin::default(),
                    };
                    let is_payable: bool = db.is_payable(&slctr);
                    let value_token: u128 = if is_payable {
                        panic!("wtf");
                        u32::from_ne_bytes(payload[0..4].try_into().unwrap()) as u128 // todo:16 not
                                                                                      // 4
                    } else {
                        0
                    };

                    input.raw_binary = Vec::from(bytes);

                    input.messages.push(Message {
                        is_payable,
                        payload: encoded_message.into(),
                        value_token,
                        message_metadata,
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

pub fn decode_contract_message(
    guard: &ContractMessageTranscoder,
    data: &mut Vec<u8>,
) -> anyhow::Result<Value> {
    use contract_transcode::Map;
    use std::io::Read;

    let mut data_as_slice = data.as_slice().clone();
    let mut msg_selector: [u8; 4] = [0u8; 4];
    data_as_slice.read_exact(&mut msg_selector)?;
    let msg_spec = guard
        .metadata()
        .spec()
        .messages()
        .iter()
        .find(|x| msg_selector == x.selector().to_bytes())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "XMessage with selector {} not found in contract metadata",
                hex::encode_upper(msg_selector)
            )
        })?;

    let mut args = Vec::new();
    for arg in msg_spec.args() {
        let name = arg.label().to_string();
        let value = guard.decode(arg.ty().ty().id, &mut data_as_slice)?;
        args.push((Value::String(name), value));
    }

    if !data_as_slice.is_empty() {
        return Err(anyhow::anyhow!(
            "input length was longer than expected by {} byte(s).\n `{}` bytes were left unread",
            data_as_slice.len(),
            hex::encode_upper(data)
        ));
    }
    let name = msg_spec.label().to_string();
    let map = Map::new(Some(&name), args.into_iter().collect());

    Ok(Value::Map(map))
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
