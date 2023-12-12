//! Run a mini redis server to set value to a key named "hello"
//! ```sh
//! # Install the mini-redis-server
//! $ cargo install mini-redis-server
//!
//! $ mini-redis-server
//! ```
//!
//! On a separate terminal, get the value of "hello" key like this:
//! ```sh
//! $ mini-redis-cli get hello
//! ```
//!
//!
//! If server not run, then this error:
//! ```sh
//! Error: Os { code: 61, kind: ConnectionRefused, message: "Connection refused" }
//! ```

use mini_redis::{client, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let client = Arc::new(Mutex::new(client));

    // setting the value to "world"
    {
        let mut locked_client = client.lock().await;
        locked_client.set("hello", "world".into()).await?;
    }
    // getting the value
    {
        let mut locked_client = client.lock().await;
        let res = locked_client.get("hello").await?;
        println!("{:?}", res);
    }

    // setting the value to "world2"
    {
        let mut locked_client = client.lock().await;
        locked_client.set("hello2", "world2".into()).await?;
    }

    // getting the value
    {
        let mut locked_client = client.lock().await;
        let res = locked_client.get("hello2").await?;
        println!("{:?}", res);
    }

    Ok(())
}
