use color_eyre::eyre::{eyre, Result, WrapErr};
use serde::{Deserialize, Serialize};

pub const PROTO_VERSION: u8 = 0x01;
pub const MSG_TYPE_DATA: u8 = 0x01;
pub const MSG_TYPE_ACK: u8 = 0x02;

#[derive(Debug, Deserialize, Serialize)]
pub struct EchoProtocol {
    ver: u8,
    pub mtype: u8,
    pub msg: String,
}

impl EchoProtocol {
    /// Create a new EchoProtocol instance with the given type and message
    pub fn create(mtype: u8, msg: String) -> Self {
        EchoProtocol {
            ver: PROTO_VERSION,
            mtype,
            msg,
        }
    }

    /// Parse an EchoProtocol from a JSON string
    pub fn from_json(raw: &str) -> Result<Self> {
        let message = serde_json::from_str(raw)?;
        Ok(message)
    }

    /// Parse an EchoProtocol from a byte vector (expects UTF-8 JSON)
    pub fn from_bytes(raw: Vec<u8>) -> Result<Self> {
        let raw_json = String::from_utf8(raw)
            .wrap_err_with(|| eyre!("unable to parse bytes as UTF-8 string"))?;
        Ok(Self::from_json(&raw_json)?)
    }

    /// Convert this EchoProtocol into bytes (currently JSON only)
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        // TODO: implement binary serialization
        Ok(self.to_json()?.into_bytes())
    }

    /// Serialize this EchoProtocol into a pretty JSON string
    pub fn to_json(&self) -> Result<String> {
        let pretty_json = serde_json::to_string_pretty(self)
            .wrap_err_with(|| eyre!("problem serializing message to JSON"))?;
        Ok(pretty_json)
    }

    /// Print a debug message along with this protocol’s JSON representation
    pub fn display_debug_message(&self, tag: &str) {
        println!(
            "<============ {}\n {} \n==============",
            tag,
            self.to_json().unwrap()
        );
    }

    /// Return a debug-friendly string representation
    pub fn debug_string(&self) -> String {
        format!("{:#?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn constructor_sanity() {
        let message = EchoProtocol::create(1, "Hello, world!".to_string());
        assert_eq!(message.mtype, 1);
    }
}
