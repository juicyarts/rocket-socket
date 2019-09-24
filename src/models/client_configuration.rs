extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ValueRange {
    pub min: i16,
    pub max: i16,
}

#[derive(Serialize, Deserialize)]
pub struct ClientConfiguration {
    pub value_range: ValueRange,
    pub interval: u64,
    pub sample_size: i16,
}
