use std::error::Error;

use actix_web::{get, web, App, HttpServer, Responder};
use crate::simple_window;

#[get("/{id}/{name}/index.html")]
async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    return format!("Hello {}! id:{}", name, id);
}

pub fn start(){
    // simple_window::left_bar_window::left_bar_window_start();
    // async move {
    // actix_web::rt::System::with_tokio_rt(|| {
    //     tokio::runtime::Builder::new_multi_thread()
    //         .enable_all()
    //         .worker_threads(8)
    //         .thread_name("main-tokio")
    //         .build()
    //         .unwrap()
    // })
    // .block_on(async_main());
    // };
    simple_window::left_bar::start();


}

async fn async_main() {
    tokio::spawn(async move {
        println!("From main tokio thread");
        // Would panic if uncommented showing no system here
        // println!("{:?}",actix_web::rt::System::current());
    });

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(index)
    })
    .workers(8)
    .bind("0.0.0.0:8088")
    .expect("Couldn't bind to port 8088")
    .run()
    .await
    .unwrap()
}

fn start_interface() {
    println!("From app thread");
    simple_window::window1::window1_start();
    

}