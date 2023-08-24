//! Create an App with `index.html` page at a particular url with `scope` method & `route` method
//! https://actix.rs/docs/application

use crate::is_port_available;
use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

async fn get_task() -> impl Responder {
    "GET task"
}

async fn create_task() -> impl Responder {
    "POST task"
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
        App::new().route("/index", web::get().to(index)).service(
            // prefixes all service (resources and routes) attached to it...
            web::scope("/app").service(
                web::resource("/tasks")
                    // ...so this handles requests for `GET /app/tasks` & `POST /app/tasks`
                    .route(web::get().to(get_task))
                    .route(web::put().to(create_task)),
            ),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

// web::resource("/app/index.html")
//     // ...so this handles requests for `GET /app/index.html`
//     .route("/tasks", web::get().to(get_task))
//     .route("/tasks", web::put().to(create_task)),
