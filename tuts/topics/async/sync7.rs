//! [Beginner level]
//! Q. **Sequential Asynchronous Tasks**: Write a program using Tokio
//! where you have two async functions, `task_one` and `task_two`.
//! Each should print a message and wait for 1 second.
//! Execute these tasks sequentially in the `main` function.
//!
//! Key learnings:
//! - Running this program will demonstrate sequential execution of the two tasks with a 1-second pause for each.

use tokio::time::{sleep, Duration};

// Asynchronous function to perform task one
async fn task_one() {
    println!("Task one started");
    sleep(Duration::from_secs(1)).await;
    println!("Task one completed");
}

// Asynchronous function to perform task two
async fn task_two() {
    println!("Task two started");
    sleep(Duration::from_secs(1)).await;
    println!("Task two completed");
}

#[tokio::main]
pub(crate) async fn main() {
    // Execute task one and wait for it to complete
    task_one().await;

    // Execute task two and wait for it to complete
    task_two().await;
}
