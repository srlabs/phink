use anyhow::{
    bail,
    Context,
};
use serde::Deserialize;
use serde_json::Value;
use std::{
    fmt::{
        Display,
        Formatter,
    },
    fs,
    path::PathBuf,
};
use thiserror::Error;
use walkdir::WalkDir;
#[derive(Error, Debug)]
pub enum SelectorError {
    #[error("Invalid hex string")]
    InvalidHex(#[from] hex::FromHexError),

    #[error("Invalid length for a 4-byte array")]
    InvalidLength,
}
#[derive(PartialEq, Clone, Debug)]
pub struct Selector([u8; 4]);

impl Display for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&hex::encode(self.0))
    }
}

impl AsMut<[u8]> for Selector {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl AsRef<[u8]> for Selector {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 4]> for Selector {
    fn from(value: [u8; 4]) -> Self {
        Self(value)
    }
}

impl From<Selector> for Vec<u8> {
    fn from(selector: Selector) -> Self {
        selector.0.to_vec()
    }
}

impl TryFrom<&str> for Selector {
    type Error = SelectorError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim_start_matches("0x"); // Remove "0x" if present
        let bytes = hex::decode(value)?;
        Selector::try_from(bytes.as_slice())
    }
}

impl TryFrom<&[u8]> for Selector {
    type Error = SelectorError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            return Err(SelectorError::InvalidLength);
        }
        let mut array = [0u8; 4];
        array.copy_from_slice(value);
        Ok(Selector(array))
    }
}

impl TryFrom<Vec<u8>> for Selector {
    type Error = SelectorError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

#[derive(Default, Clone)]
pub struct PayloadCrafter {}

/// This prefix defines the way a property start with
///
/// # Example
// ```rust
// #[ink(message)]
// pub fn phink_assert_abc_dot_com_cant_be_registered(&self) -> bool {}
/// ```
pub const DEFAULT_PHINK_PREFIX: &str = "phink_";
#[derive(Deserialize)]
pub struct Spec {
    constructors: Vec<SelectorEntry>,
    messages: Vec<SelectorEntry>,
}

#[derive(Deserialize)]
pub struct SelectorEntry {
    selector: String,
}
impl PayloadCrafter {
    pub fn extract_all(contract_path: PathBuf) -> anyhow::Result<Vec<Selector>> {
        let mut all_selectors: Vec<Selector> = Vec::new();

        for entry in WalkDir::new(contract_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().extension().map_or(false, |ext| ext == "json") {
                if let Ok(contents) = fs::read_to_string(entry.path()) {
                    if let Ok(v) = serde_json::from_str::<Value>(&contents) {
                        if let Ok(spec) = serde_json::from_value::<Spec>(v["spec"].clone()) {
                            let selectors = Self::parse_selectors(&spec)
                                .context("Couldn't parse all the selectors")?;
                            all_selectors.extend(selectors);
                            return Ok(all_selectors);
                        }
                    }
                }
            }
        }

        Ok(all_selectors)
    }

    pub fn parse_selectors(spec: &Spec) -> anyhow::Result<Vec<Selector>> {
        let mut selectors: Vec<Selector> = Vec::new();
        for entry in spec.constructors.iter().chain(spec.messages.iter()) {
            match Selector::try_from(entry.selector.as_str()) {
                Ok(sel) => selectors.push(sel),
                Err(e) => {
                    bail!("Couldn't push the selector while parsing: {e}")
                }
            }
        }
        Ok(selectors)
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
                        .map(Self::decode_selector)
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
            return Self::get_selector_bytes(constructors[0]["selector"].as_str().unwrap());
        }

        // Otherwise, look for a constructor without arguments.
        for constructor in constructors {
            if constructor["args"].as_array().map_or(false, Vec::is_empty) {
                return Self::get_selector_bytes(constructor["selector"].as_str().unwrap());
            }
        }
        bail!("No selector found")
    }

    /// Decode `encoded` to a proper `Selector`
    fn decode_selector(encoded: &str) -> Selector {
        // todo: refeactor
        let bytes: Vec<u8> = hex::decode(encoded.trim_start_matches("0x")).unwrap();
        Selector(<[u8; 4]>::try_from(bytes).expect("Selector is not a valid 4-byte array"))
    }

    /// Helper function to decode a hexadecimal string selector into a byte
    /// array of length 4. Returns `None` if the decoding or conversion
    /// fails.
    fn get_selector_bytes(selector: &str) -> anyhow::Result<Selector> {
        let trimmed = hex::decode(selector.trim_start_matches("0x"))?.to_vec();
        match Selector::try_from(trimmed) {
            Ok(sel) => Ok(sel),
            Err(e) => {
                bail!(format!("Couldn't parse the selector - {e}"))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        cli::config::Configuration,
        contract::payload::{
            PayloadCrafter,
            Selector,
        },
        fuzzer::parser::{
            parse_input,
            Origin,
        },
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

        let selectors = PayloadCrafter::parse_selectors(&spec).unwrap();

        assert_eq!(selectors.len(), 3);
        assert_eq!(
            selectors[0],
            Selector::try_from([0x12, 0x34, 0x56, 0x78]).unwrap()
        );
        assert_eq!(
            selectors[1],
            Selector::try_from([0xab, 0xcd, 0xef, 0x01]).unwrap()
        );
        assert_eq!(
            selectors[2],
            Selector::try_from([0x23, 0x45, 0x67, 0x89]).unwrap()
        );
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
        assert_eq!(
            invariants[0],
            Selector::try_from([0x12, 0x34, 0x56, 0x78]).unwrap()
        );
        assert_eq!(
            invariants[1],
            Selector::try_from([0x23, 0x45, 0x67, 0x89]).unwrap()
        );
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
        let file_path = temp_dir.path().join("contract.json");
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
        fs::write(&file_path, json_content).unwrap();

        let selectors = PayloadCrafter::extract_all(temp_dir.path().to_path_buf()).unwrap();

        assert_eq!(selectors.len(), 2);
        assert_eq!(
            selectors[0],
            Selector::try_from([0x12, 0x34, 0x56, 0x78]).unwrap()
        );
        assert_eq!(
            selectors[1],
            Selector::try_from([0xab, 0xcd, 0xef, 0x01]).unwrap()
        );
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
    fn fetch_correct_selectors() {
        let extracted: String = PayloadCrafter::extract_all(PathBuf::from("sample/dns"))
            .unwrap()
            .iter()
            .map(|x| hex::encode(x) + " ")
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
    fn parse_one_input_with_two_messages() {
        let metadata_path = Path::new("sample/dns/target/ink/dns.json");

        let encoded_bytes = hex::decode(
            "00000000229b553f9400000000000000000027272727272727272700002727272727272727272727\
            2a2a2a2a2a2a2a2a\
            00000000229b553f9400000000000000000027272727272727272700002727272727272727272727",
        )
        .unwrap();

        let mut transcoder_loader = std::sync::Mutex::new(
            ContractMessageTranscoder::load(Path::new(metadata_path)).unwrap(),
        );

        let input = parse_input(
            encoded_bytes.as_bytes_ref(),
            &mut transcoder_loader,
            Configuration::default(),
        );
        let msg = input.messages;
        assert_eq!(msg.len(), 2, "No messages decoded");
        assert_eq!(
            msg.first().unwrap().origin,
            Origin::default(),
            "Origin is supposed to be the default one"
        );

        for i in 0..msg.len() {
            let hex = transcoder_loader
                .lock()
                .unwrap()
                .decode_contract_message(&mut &*msg.get(i).unwrap().payload);
            assert!(hex.is_ok(), "Decoding wasn't Ok")
        }
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
