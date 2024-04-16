use hex::FromHex;
use serde::ser::Error;
use serde::Deserialize;
use serde_json::Value;
use std::fs;

#[derive(Deserialize)]
struct Spec {
    constructors: Vec<SelectorEntry>,
    messages: Vec<SelectorEntry>,
}

#[derive(Deserialize)]
struct SelectorEntry {
    selector: String,
}
#[macro_export]
macro_rules! message_to_bytes {
    ($s:expr) => {{
        let hash = blake2_256($s.as_bytes());
        [hash[0], hash[1], hash[2], hash[3]]
    }};
}
// Parses a JSON and returns a list of all possibles messages
pub fn extract_selectors(json_data: String) -> Vec<[u8; 4]> {
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

#[test]
fn fetches_selector() {
    let flipper_specs = fs::read_to_string(
        "/Users/kevinvalerio/Desktop/phink/sample/flipper/target/ink/flipper.json",
    )
    .unwrap();
    let extracted: Vec<_> = extract_selectors(flipper_specs)
        .iter()
        .map(|x| hex::encode(x))
        .collect();
    println!("{:?}", extracted); //["9bae9d5e", "ed4b9d1b", "633aa551", "2f865bd9"]
}
