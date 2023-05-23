//! Move the macro for the handler function - `name`
//! inside `route` method.

use crate::is_port_available;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

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

    HttpServer::new(|| App::new().route("/name", web::get().to(name)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
