use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PDU {
    pub filename: String,
    pub checksum: String,
}

impl PDU {
    pub fn new(filename: String, checksum: String) -> Self {
        PDU { filename, checksum }
    }
}
