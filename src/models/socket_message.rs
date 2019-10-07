extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueRange {
    pub min: i16,
    pub max: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SocketMessage {
    pub value_range: ValueRange,
    pub interval: u64,
    pub sample_size: i16,
}
