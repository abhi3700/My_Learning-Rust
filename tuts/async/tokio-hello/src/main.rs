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

async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // setting the value to "world"
    client.set("hello", "world".into()).await?;
    let res = client.get("hello").await?;
    println!("{:?}", res);

    // setting the value to "world2"
    client.set("hello", "world2".into()).await?;
    let res = client.get("hello").await?;
    println!("{:?}", res);

    Ok(())
}
