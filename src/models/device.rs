extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: i16,
    pub data: i16,
}
