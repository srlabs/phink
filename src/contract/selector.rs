use std::fmt::{
    Display,
    Formatter,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SelectorError {
    #[error("Invalid hex string")]
    InvalidHex(#[from] hex::FromHexError),

    #[error("Invalid length for a 4-byte array")]
    InvalidLength,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Selector(pub [u8; 4]);

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
