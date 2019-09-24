extern crate rand;
extern crate serde;
extern crate serde_json;

use serde_json::json;

use rand::Rng;
use std::{thread, time};

use crate::models::channel_message::ChannelMessage;
use crate::models::client_configuration::ClientConfiguration;
use crate::models::device::Device;

pub fn generate(config: ChannelMessage, sender: ws::Sender) -> () {
    generate_fake_data(sender, config.data);
}

fn generate_fake_data(sender: ws::Sender, client_config: std::string::String) -> () {
    let mut rng = rand::thread_rng();
    let config: ClientConfiguration = serde_json::from_str(&client_config).unwrap();

    loop {
        let mut devices: Vec<Device> = Vec::new();
        for index in 0..config.sample_size {
            let device = Device {
                id: index,
                value: rng.gen_range(config.value_range.min, config.value_range.max),
            };
            devices.push(device);
        }
        sender
            .send(
                json!({
                "action": "publish",
                "data": json!(devices),
                })
                .to_string(),
            )
            .unwrap();

        thread::sleep(time::Duration::from_millis(config.interval.into()));
    }
}
