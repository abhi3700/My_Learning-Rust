use std::time::Duration;

use tokio::time::sleep;

// here, current_thread enforces single thread & executes by time slicing.
// Otherwise, there would be more than 1 thread & the OS would switch from
// one thread to another at any given time.
#[tokio::main/* (flavor = "current_thread") */]
pub async fn main() {
    let mut handles = vec![];
    for i in 0..2 {
        let handle = tokio::spawn(async move { my_function(i).await });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_function(i: i32) {
    println!("[{i}] I'm a async function");
    let s1 = read_from_database().await;
    println!("[{i}]First result: {s1}");
    let s2 = read_from_database().await;
    println!("[{i}] Second result: {s2}");
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(500)).await; // sleep for 50 ms
    "DB read".to_string()
}
