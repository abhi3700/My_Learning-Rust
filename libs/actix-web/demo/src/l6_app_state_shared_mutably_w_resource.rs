//! The App state has to be wrapped with mutex for
//! safe mutation across workers (threads).
//!
//! - State initialized inside the closure passed to HttpServer::new is local to the worker thread and may become de-synced if modified.
//! - To achieve globally shared state, it must be created outside of the closure passed to HttpServer::new and moved/cloned in.
//!
//! Source: https://actix.rs/docs/application#shared-mutable-state

use actix_web::{get, web, App, HttpServer, Responder};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::is_port_available;

pub struct Task {
    pub created_at: u32,
    pub title: String,
    pub assignee: Option<String>,
}

async fn home() -> &'static str {
    "Todo App!"
}

async fn get_tasks() -> &'static str {
    "Get tasks"
}

async fn create_task() -> &'static str {
    "Create a task"
}

async fn update_task() -> &'static str {
    "Modify a task"
}

async fn delete_task() -> &'static str {
    "remove a task"
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let task_map: web::Data<Mutex<HashMap<u64, Task>>> =
        web::Data::new(Mutex::new(HashMap::<u64, Task>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(task_map.clone())
            .route("/index", web::get().to(home))
            .service(
                web::resource("/tasks")
                    .route(web::get().to(get_tasks))
                    .route(web::post().to(create_task)),
            )
            .service(
                web::resource("/tasks/{id}")
                    .route(web::put().to(update_task))
                    .route(web::delete().to(delete_task)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
