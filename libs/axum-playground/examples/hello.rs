//! Hello world example for Axum

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // depend on tracing for all sorts of logging
    tracing_subscriber::fmt().init();
    // env_logger::init();

    // build our router/application with single route
    let app = Router::new().route("/", get(|| async { "Hello World!" }));

    // define the TCP listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("Listening on port: {}", listener.local_addr().unwrap());

    // listen to/start the server
    axum::serve(listener, app).await.unwrap();
}
