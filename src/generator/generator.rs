extern crate rand;
extern crate serde;
extern crate serde_json;

use crossbeam_channel::Receiver;
use serde_json::json;

use rand::Rng;
use std::{thread, time};

use crate::models::channel_message::ChannelMessage;
use crate::models::client_configuration::ClientConfiguration;
use crate::models::device::Device;

pub fn generate(
    config: ChannelMessage,
    sender: ws::Sender,
    receiver: Receiver<ChannelMessage>,
) -> () {
    generate_fake_data(sender, config.data, receiver);
}

fn generate_fake_data(
    sender: ws::Sender,
    client_config: std::string::String,
    receiver: Receiver<ChannelMessage>,
) -> () {
    let mut rng = rand::thread_rng();
    println!("set config");
    let mut config: ClientConfiguration = serde_json::from_str(&client_config).unwrap();

    loop {
        let mut devices: Vec<Device> = Vec::new();
        for index in 0..config.sample_size {
            let device = Device {
                id: index,
                data: rng.gen_range(config.value_range.min, config.value_range.max),
            };
            devices.push(device);
        }
        sender
            .send(
                json!({
                "topic": "update",
                "data": json!(devices),
                })
                .to_string(),
            )
            .unwrap();

        let messages: Vec<ChannelMessage> = receiver.try_iter().collect();

        thread::sleep(time::Duration::from_millis(config.interval.into()));

        for message in messages {
            if message.action == String::from("configUpdated") {
                println!("updated config");
                config = serde_json::from_str(&message.data).unwrap();
            } else if message.action == String::from("clientClosedConnection") {
                println!("terminated generator");
                break;
            }
        }
    }
}
