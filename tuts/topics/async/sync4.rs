//! This module demonstrates how to use `Arc` and `Mutex` to share data between threads.
//!
//! The `main` function creates an `Arc` wrapped `Mutex` to store an integer value. It then spawns three threads
//! to modify and read the value of the integer. Each thread has its own reference to the `Arc` wrapped `Mutex`.
//!
//! The first thread modifies the value of the integer to 100, the second thread reads the value of the integer,
//! and the third thread modifies the value of the integer to 150. The final value of the integer is printed
//! at the end of the `main` function.
//!
//! Note that the final value of the integer may not be deterministic as it depends on which thread runs last.
//! The `assert_eq!` macro is used to ensure that the final value of the integer is 150.
//!
//! # Examples
//!
//! ```
//! use std::sync::{Arc, Mutex};
//! use std::thread;
//!
//! let data: Arc<Mutex<i32>> = Arc::new(Mutex::new(42));
//!
//! let thread1_data = data.clone();
//! let thread2_data = data.clone();
//! let thread3_data = data.clone();
//!
//! let thread1 = thread::spawn(move || {
//!     let mut new_data = thread1_data.lock().unwrap();
//!     *new_data = 100;
//! });
//!
//! let thread2 = thread::spawn(move || {
//!     println!(
//!         "read original data from thread 2: {}",
//!         thread2_data.lock().unwrap()
//!     );
//! });
//!
//! let thread3 = thread::spawn(move || {
//!     let mut new_data = thread3_data.lock().unwrap();
//!     *new_data = 150;
//! });
//!
//! let _ = thread1.join();
//! let _ = thread2.join();
//! let _ = thread3.join();
//!
//! assert_eq!(*data.lock().unwrap(), 150);
//! ```
use std::sync::{Arc, Mutex};
use std::thread;

pub(crate) fn main() {
    let data: Arc<Mutex<i32>> = Arc::new(Mutex::new(42));

    println!("original data [Start]: {}", data.lock().unwrap());

    // let thread1_data = Arc::clone(&data);
    let thread1_data = data.clone();
    let thread2_data = data.clone();
    let thread3_data = data.clone();

    // modify original data inside thread 1
    let thread1 = thread::spawn(move || {
        // lock the thread1 data
        let mut new_data = thread1_data.lock().unwrap();
        // modify the original data after locking
        *new_data = 100;
        println!("original data modified as: {} in thread 1", new_data);
    })
    .join();

    // read the data inside thread 2
    let thread2 = thread::spawn(move || {
        println!(
            "read original data from thread 2: {}",
            thread2_data.lock().unwrap()
        );
    })
    .join();

    // modify original data inside thread 3
    let thread3 = thread::spawn(move || {
        // lock the thread1 data
        let mut new_data = thread3_data.lock().unwrap();
        // modify the original data after locking
        *new_data = 150;
        println!("original data modified as {} from thread 3", new_data);
    })
    .join();

    // wait for the threads to join
    // let _ = thread1.join();
    // let _ = thread2.join();
    // let _ = thread3.join();

    println!("original data [End]: {}", data.lock().unwrap());
    // NOTE: this might fail as the value returned by the last
    // thread could be anything 100 or 150 depending on which thread (editing one)
    // runs last:
    // 123: last value - 150
    // 132: last value - 150
    // 213: last value - 150
    // 231: last value - 100
    // 312: last value - 100
    // 321: last value - 100
    //
    // Illustration:
    // original data [Start]: 42
    // original data modified as 150 from thread 3
    // original data modified as: 100 in thread 1
    // read original data from thread 2: 100
    // original data [End]: 100
    // thread 'main' panicked at 'assertion failed: `(left == right)`
    //   left: `100`,
    //  right: `150`', src/main.rs:89:5
    assert_eq!(*data.lock().unwrap(), 150);
}
