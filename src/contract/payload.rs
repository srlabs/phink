use crate::contract::selectors::selector::Selector;
use anyhow::{
    bail,
    Context,
};
use serde::Deserialize;
use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    str::FromStr,
};

#[derive(Default, Clone)]
pub struct PayloadCrafter;

/// This prefix defines the way a property start with
///
/// # Example
// ```rust
// #[ink(message)]
// pub fn phink_assert_abc_dot_com_cant_be_registered(&self) -> bool {}
/// ```
pub const DEFAULT_PHINK_PREFIX: &str = "phink_";
#[derive(Deserialize, Debug, Clone)]
struct Spec {
    constructors: Vec<SelectorEntry>,
    messages: Vec<SelectorEntry>,
}

impl Spec {
    pub fn parse(&self) -> anyhow::Result<Vec<Selector>> {
        if self.constructors.is_empty() || self.messages.is_empty() {
            bail!("Empty constructor or messages vec")
        };

        self.constructors
            .iter()
            .chain(self.messages.iter())
            .map(|entry| Selector::try_from(entry.selector.as_str()))
            .collect::<Result<_, _>>()
            .map_err(|e| anyhow::anyhow!("Couldn't push the selector while parsing: {e}"))
    }
}

#[derive(Deserialize, Clone, Debug)]
struct SelectorEntry {
    selector: String,
}
impl PayloadCrafter {
    pub fn extract_all(contract_path: PathBuf) -> anyhow::Result<Vec<Selector>> {
        let mut all_selectors = Vec::new();

        let target_ink_path = contract_path.join("target/ink");
        let entries = fs::read_dir(&target_ink_path)
            .with_context(|| format!("Failed to read directory {:?}", target_ink_path))?;

        for entry in entries {
            let path = entry
                .with_context(|| "Failed to read directory entry")?
                .path();

            if path.extension().map_or(false, |ext| ext == "json")
                && !path.file_name().unwrap().to_str().unwrap().starts_with(".")
            {
                let contents = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read file {:?}", path))?;

                let v: Value = serde_json::from_str(&contents)
                    .with_context(|| format!("Failed to parse JSON from file {:?}", path))?;

                let spec: Spec = serde_json::from_value(v["spec"].clone())
                    .with_context(|| format!("Failed to deserialize spec from file {:?}", path))?;

                let selectors = spec.parse().context("Couldn't parse all the selectors")?;
                all_selectors.extend(selectors);
                break; // Since we only want to process the first JSON file found, break the loop
            }
        }

        Ok(all_selectors)
    }

    /// Extract every selector associated to the invariants defined in the ink!
    /// smart-contract See the documentation of `DEFAULT_PHINK_PREFIX` to know
    /// more about how to create a properties
    ///
    /// # Arguments
    /// * `json_data`: The JSON specs of the smart-contract
    pub fn extract_invariants(json_data: &str) -> Option<Vec<Selector>> {
        let data: Value = serde_json::from_str(json_data).expect("JSON was not well-formatted");

        Some(
            data["spec"]["messages"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|message| {
                    message["label"]
                        .as_str()
                        .filter(|label| label.starts_with(DEFAULT_PHINK_PREFIX))
                        .and_then(|_| message["selector"].as_str())
                        .map(|e| Selector::try_from(e).unwrap())
                })
                .collect(),
        )
    }

    /// Return the smart-contract constructor based on its spec. If there are
    /// multiple constructors, returns the one that preferably doesn't have
    /// args. If no suitable constructor is found or there is an error in
    /// processing, this function returns `Err`.
    pub fn get_constructor(json_data: &str) -> anyhow::Result<Selector> {
        let parsed_json: Value = serde_json::from_str(json_data)?;

        let constructors = parsed_json["spec"]["constructors"].as_array().unwrap();

        if constructors.len() == 1 {
            return Selector::from_str(constructors[0]["selector"].as_str().unwrap());
        }

        // Otherwise, look for a constructor without arguments.
        for constructor in constructors {
            if constructor["args"].as_array().map_or(false, Vec::is_empty) {
                return Selector::from_str(constructor["selector"].as_str().unwrap())
            }
        }
        bail!("No selector found")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        cli::{
            config::Configuration,
            ziggy::ZiggyConfig,
        },
        contract::{
            payload::PayloadCrafter,
            remote::BalanceOf,
            runtime::Runtime,
        },
        fuzzer::{
            fuzz::Fuzzer,
            parser::{
                parse_input,
                Origin,
            },
        },
        instrumenter::path::InstrumentedPath,
    };
    use contract_transcode::ContractMessageTranscoder;
    use sp_core::hexdisplay::AsBytesRef;
    use std::{
        fs,
        path::{
            Path,
            PathBuf,
        },
    };
    use tempfile::TempDir;

    #[test]
    fn test_parse_selectors() {
        let spec = Spec {
            constructors: vec![SelectorEntry {
                selector: "0x12345678".to_string(),
            }],
            messages: vec![
                SelectorEntry {
                    selector: "0xabcdef01".to_string(),
                },
                SelectorEntry {
                    selector: "0x23456789".to_string(),
                },
            ],
        };

        let selectors = spec.parse().unwrap();

        assert_eq!(selectors.len(), 3);
        assert_eq!(selectors[0], Selector::from([0x12, 0x34, 0x56, 0x78]));
        assert_eq!(selectors[1], Selector::from([0xab, 0xcd, 0xef, 0x01]));
        assert_eq!(selectors[2], Selector::from([0x23, 0x45, 0x67, 0x89]));
    }

    #[test]
    fn test_extract_invariants() {
        let json_data = r#"
        {
            "spec": {
                "messages": [
                    {
                        "label": "phink_test_invariant",
                        "selector": "0x12345678"
                    },
                    {
                        "label": "normal_function",
                        "selector": "0xabcdef01"
                    },
                    {
                        "label": "phink_another_invariant",
                        "selector": "0x23456789"
                    }
                ]
            }
        }
        "#;

        let invariants = PayloadCrafter::extract_invariants(json_data).unwrap();

        assert_eq!(invariants.len(), 2);
        assert_eq!(invariants[0], Selector::from([0x12, 0x34, 0x56, 0x78]));
        assert_eq!(invariants[1], Selector::from([0x23, 0x45, 0x67, 0x89]));
    }

    #[test]
    fn test_get_constructor() {
        let json_data = r#"
        {
            "spec": {
                "constructors": [
                    {
                        "label": "new",
                        "selector": "0x12345678",
                        "args": []
                    },
                    {
                        "label": "new_with_value",
                        "selector": "0xabcdef01",
                        "args": [
                            {
                                "label": "value",
                                "type": {
                                    "displayName": ["u128"],
                                    "type": 0
                                }
                            }
                        ]
                    }
                ]
            }
        }
        "#;

        let constructor = PayloadCrafter::get_constructor(json_data).unwrap();
        assert_eq!(constructor, [0x12, 0x34, 0x56, 0x78].into());
    }

    #[test]
    fn test_extract_all() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("target/ink/");
        fs::create_dir_all(&file_path).unwrap();
        let json_content = r#"
        {
            "spec": {
                "constructors": [
                    {
                        "selector": "0x12345678"
                    }
                ],
                "messages": [
                    {
                        "selector": "0xabcdef01"
                    }
                ]
            }
        }
        "#;
        fs::write(file_path.join("contract.json"), json_content).unwrap();

        let selectors = PayloadCrafter::extract_all(temp_dir.path().to_path_buf()).unwrap();

        assert_eq!(selectors.len(), 2);
        assert_eq!(selectors[0], Selector::from([0x12, 0x34, 0x56, 0x78]));
        assert_eq!(selectors[1], Selector::from([0xab, 0xcd, 0xef, 0x01]));
    }
    #[test]
    fn fetch_good_invariants() {
        let specs = fs::read_to_string("sample/dns/target/ink/dns.json").unwrap();
        let extracted: String = PayloadCrafter::extract_invariants(&specs)
            .unwrap()
            .iter()
            .map(|x| hex::encode(x) + " ")
            .collect();

        // DNS invariants
        assert_eq!(extracted, "2093daa4 ");
    }
    #[test]
    fn fetch_dummy_selectors() {
        let extracted: String = PayloadCrafter::extract_all(PathBuf::from("sample/dummy/"))
            .unwrap()
            .iter()
            .map(|x| x.to_string() + " ")
            .collect();

        // Dummy selectors
        assert_eq!(extracted, "9bae9d5e fa80c2f6 27d8f137 ");
    }
    #[test]
    fn fetch_correct_selectors() {
        let extracted: String = PayloadCrafter::extract_all(PathBuf::from("sample/dns/"))
            .unwrap()
            .iter()
            .map(|x| x.to_string() + " ")
            .collect();

        // DNS selectors
        assert_eq!(
            extracted,
            "9bae9d5e 229b553f b8a4d3d9 84a15da1 d259f7ba 07fcd0b1 2093daa4 "
        );
    }

    #[test]
    fn fetch_correct_dns_constructor() {
        let dns_spec = fs::read_to_string("sample/dns/target/ink/dns.json").unwrap();
        let ctor: Selector = PayloadCrafter::get_constructor(&dns_spec).unwrap();

        // DNS default selectors
        assert_eq!(hex::encode(ctor), "9bae9d5e");
    }

    #[test]
    fn encode_works_good() {
        let metadata_path = Path::new("sample/dns/target/ink/dns.json");
        let transcoder = ContractMessageTranscoder::load(metadata_path).unwrap();
        let constructor = "set_address";
        let args = [
            // name: Hash, new_address: AccountId
            "re",
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        ];
        let data = transcoder.encode(constructor, args).unwrap();
        let hex = hex::encode(data);
        assert_eq!(
            hex,
            "b8a4d3d9d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
        );
    }
    #[test]
    fn dummy_encode() {
        let metadata_path = Path::new("sample/dummy/target/ink/dummy.json");
        let transcoder = ContractMessageTranscoder::load(metadata_path).unwrap();
        let constructor = "crash_with_invariant";
        let data = transcoder.encode(constructor, ["\"\""]).unwrap();
        let hex = hex::encode(data);
        println!("{:?}", hex);
    }

    #[test]
    fn parse_one_message_dummy() -> anyhow::Result<()> {
        let encoded_bytes = hex::decode("0000000001fa80c2f600")?;

        let configuration = Configuration {
            max_messages_per_exec: Some(4), // because we have two messages below
            instrumented_contract_path: Some(InstrumentedPath::from("sample/dummy")),
            // below is a hack, `sample/dns` isn't the instrumented, but for the test we don't care
            ..Default::default()
        };

        let ziggy_config: ZiggyConfig =
            ZiggyConfig::new(configuration, PathBuf::from("sample/dummy"));

        let manager = Fuzzer::new(ziggy_config)?
            .init_fuzzer()
            .context("Couldn't grap the transcoder and the invariant manager")?;

        let input = parse_input(encoded_bytes.as_bytes_ref(), manager.to_owned());

        let msg = input.messages;
        println!("{:?}", msg);

        assert_eq!(msg.len(), 1, "No messages decoded");
        assert_eq!(
            msg.first().unwrap().origin,
            Origin::default(),
            "Origin is supposed to be the default one"
        );

        for i in 0..msg.len() {
            let hex = manager
                .transcoder()
                .lock()
                .unwrap()
                .decode_contract_message(&mut &*msg.get(i).unwrap().payload);
            assert!(hex.is_ok(), "Decoding wasn't Ok")
        }
        Ok(())
    }

    #[test]
    fn test_custom_origin() -> anyhow::Result<()> {
        let encoded_bytes = hex::decode("00000000fffa80c2f600")?;

        let configuration = Configuration {
            max_messages_per_exec: Some(4), // because we have two messages below
            fuzz_origin: true,
            instrumented_contract_path: Some(InstrumentedPath::from("sample/dummy")),
            // below is a hack, `sample/dns` isn't the instrumented, but for the test we don't care
            ..Default::default()
        };

        let ziggy_config: ZiggyConfig =
            ZiggyConfig::new(configuration, PathBuf::from("sample/dummy"));

        let manager = Fuzzer::new(ziggy_config)?
            .init_fuzzer()
            .context("Couldn't grap the transcoder and the invariant manager")?;

        let input = parse_input(encoded_bytes.as_bytes_ref(), manager.to_owned());

        let msg = input.messages;
        println!("{:?}", msg);

        assert_eq!(msg.len(), 1, "No messages decoded");
        assert_eq!(
            msg.first().unwrap().origin,
            Origin::from(0xff), // origin was ff
            "Origin is supposed to be the default one"
        );

        for i in 0..msg.len() {
            let hex = manager
                .transcoder()
                .lock()
                .unwrap()
                .decode_contract_message(&mut &*msg.get(i).unwrap().payload);
            assert!(hex.is_ok(), "Decoding wasn't Ok");

            println!("{:?}", hex.unwrap());
        }
        Ok(())
    }

    #[test]
    fn test_good_money_transfered() -> anyhow::Result<()> {
        let encoded_bytes = hex::decode("fffffffffffa80c2f600")?;

        let configuration = Configuration {
            max_messages_per_exec: Some(4), // because we have two messages below
            fuzz_origin: true,
            instrumented_contract_path: Some(InstrumentedPath::from("sample/dummy")),
            // below is a hack, `sample/dns` isn't the instrumented, but for the test we don't care
            ..Default::default()
        };

        let ziggy_config: ZiggyConfig =
            ZiggyConfig::new(configuration, PathBuf::from("sample/dummy"));

        let manager = Fuzzer::new(ziggy_config)?
            .init_fuzzer()
            .context("Couldn't grap the transcoder and the invariant manager")?;

        let input = parse_input(encoded_bytes.as_bytes_ref(), manager.to_owned());

        let msg = input.messages;
        println!("{:?}", msg);

        assert_eq!(msg.len(), 1, "No messages decoded");
        assert_eq!(
            msg.first().unwrap().origin,
            Origin::from(0xff), // origin was ff
            "Origin is supposed to be the default one"
        );

        assert_eq!(
            msg.first().unwrap().value_token,
            4294967295_u128, // origin was ff
            "Origin is supposed to be the default one"
        );

        for i in 0..msg.len() {
            let hex = manager
                .transcoder()
                .lock()
                .unwrap()
                .decode_contract_message(&mut &*msg.get(i).unwrap().payload);
            assert!(hex.is_ok(), "Decoding wasn't Ok")
        }
        Ok(())
    }
    #[test]
    fn parse_one_input_with_two_messages_dns() -> anyhow::Result<()> {
        let encoded_bytes = hex::decode(
            "0000000001229b553f9400000000000000000027272727272727272700002727272727272727272727\
            2a2a2a2a2a2a2a2a\
            0000000001229b553f9400000000000000000027272727272727272700002727272727272727272727",
        )?;

        let configuration = Configuration {
            max_messages_per_exec: Some(4), // because we have two messages below
            instrumented_contract_path: Some(InstrumentedPath::from("sample/dns")),
            // below is a hack, `sample/dns` isn't the instrumented, but for the test we don't care
            ..Default::default()
        };

        let ziggy_config: ZiggyConfig =
            ZiggyConfig::new(configuration, PathBuf::from("sample/dns"));

        let manager = Fuzzer::new(ziggy_config)?
            .init_fuzzer()
            .context("Couldn't grap the transcoder and the invariant manager")?;

        let input = parse_input(encoded_bytes.as_bytes_ref(), manager.to_owned());

        let msg = input.messages;
        println!("{:?}", msg);

        assert_eq!(msg.len(), 2, "No messages decoded");
        assert_eq!(
            msg.first().unwrap().origin,
            Origin::default(),
            "Origin is supposed to be the default one"
        );

        for i in 0..msg.len() {
            let hex = manager
                .transcoder()
                .lock()
                .unwrap()
                .decode_contract_message(&mut &*msg.get(i).unwrap().payload);
            assert!(hex.is_ok(), "Decoding wasn't Ok")
        }
        Ok(())
    }

    #[test]
    fn assert_reached_too_many_message() -> anyhow::Result<()> {
        let encoded_bytes = hex::decode(
            "0000000001229b553f9400000000000000000027272727272727272700002727272727272727272727\
            2a2a2a2a2a2a2a2a\
            0000000001229b553f9400000000000000000027272727272727272700002727272727272727272727\
            2a2a2a2a2a2a2a2a\
            0000000001229b553f9400000000000000000027272727272727272700002727272727272727272727\
            2a2a2a2a2a2a2a2a\
            0000000001229b553f9400000000000000000027272727272727272700002727272727272727272727",
        )?;

        let configuration = Configuration {
            max_messages_per_exec: Some(2), // two messages allow max
            instrumented_contract_path: Some(InstrumentedPath::from("sample/dns")),
            // below is a hack, `sample/dns` isn't the instrumented, but for the test we don't care
            ..Default::default()
        };

        let ziggy_config: ZiggyConfig =
            ZiggyConfig::new(configuration, PathBuf::from("sample/dns"));

        let manager = Fuzzer::new(ziggy_config)?
            .init_fuzzer()
            .context("Couldn't grap the transcoder and the invariant manager")?;

        let input = parse_input(encoded_bytes.as_bytes_ref(), manager.to_owned());

        let msg = input.messages;
        println!("{:?}", msg);

        assert_eq!(msg.len(), 2, "Tree parsed but  we put only two max");

        for i in 0..msg.len() {
            let hex = manager
                .transcoder()
                .lock()
                .unwrap()
                .decode_contract_message(&mut &*msg.get(i).unwrap().payload);
            assert!(hex.is_ok(), "Decoding wasn't Ok")
        }
        Ok(())
    }

    #[test]
    fn decode_works_good() {
        let metadata_path = Path::new("sample/dns/target/ink/dns.json");
        let transcoder = ContractMessageTranscoder::load(metadata_path).unwrap();

        let encoded_bytes =
            hex::decode("229b553f9400000000000000000027272727272727272700002727272727272727272727")
                .unwrap();
        let hex = transcoder.decode_contract_message(&mut &encoded_bytes[..]);
        assert_eq!(
            hex.unwrap().to_string(),
            "register { name: 0x9400000000000000000027272727272727272700002727272727272727272727 }"
        );
    }
}
