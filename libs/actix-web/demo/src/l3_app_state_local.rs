use crate::is_port_available;
use actix_web::{get, web, App, HttpServer, Responder};

// Application state
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

    // AppState locally defined for 1 app instance.
    HttpServer::new(|| {
        App::new()
            // data is fed locally for this port.
            .data(AppState {
                app_name: "Actix-web Application state locally defined".to_string(),
            })
            // service (GET, POST, DELETE) is registered to the app
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
