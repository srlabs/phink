use serde::Deserialize;
use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
};
use walkdir::WalkDir;

pub type Selector = [u8; 4];

#[derive(Default, Clone)]
pub struct PayloadCrafter {}

/// This prefix defines the way a property start with
/// # Example
/// ```
/// #[ink(message)]
///  pub fn phink_assert_abc_dot_com_cant_be_registered(&self) -> bool
/// ...
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
    pub fn extract_all(contract_path: PathBuf) -> Vec<Selector> {
        let mut all_selectors: Vec<Selector> = Vec::new();

        for entry in WalkDir::new(contract_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().extension().map_or(false, |ext| ext == "json") {
                if let Ok(contents) = fs::read_to_string(entry.path()) {
                    if let Ok(v) = serde_json::from_str::<Value>(&contents) {
                        if let Ok(spec) = serde_json::from_value::<Spec>(v["spec"].clone()) {
                            let selectors = Self::parse_selectors(&spec);
                            all_selectors.extend(selectors);
                        }
                    }
                }
            }
        }

        all_selectors
    }

    pub fn parse_selectors(spec: &Spec) -> Vec<Selector> {
        let mut selectors: Vec<Selector> = Vec::new();
        for entry in spec.constructors.iter().chain(spec.messages.iter()) {
            if let Ok(bytes) = hex::decode(entry.selector.trim_start_matches("0x")) {
                if let Ok(selector) = <[u8; 4]>::try_from(bytes.as_slice()) {
                    selectors.push(selector);
                }
            }
        }
        selectors
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
    /// processing, this function returns `None`.
    pub fn get_constructor(json_data: &str) -> Option<Selector> {
        // Parse the JSON data safely, return None if parsing fails.
        let parsed_json: Value = match serde_json::from_str(json_data) {
            Ok(data) => data,
            Err(_) => return None,
        };

        // Access the constructors array, return None if it's not found or not
        // an array.
        let constructors = parsed_json["spec"]["constructors"].as_array()?;

        // If there is exactly one constructor, return its selector if
        // available.
        if constructors.len() == 1 {
            return Self::get_selector_bytes(constructors[0]["selector"].as_str()?);
        }

        // Otherwise, look for a constructor without arguments.
        for constructor in constructors {
            if constructor["args"].as_array().map_or(false, Vec::is_empty) {
                return Self::get_selector_bytes(constructor["selector"].as_str()?);
            }
        }

        // Return None if no suitable constructor is found.
        None
    }

    /// Decode `encoded` to a proper `Selector`
    fn decode_selector(encoded: &str) -> Selector {
        let bytes: Vec<u8> = hex::decode(encoded.trim_start_matches("0x")).unwrap();
        <[u8; 4]>::try_from(bytes).expect("Selector is not a valid 4-byte array")
    }

    /// Helper function to decode a hexadecimal string selector into a byte
    /// array of length 4. Returns `None` if the decoding or conversion
    /// fails.
    fn get_selector_bytes(selector_str: &str) -> Option<Selector> {
        hex::decode(selector_str.trim_start_matches("0x"))
            .ok()?
            .try_into()
            .ok()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        cli::config::Configuration,
        contract::payload::{
            PayloadCrafter,
            Selector,
        },
        fuzzer::parser::parse_input,
    };
    use contract_transcode::ContractMessageTranscoder;
    use parity_scale_codec::Encode;
    use sp_core::hexdisplay::AsBytesRef;
    use std::{
        fs,
        path::Path,
    };

    #[test]
    fn fetch_good_invariants() {
        let specs = fs::read_to_string("sample/dns/target/ink/dns.json").unwrap();
        let extracted: String = PayloadCrafter::extract_invariants(&specs)
            .unwrap()
            .iter()
            .map(|x| hex::encode(x) + " ")
            .collect();

        // DNS invariants
        assert_eq!(extracted, "b587edaf 27d8f137 ");
    }

    #[test]
    fn fetch_correct_selectors() {
        let specs = fs::read_to_string("sample/dns/target/ink/dns.json").unwrap();
        let extracted: String = PayloadCrafter::extract_all(&specs)
            .iter()
            .map(|x| hex::encode(x) + " ")
            .collect();

        // DNS selectors
        assert_eq!(
            extracted,
            "9bae9d5e 229b553f b8a4d3d9 84a15da1 d259f7ba 07fcd0b1 2e15cab0 5d17ca7f "
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
        let data = transcoder.encode(&constructor, args).unwrap();
        let hex = hex::encode(data);
        println!("Encoded constructor data {}", hex);
        assert!(!hex.is_empty())
    }

    #[test]
    fn parse_one_input_with_two_messages() {
        let metadata_path = Path::new("sample/dns/target/ink/dns.json");

        let encoded_bytes = hex::decode(
            "\
            3007fcd09e3707fcd0b13038ff7f00304d302f3030d259f7ba303000042438ff7fe4fcd09e3763000000\
            2a2a2a2a2a2a2a2a\
            3007fcd09e3707fcd0b13038ff7f00304d302f3030d259f7ba303000042438ff7fe4fcd09e3763000000",
        )
        .unwrap();

        let mut transcoder_loader = std::sync::Mutex::new(
            ContractMessageTranscoder::load(Path::new(metadata_path)).unwrap(),
        );

        let msg = parse_input(
            encoded_bytes.as_bytes_ref(),
            &mut transcoder_loader,
            Configuration::default(),
        )
        .messages;
        println!("{:?}", msg);

        for i in 0..msg.len() {
            let hex = transcoder_loader
                .lock()
                .unwrap()
                .decode_contract_message(&mut &*msg.get(i).unwrap().payload);
            println!("{:?}", hex);
        }

        let hash_two: [u8; 32] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 2,
        ];

        println!("{:?}", hex::encode(hash_two.encode()));
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
