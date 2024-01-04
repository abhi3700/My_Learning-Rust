//! Hello world example for Axum
//!
//! Lessons:
//! - Axum server has 2 main components:
//!     - app/router
//!     - listener/address
//!
//! Problems:
//! - How to view the logs of what is happening under the hood like in Actix framework?

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // depend on tracing for all sorts of logging
    tracing_subscriber::fmt().init();

    dotenv::dotenv().ok();
    let port = std::env::var("PORT").expect("Failed to set PORT env var.");

    // build our router/application with single route
    let app = Router::new().route("/", get(|| async { "Hello World!" }));

    // define the TCP listener
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    tracing::debug!("Listening on port: {}", listener.local_addr().unwrap());

    // info!("API server started...");

    // listen to/start the server
    axum::serve(listener, app).await.unwrap();
}
