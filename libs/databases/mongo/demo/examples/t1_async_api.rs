//! Using async-std
//!
//! Dependencies are:
//!
//! ```toml
//! [dependencies]
//! async-std = "1.12.0"
//! chrono = "0.4.26"
//! mongodb = {version = "2.6.0", default-features = false, features = ["async-std-runtime"]}
//! serde = "1.0.183"
//! serde_json = "1.0.104"
//! ```

use async_std;
use chrono::{TimeZone, Utc};
use mongodb::{
	bson::{self, doc, Bson},
	Client,
};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Debug)]
struct Movies {}

#[derive(Debug)]
struct User {
	name: String,
	email: String,
	password: String,
}

#[async_std::main]
pub(crate) async fn main() -> Result<(), Box<dyn Error>> {
	dotenv::from_path("./.env").expect("Failed to load the env file");

	let mongodb_uri = std::env::var("MONGODB_URI").expect("Invalid MONGODB_URI");

	// A Client is needed to connect to MongoDB:
	let client = Client::with_uri_str(mongodb_uri).await?;

	// Print the databases in our MongoDB cluster:
	println!("Databases:");
	for db_name in client.list_database_names(None, None).await? {
		println!("- {}", db_name);
	}

	// Get the 'movies' collection from the 'sample_mflix' database:
	let movies = client.database("mflix").collection::<Movies>("movies");
	println!("{:?}", movies);

	// FIXME: refer freecodecamp to try out its blog
	// let new_doc = doc! {
	// 	"title": "Parasite",
	// 	"year": 2020,
	// 	"plot": "A poor family, the Kims, con their way into becoming the servants of a rich family,
	// the Parks. But their easy life gets complicated when their deception is threatened with
	// exposure.", 	"released": Utc.with_ymd_and_hms(2020, 2, 7, 0, 0, 0)
	// };

	// println!("New Document: {}", new_doc);
	// // TODO: Get the 'users' collection from the 'mflix' database
	// let users = client.database("mflix").collection::<User>("users");
	// dbg!(users.name());

	// TODO: https://github.com/mongodb-developer/rust-quickstart-code/blob/async-std/src/main.rs

	Ok(())
}
