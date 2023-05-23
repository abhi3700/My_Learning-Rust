//! Create an App with `index.html` page at a particular url with `scope` method & `route` method
//! https://actix.rs/docs/application

use crate::is_port_available;
use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello world!"
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
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ...so this handles requests for `GET /app/index.html`
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
