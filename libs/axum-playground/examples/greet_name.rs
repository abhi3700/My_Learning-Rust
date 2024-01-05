//! Greetings with custom name in the path
//!
//! Example:
//! > localhost:3000/greet/abhi
//! < Good morning, abhi
//!
//! > localhost:3000/greet/adi
//! < Good morning, adi
//!

use axum::{extract::Path, routing::get, Router};
use tokio::net::TcpListener;

async fn greet(Path(n): Path<String>) -> String {
    format!("Good morning, {}", n)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/greet/:name", get(greet))
        .route("/", get(|| async { "Hello world" }));

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Error in binding address");

    axum::serve(listener, app)
        .await
        .expect("Error in initiating server");
}
