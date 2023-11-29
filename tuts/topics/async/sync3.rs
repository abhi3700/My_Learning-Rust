//! Arc pointer simple example to read data in multiple threads
//!
//! This function demonstrates the use of Arc (Atomic Reference Counting) to share ownership of immutable data between threads.
//! It creates two threads and shares the ownership of data between them using Arc::clone().
//! The threads read the data and print it to the console.
//! Finally, the original data is printed to the console.

use std::sync::Arc;

use std::thread;

pub(crate) fn main() {
    // Create a mutable reference to an Arc pointer that holds an integer value of 42.
    let mut data = &Arc::new(42);

    // Print the original data before sharing it between threads.
    println!("original data Before: {}", data);

    // Clone the Arc pointer to share ownership of the data between threads.
    let thread1_data = Arc::clone(&data);
    let thread2_data = Arc::clone(&data);

    // Spawn two threads to read the shared data and print it to the console.
    let handle_1 = thread::spawn(move || {
        println!("data read in handle 1: {}", thread1_data);
    });

    let handle_2 = thread::spawn(move || {
        println!("data read in handle 2: {}", thread2_data);
    });

    // Wait for the threads to finish executing.
    let _ = handle_1.join();
    let _ = handle_2.join();

    // Print the original data after sharing it between threads.
    println!("original data After: {}", data);
}
