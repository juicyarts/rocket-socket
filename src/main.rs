#![feature(
    proc_macro_hygiene,
    decl_macro,
    custom_attribute,
    rustc_private,
    type_ascription
)]
#[macro_use]
extern crate rocket;
extern crate crossbeam_channel;
extern crate ws;

use crossbeam_channel::unbounded;
use std::thread;

mod generator;
mod models;
mod route;
mod socket;

use crate::route::static_files;
use crate::socket::ws_rs;


fn rocket() -> rocket::Rocket {
    let rocket_routes = routes![static_files::file,];
    rocket::ignite().mount("/", rocket_routes)
}

fn main() {
    thread::Builder::new()
        .name("Thread with Socket Connection".into())
        .spawn( move || {
            ws_rs::websocket();
        })
        .unwrap();

    // launch the rocket
    rocket().launch();
}
