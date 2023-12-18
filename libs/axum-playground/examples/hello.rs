//! Hello world example for Axum

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our router/application with single route
    let app = Router::new().route("/", get(|| async { "hello world" }));

    // define the TCP listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on port: {}", listener.local_addr().unwrap());

    // listen to/start the server
    axum::serve(listener, app).await.unwrap();
}
