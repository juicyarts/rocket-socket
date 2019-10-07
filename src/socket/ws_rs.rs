extern crate rand;
extern crate serde;
extern crate serde_json;

use ws::{
    listen, CloseCode, Error, Handler, Handshake, Message, Request, Response, Result as WsResult,
};

use std::thread;

use crossbeam_channel::bounded;

use serde_json::json;

use crate::generator::generator;
use crate::models::channel_message::ChannelMessage;
use crate::models::server::Server;
use crate::models::socket_message::SocketMessage;

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> WsResult<(Response)> {
        match req.resource() {
            "/ws" => {
                println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
                println!("Client found is {:?}", req.client_addr().unwrap());

                let resp = Response::from_request(req);
                resp
            }
            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_open(&mut self, handshake: Handshake) -> WsResult<()> {
        let open_message = format!("{} entered the room", &handshake.peer_addr.unwrap(),);

        println!("{}", &open_message);

        Ok(())
    }

    fn on_message(&mut self, message: Message) -> WsResult<()> {
        let client_message = message.into_text().unwrap();
        let client_config: SocketMessage = serde_json::from_str(&client_message).unwrap();

        println!(
            "Interval set to: {},  Sample Size is: {}",
            client_config.interval, client_config.sample_size
        );

        self.out
            .send(
                json!({
                "topic": "status",
                "data": "ok",
                })
                .to_string(),
            )
            .unwrap();

        let sender: ws::Sender = self.out.clone();
        let receiver = self.queue_receiver.clone();

        if !self.child_thread_started {
            self.child_thread_started = true;
            thread::spawn(move || {
                generator::generate(
                    ChannelMessage {
                        action: String::from("configReceived"),
                        data: client_message,
                    },
                    sender,
                    receiver,
                )
            });
        } else {
            self.queue_sender
                .send(ChannelMessage {
                    action: String::from("configUpdated"),
                    data: client_message,
                })
                .unwrap();
        }

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => {
                println!("The client is done with the connection.");
                self.queue_sender
                    .send(ChannelMessage {
                        action: String::from("clientClosedConnection"),
                        data: String::from(""),
                    })
                    .unwrap();
            }
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.");
                self.queue_sender
                    .send(ChannelMessage {
                        action: String::from("clientClosedConnection"),
                        data: String::from(""),
                    })
                    .unwrap();
            }
            _ => {
                println!("The client encountered an error: {}", reason);
            }
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

pub fn websocket() -> () {
    println!("Web Socket Server is ready at ws://127.0.0.1:7777/ws");
    println!("Server is ready at http://127.0.0.1:7777/");

    listen("127.0.0.1:7777", |out| {
        let (s1, r1) = bounded::<ChannelMessage>(0);
        Server {
            out,
            child_thread_started: false,
            queue_sender: s1,
            queue_receiver: r1,
        }
    })
    .unwrap()
}
