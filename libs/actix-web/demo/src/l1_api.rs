//! Hello World simple API
//! Source: https://actix.rs/docs/getting-started

use crate::is_port_available;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/name")]
async fn name() -> impl Responder {
    HttpResponse::Ok().body("Abhijit")
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let port = 8080;

    if !is_port_available(port) {
        panic!("Port {} is already in use.", port);
    }

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(name)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
