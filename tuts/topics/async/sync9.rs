//! [Intermediate level]
//! **Concurrent Tasks with `join!`**:
//! Q. Write a program that starts two async tasks. One task should
//! simulate a file download, and the other should simulate data processing.
//! Use `tokio::join!` to run these tasks concurrently and print the time
//! taken for both tasks to complete.
//!
//! A. Key learnings:
//! - Run in logging mode like this:
//! ```sh
//! ❯ RUST_LOG=trace cargo r -r
//!     Finished release [optimized] target(s) in 0.14s
//!      Running `/Users/abhi3700/F/coding/github_repos/My_Learning-Rust/target/release/tuts`
//! [2023-12-13T08:30:41Z INFO  tuts::sync9] Thread for task-1: ThreadId(11)
//! [2023-12-13T08:30:41Z INFO  tuts::sync9] Thread for task-2: ThreadId(8)
//! File downloading started...
//! data processing started...
//! File downloading completed...
//! data processing completed...
//! Total time taken: 5s
//! ```
//!
//! NOTE: Here, we can see the details of when thread (with ID) is spawned
//! & when the activity is happening in terms of sequence.
//! The sequence might get altered though as these are async operations.
//!
//! - I ran 3 times. I got different times elapsed: 5002545us/5001562us/5002297us.
//! Observe that there is a difference of 0.3 to 1 ms.

use tokio::{
    join,
    time::{sleep, Duration, Instant},
};

async fn task1() {
    println!("File downloading started...");
    sleep(Duration::from_secs(3)).await;
    println!("File downloading completed...");
}

async fn task2() {
    println!("data processing started...");
    sleep(Duration::from_secs(5)).await;
    println!("data processing completed...");
}

#[tokio::main]
pub(crate) async fn main() {
    env_logger::init();

    let start = Instant::now();

    // USAGE: Use `tokio::spawn` when tasks are independent, or when you need to keep
    // the main flow unblocked. It's also useful when you want tasks to continue
    // running even if the part of your program that spawned them moves on.
    // Here, whichever task completes first can return to the main thread.

    let file_download = tokio::spawn(async move {
        log::info!("Thread for task-1: {:?}", std::thread::current().id());
        task1().await;
    });

    let data_processing = tokio::spawn(async move {
        log::info!("Thread for task-2: {:?}", std::thread::current().id());
        task2().await;
    });

    // ==== M-1 (start)
    // This ensures that `file_download` would always begin first. So, relatively sequential than M-2.
    // But, either of them can return to the main thread when finished.
    let _ = file_download.await;
    let _ = data_processing.await;
    // ==== M-1 (end)

    // ==== M-2 (start)
    // Here, any task can begin first. So, relatively concurrent than M-1.
    // Use `join!` when you need to run multiple futures concurrently and
    // want to wait for all of them to complete before returning to the main thread.
    // let _ = join!(file_download, data_processing);
    // ==== M-2 (end)

    let elapsed = start.elapsed();
    println!("Total time taken: {}us", elapsed.as_micros());
}
