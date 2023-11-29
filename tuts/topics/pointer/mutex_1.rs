//! It is mainly used in concurrency to share data between threads.
//! Before making any CRUD operation, we need to lock the mutex
//! with mutex.lock() method i.e. MutexGuard.
//!
//! For more, read from [here](../../README.md#mutex)

use std::sync::Mutex;

pub fn main() {
    let x = Mutex::new(10);
    {
        let mut y = x.lock().unwrap();
        *y += 1;
        println!("y = {:?}", y)
    }

    println!("x = {:?}", x);
}
