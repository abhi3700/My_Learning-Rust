//! Source: https://www.freecodecamp.org/news/mongodb-in-rust/
//!
//! This is about doing CRUD operations on top of MongoDB.

// use futures_lite::StreamExt;
use futures::StreamExt;
use mongodb::{
	bson::{doc, Document},
	Client,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
	name: String,
	age: u8,
}

async fn is_coll_in_db(client: &Client, db_name: &str, coll_name: &str) -> bool {
	let db = client.database(db_name);
	db.list_collection_names(None)
		.await
		.map(|collections| collections.contains(&coll_name.to_string()))
		.unwrap_or(false)
}

async fn create_collection(
	client: &Client,
	db_name: &str,
	coll_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
	let db = client.database(db_name);
	if !is_coll_in_db(client, db_name, coll_name).await {
		db.create_collection(coll_name, None).await?;
	}

	Ok(())
}

async fn insert_document<T: Serialize>(
	client: &Client,
	db_name: &str,
	coll_name: &str,
	document: &T,
) -> Result<(), Box<dyn std::error::Error>> {
	let db = client.database(db_name);
	let coll: mongodb::Collection<T> = db.collection(coll_name);
	coll.insert_one(document, None).await?;

	Ok(())
}

async fn get_document(
	client: &Client,
	db_name: &str,
	coll_name: &str,
	filter: Document,
) -> Result<(), Box<dyn std::error::Error>> {
	let db = client.database(db_name);
	let coll: mongodb::Collection<User> = db.collection(coll_name);
	// let filter = doc! {"name": "John"};
	let result = coll.find_one(filter, None).await?;

	match result {
		Some(doc) => println!("\t{:?}", doc),
		None => println!("\tNo document found"),
	}

	Ok(())
}

async fn delete_document(
	client: &Client,
	db_name: &str,
	coll_name: &str,
	filter: Document,
) -> Result<(), Box<dyn std::error::Error>> {
	let db = client.database(db_name);
	let coll: mongodb::Collection<User> = db.collection(coll_name);
	coll.delete_one(filter, None).await?;

	Ok(())
}

async fn update_document(
	client: &Client,
	db_name: &str,
	coll_name: &str,
	filter: Document,
	update: Document,
) -> Result<(), Box<dyn std::error::Error>> {
	let db = client.database(db_name);

	let coll: mongodb::Collection<User> = db.collection(coll_name);
	coll.update_one(filter, update, None).await?;

	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// connect to local DB.
	let client = Client::with_uri_str("mongodb://localhost:27017").await?;

	let db_name = "mydatabase";
	let coll_name = "mycollection";

	// delete everything in the referred db.
	client.database(db_name).drop(None).await?;

	// Create collection
	create_collection(&client, db_name, coll_name).await?;

	// Insert multiple documents
	insert_document(&client, db_name, coll_name, &User { name: "John".to_string(), age: 28 })
		.await?;
	insert_document(&client, db_name, coll_name, &User { name: "Alice".to_string(), age: 21 })
		.await?;
	insert_document(&client, db_name, coll_name, &User { name: "Bob".to_string(), age: 24 })
		.await?;
	insert_document(&client, db_name, coll_name, &User { name: "Dave".to_string(), age: 42 })
		.await?;

	get_document(&client, db_name, coll_name, doc! {"name": "John"}).await?;

	update_document(&client, db_name, coll_name, doc! {"name": "John"}, doc! {"$set": {"age": 30}})
		.await?;

	// read now
	println!("Updated John's info:");
	get_document(&client, db_name, coll_name, doc! {"name": "John"}).await?;

	// delete document with name as "John"
	delete_document(&client, db_name, coll_name, doc! {"name": "John"}).await?;

	// read all the documents
	println!("Docs after deletion:");
	println!("____________________");
	let documents = client
		.database(db_name)
		.collection::<User>(coll_name)
		.find(None, None)
		.await?
		.collect::<Vec<_>>()
		.await;
	// let documents = docs_cursor.collect::<Vec<_>>().await;
	// print the documents
	for doc in documents {
		println!("{:#?}", doc?);
	}

	// while let Some(doc) = docs_cursor.next().await {
	// 	println!("{:#?}", doc?);
	// }

	Ok(())
}
