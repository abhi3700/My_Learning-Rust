//! [Beginner level]
//! Q. **Error Handling in Async**:
//! Modify the `fetch_data` function from the first question to
//! return a `Result<String, String>`. Simulate a failure scenario
//! where the function returns an `Err` saying `"Fetch failed"`.
//! In your `main` function, handle this error gracefully.

use tokio::time::{sleep, Duration};

// Asynchronous function modified to return a Result
async fn fetch_data() -> Result<String, String> {
    // Simulating a delay
    sleep(Duration::from_secs(2)).await;

    // Simulating a failure scenario
    if true {
        // Replace `true` with a condition to simulate success or failure
        Err("Fetch failed".to_string())
    } else {
        Ok("Data fetched".to_string())
    }
}

#[tokio::main]
pub(crate) async fn main() {
    println!("Waiting for 2s...");

    // Handle the Result returned by fetch_data
    match fetch_data().await {
        Ok(data) => println!("Success: {}", data),
        Err(error) => println!("Error: {}", error),
    }
}
