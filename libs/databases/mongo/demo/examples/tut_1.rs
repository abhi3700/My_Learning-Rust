//! Quickstart tutorial
//! Source: https://www.mongodb.com/docs/drivers/rust/current/quick-start/connect-to-mongodb/

use mongodb::{
	bson::{doc, Document},
	Client, Collection,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
	dotenv::from_path("./.env").expect("Failed to load the env file");

	let mongodb_uri = std::env::var("MONGODB_URI").expect("Invalid MONGODB_URI");

	// Create a new client and connect to the server
	let client = Client::with_uri_str(mongodb_uri).await?;

	// Get a handle on the movies collection
	let database = client.database("sample_mflix");
	let my_coll: Collection<Document> = database.collection("movies");

	// Find a movie based on the title value
	let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }, None).await?;

	// Print the document
	println!("Found a movie:\n{:#?}", my_movie);
	Ok(())
}
