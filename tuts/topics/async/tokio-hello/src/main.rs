//! **I/O bound Async**
//! ===================
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
//!
//! Commits (in increasing order of efficient code):
//! 1. [x] Simple connect to Redis server & set/get {hello: world}
//! 2. [x] Sharing resource via Arc & Mutex: https://github.com/abhi3700/My_Learning-Rust/blob/751f6bd6ec88bcb75fa7bcc26771b39df4f3b840/tuts/topics/async/tokio-hello/src/main.rs
//!     The different scopes in this code are used to ensure that the lock on the `Mutex` is released as soon as it's no longer needed.
//!     In Rust, a `MutexGuard` (the type returned by `lock().await`) is automatically unlocked when it goes out of scope. By using
//!     separate scopes for each operation, the code ensures that the lock is released immediately after each operation completes,
//!     allowing other tasks to acquire the lock if needed.
//!     If all operations were performed in the same scope, the lock would be held for the entire duration of the scope, potentially
//!     preventing other tasks from acquiring the lock even when it's not actively being used. This could lead to unnecessary contention
//!     and reduced concurrency.
//!     So, the use of different scopes in this code is a way to manage the lifetime of the `MutexGuard` and ensure that locks are held
//!     for the shortest time possible.
//!     Use `RwLock` instead of Mutex. But not recommended in this as it won't add any efficiency.
//!     If multiple threads are frequently trying to acquire the same lock, it can lead to performance issues. Consider using finer-grained
//!     locks, or other synchronization primitives like RwLock which allows multiple readers or one writer at a time.
//! 3. [ ] Concurrent approach of handling I/O tasks: https://github.com/abhi3700/My_Learning-Rust/blob/8821bbb12f8ec4b55cceb60e8270921b2d0ef3d0/tuts/topics/async/tokio-hello/src/main.rs

use mini_redis::{client, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let client = Arc::new(Mutex::new(client));

    // Concurrently setting the values to "world" and "world2"
    let client_clone = Arc::clone(&client);
    let set_hello = tokio::spawn(async move {
        let mut locked_client = client_clone.lock().await;
        locked_client.set("hello", "world".into()).await
    });

    let client_clone = Arc::clone(&client);
    let set_hello2 = tokio::spawn(async move {
        let mut locked_client = client_clone.lock().await;
        locked_client.set("hello2", "world2".into()).await
    });

    // Wait for the set operations to complete
    set_hello.await??;
    set_hello2.await??;

    // Concurrently getting the values of "hello" and "hello2"
    let client_clone = Arc::clone(&client);
    let get_hello = tokio::spawn(async move {
        let mut locked_client = client_clone.lock().await;
        locked_client.get("hello").await
    });

    let client_clone = Arc::clone(&client);
    let get_hello2 = tokio::spawn(async move {
        let mut locked_client = client_clone.lock().await;
        locked_client.get("hello2").await
    });

    // Wait for the get operations to complete and print results
    println!("{:?}", get_hello.await??);
    println!("{:?}", get_hello2.await??);

    Ok(())
}
