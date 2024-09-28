//! Greetings with custom name in the path
//!
//! Example:
//! > localhost:3000/greet/abhi GET
//! < Good morning, abhi
//!
//! > localhost:3000/greet/adi GET
//! < Good morning, adi

use axum::{extract::Path, routing::get, Router};
use tokio::net::TcpListener;

async fn greet(Path(n): Path<String>) -> String {
	format!("Good morning, {}", n)
}

#[tokio::main]
async fn main() {
	// initialize tracing
	tracing_subscriber::fmt::init();

	let app = Router::new()
		.route("/greet/:name", get(greet))
		.route("/", get(|| async { "Hello world" }));

	let listener = TcpListener::bind("127.0.0.1:3000").await.expect("Error in binding address");
	tracing::debug!("Listening on {}", listener.local_addr().unwrap());

	axum::serve(listener, app).await.expect("Error in initiating server");
}
