//! [Beginner level]
//! Q. Basic Async Function: Write an async function named `fetch_data` that
//! simulates fetching data from a database. Use `tokio::time::sleep`
//! to simulate a delay of 2 seconds. The function should return a String
//! that says "Data fetched".
//!
//! Key learnings:
//! 1. Add `await` on making a call on future task.

use tokio::time::{sleep, Duration};

/// Asynchronous function to simulate data fetching
async fn fetch_data() -> &'static str {
    // Simulating a delay (e.g., database operation or network call)
    sleep(Duration::from_secs(5)).await;
    "Data fetched"
}

#[tokio::main]
pub(crate) async fn main() {
    println!("Waiting for 5s...");
    // Call the async function and await its result
    let data = fetch_data().await;
    println!("{}", data);
}
