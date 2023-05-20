//! The App state has to be wrapped with mutex for
//! safe mutation across workers (threads).
//!
//! - State initialized inside the closure passed to HttpServer::new is local to the worker thread and may become de-synced if modified.
//! - To achieve globally shared state, it must be created outside of the closure passed to HttpServer::new and moved/cloned in.

use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

use crate::is_port_available;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/")]
async fn index(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let port = 8080;

    if !is_port_available(port) {
        panic!("Port {} is already in use.", port);
    }

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        // NOTE: Here, the counter is initialized _outside_ HttpServer::new closure.
        // Hence, it is globally shared across all workers.
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
