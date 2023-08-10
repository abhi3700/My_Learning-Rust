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

use mongodb::{options::ClientOptions, Client};
use std::error::Error;

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

    // TODO: https://github.com/mongodb-developer/rust-quickstart-code/blob/async-std/src/main.rs

    Ok(())
}
