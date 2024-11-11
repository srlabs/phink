use anyhow::bail;
use std::{
    fmt::{
        Display,
        Formatter,
    },
    str::FromStr,
};

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Selector(pub [u8; 4]);

impl Display for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&hex::encode(self.0))
    }
}

impl FromStr for Selector {
    type Err = anyhow::Error;
    /// Decode a hexadecimal string selector into a byte
    /// array of length 4. Returns `None` if the decoding or conversion
    /// fails.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = hex::decode(s.trim_start_matches("0x"))?;
        if trimmed.len() != 4 {
            bail!("Decoded hex does not match the expected length of 4");
        }
        match Selector::try_from(trimmed.to_vec()) {
            Ok(sel) => Ok(sel),
            Err(e) => bail!(format!("Couldn't parse the selector {s} because {e}").to_string()),
        }
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
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim_start_matches("0x"); // Remove "0x" if present
        let bytes = hex::decode(value)?;
        Selector::try_from(bytes.as_slice())
    }
}

impl TryFrom<&[u8]> for Selector {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            return Err(anyhow::anyhow!("Invalid lenght for selector"));
        }
        let mut array = [0u8; 4];
        array.copy_from_slice(value);
        Ok(Selector(array))
    }
}

impl TryFrom<Vec<u8>> for Selector {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}
