//! Hello world example for Axum
//!
//! Lessons:
//! - Axum server has 2 main components:
//!     - app/router
//!     - listener/address
//!
//! Problems:
//! - TODO: How to view the logs of what is happening under the hood like in Actix framework?

use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	// initialize tracing
	tracing_subscriber::fmt::init();

	dotenv::dotenv().ok();
	// let port = std::env::var("PORT").expect("Failed to set PORT env var.");

	// build our router/application with single route
	let app = Router::new().route("/", get(|| async { "Hello World!" }));

	// define the TCP listener
	let listener = TcpListener::bind("127.0.0.1:3000").await.expect("Error in binding address");
	tracing::debug!("Listening on {}", listener.local_addr().unwrap());

	// listen to/start the server
	axum::serve(listener, app).await.unwrap();
}
