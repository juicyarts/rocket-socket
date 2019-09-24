extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChannelMessage {
    pub action: std::string::String,
    pub data: std::string::String,
}
