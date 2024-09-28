//! GET /
//! ```
//! $ curl localhost:3000
//! Hello, World!
//! ```
//! =====================================================================================
//! POST /users
//! ```
//! $ curl -H "Content-Type: application/json" -d @body.json localhost:3000/users -X POST
//! {"id":1337,"username":"abhi3700"}
//! ```
//!
//! =====================================================================================
//! or just play with the `axum_jon_request.http` file in the same directory

use std::collections::HashMap;

use axum::{
	extract::Query,
	http::StatusCode,
	routing::{get, post},
	Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
	// initialize tracing
	tracing_subscriber::fmt::init();

	// build our application with a route
	let app = Router::new()
		// `GET /` goes to `root`
		.route("/", get(root))
		// `POST /users` goes to `create_user`
		.route("/users", post(create_user));

	// run our app with hyper, listening globally on port 3000
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	tracing::debug!("Listening on 0.0.0.0:3000");

	// let (shutdown, rx) = tokio::sync::oneshot::channel::<()>();
	// let _ = shutdown.send(());
	axum::serve(listener, app)
		// for handling graceful shutdown
		// .with_graceful_shutdown(async move {
		// 	let _ = rx.await;
		// })
		.await
		.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
	"Hello, World!"
}

async fn create_user(
	// it is for extracting query params
	// Query(param): Query<HashMap<String, String>>,
	// this argument tells axum to parse the request body
	// as JSON into a `CreateUser` type
	Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
	// let query = param.get("my_param").unwrap();

	// insert your application logic here
	let user = User { id: 1337, username: payload.username };

	// you could panic the server. but it won't hung the other routes.
	// panic!();

	// this will be converted into a JSON response
	// with a status code of `201 Created`
	(StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
	username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
	id: u64,
	username: String,
}
