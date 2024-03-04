//! Source: https://www.freecodecamp.org/news/mongodb-in-rust/
//!
//! This is about doing CRUD operations on top of MongoDB.

use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
	name: String,
	age: u8,
}

async fn create_collection(client: &Client, db_name: &str, coll_name: &str) {
	let db = client.database(db_name);
	db.create_collection(coll_name, None).await.unwrap();
}

async fn insert_document(client: &Client, db_name: &str, coll_name: &str) {
	let db = client.database(db_name);
	let coll = db.collection(coll_name);
	let doc = doc! {"name": "John", "age": 28};
	coll.insert_one(doc, None).await.unwrap();
}

async fn get_document(client: &Client, db_name: &str, coll_name: &str) {
	let db = client.database(db_name);
	let coll: mongodb::Collection<User> = db.collection(coll_name);
	let filter = doc! {"name": "John"};
	let result = coll.find_one(filter, None).await;

	match result {
		Ok(Some(doc)) => println!("{:?}", doc),
		Ok(None) => println!("No document found"),
		Err(_) => todo!(),
	}
}
async fn delete_document(client: &Client, db_name: &str, coll_name: &str) {
	let db = client.database(db_name);
	let coll: mongodb::Collection<User> = db.collection(coll_name);
	let filter = doc! {"name": "John"};
	coll.delete_one(filter, None).await.unwrap();
}

async fn update_document(client: &Client, db_name: &str, coll_name: &str) {
	let db = client.database(db_name);
	let coll: mongodb::Collection<User> = db.collection(coll_name);
	let filter = doc! {"name": "John"};
	let update = doc! {"$set": {"age": 30}};
	coll.update_one(filter, update, None).await.unwrap();
}

#[async_std::main]
async fn main() {
	let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
	let client = Client::with_options(client_options).unwrap();

	let db_name = "mydatabase";
	let coll_name = "mycollection";

	create_collection(&client, db_name, coll_name).await;

	insert_document(&client, db_name, coll_name).await;

	get_document(&client, db_name, coll_name).await;

	delete_document(&client, db_name, coll_name).await;

	update_document(&client, db_name, coll_name).await;
}
