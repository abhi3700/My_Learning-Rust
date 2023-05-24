//! The App state has to be globally defined for each worker thread.
//!

use crate::is_port_available;
use actix_web::{get, web, App, HttpServer, Responder};

// Application state
#[derive(Clone)]
struct AppState {
    app_name: String,
}

// `web::Data<AppState>` is a type for passing application state
#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let port = 8080;

    if !is_port_available(port) {
        panic!("Port {} is already in use.", port);
    }

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // AppState defined outside for app instance.
    let app_state = web::Data::new(AppState {
        app_name: "Actix-web Application state defined outside".to_string(),
    });

    HttpServer::new(move || {
        App::new()
            // data is fed locally for this port.
            .app_data(app_state.clone())
            // service (GET, POST, DELETE) is registered to the app
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
