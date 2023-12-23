mod web_server;
use std::thread;
use actix_web::{HttpServer, App, web, Responder};
use ansync_main::async_main;
use clock::clock::clock;
use counter::counter::counter;
use iced::Settings;
use web_server::actix_main::create_new_server;
mod counter;
mod clock;
mod ansync_main;
mod simple_window;


// #[tokio::main]
// async fn start_web() {
//     let _create_new_server = web_server::actix_main::create_new_server().expect("cannot create server");
// }


fn main() -> std::io::Result<()> {
    async_main::start();
    Ok(())
}

async fn hello_world() -> impl Responder {
    "Hello World!"
}
