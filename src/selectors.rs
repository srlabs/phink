use hex::FromHex;
use serde::ser::Error;
use serde::Deserialize;
use serde_json::Value;
use std::fs;

#[macro_export]
macro_rules! message_to_bytes {
    ($s:expr) => {{
        let hash = blake2_256($s.as_bytes());
        [hash[0], hash[1], hash[2], hash[3]]
    }};
}
// Parses a JSON and returns a list of all possibles messages
pub fn extract_all(json_data: String) -> Vec<[u8; 4]> {
    #[derive(Deserialize)]
    struct Spec {
        constructors: Vec<SelectorEntry>,
        messages: Vec<SelectorEntry>,
    }

    #[derive(Deserialize)]
    struct SelectorEntry {
        selector: String,
    }

    let v: Value = serde_json::from_str(json_data.as_str()).unwrap();

    let spec: Spec = serde_json::from_value(v["spec"].clone()).unwrap();

    let mut selectors: Vec<[u8; 4]> = Vec::new();
    for entry in spec.constructors.iter().chain(spec.messages.iter()) {
        let bytes: Vec<u8> = hex::decode(&entry.selector.trim_start_matches("0x"))
            .unwrap()
            .try_into()
            .map_err(|_| serde_json::Error::custom("Selector is not a valid 4-byte array"))
            .unwrap();
        selectors.push(<[u8; 4]>::try_from(bytes).unwrap());
    }
    selectors
}

// Return the smart-contract constructor based on its spec. If there's multiple constructors,
// returns the one preferably that doesn't have args
pub fn constructor(json_data: String) -> Vec<[u8; 4]> {
    let parsed_json: Value = serde_json::from_str(json_data.as_str()).unwrap();

    if let Some(constructors) = parsed_json["spec"]["constructors"].as_array() {
        // If there is only one constructor, add its selector
        if constructors.len() == 1 {
            if let Some(selector_str) = constructors[0]["selector"].as_str() {
                let bytes = hex::decode(selector_str.trim_start_matches("0x")).unwrap();

                let chunks: Vec<[u8; 4]> = bytes
                    .chunks_exact(4)
                    .map(|chunk| chunk.try_into().unwrap())
                    .collect();

                return chunks;
            }
        }

        // Find the constructor with no arguments
        if let Some(constructor) = constructors
            .iter()
            .find(|c| c["args"].as_array().map_or(false, |args| args.is_empty()))
        {
            if let Some(selector_str) = constructor["selector"].as_str() {
                let bytes = hex::decode(selector_str.trim_start_matches("0x")).unwrap();

                let chunks: Vec<[u8; 4]> = bytes
                    .chunks_exact(4)
                    .map(|chunk| chunk.try_into().unwrap())
                    .collect();

                return chunks;
            }
        }
    }
    panic!("No constructor found, or there's multiple constructor but no one without arguments");
}

#[test]
fn fetch_correct_flipper_selectors() {
    let flipper_specs = fs::read_to_string("sample/flipper/target/ink/flipper.json").unwrap();
    let extracted: String = extract_all(flipper_specs)
        .iter()
        .map(|x| hex::encode(x) + " ")
        .collect();

    // Flipper default selectors
    assert_eq!(extracted, "9bae9d5e ed4b9d1b 633aa551 2f865bd9 ");
}

#[test]
fn fetch_correct_dns_constructor() {
    let dns_spec = fs::read_to_string("sample/dns/target/ink/dns.json").unwrap();
    let ctor: String = constructor(dns_spec)
        .iter()
        .map(|x| hex::encode(x))
        .collect();

    // DNS default selectors
    assert_eq!(ctor, "9bae9d5e");
}
