//! ## `main` function
//!
//! The `main` function demonstrates how to use a `Mutex` and a `Condvar` to coordinate the execution of multiple threads.
//!
//! ### Arguments
//!
//! None.
//!
//! ### Return value
//!
//! None.
//!
//! ### Example
//!
//! ```rust
//! use chrono::Utc;
//! use std::sync::{Arc, Condvar, Mutex};
//! use std::thread;
//!
//! fn main() {
//!     // Create an Arc containing a tuple of a Mutex and a Condvar
//!     let data = Arc::new((Mutex::new(false), Condvar::new()));
//!
//!     let start = chrono::Utc::now().timestamp_nanos();
//!     // create the first thread
//!     let data1 = data.clone();
//!     let thread1 = thread::spawn(move || {
//!         // Lock the Mutex and set the value to true
//!         let &(ref lock, ref cvar) = &*data1;
//!         let mut started = lock.lock().unwrap();
//!         *started = true;
//!         println!("work done in thread 1");
//!         // Notify the Condvar that the value has been set
//!         cvar.notify_one();
//!     });
//!     let elapsed = chrono::Utc::now().timestamp_nanos().checked_sub(start);
//!     println!("{} ns elapsed in running thread 1", elapsed.unwrap());
//!
//!     let start = chrono::Utc::now().timestamp_nanos();
//!     // create the second thread
//!     let data2 = data.clone();
//!     let thread2 = thread::spawn(move || {
//!         let &(ref lock, ref cvar) = &*data2;
//!         let mut started = lock.lock().unwrap();
//!         let start = Utc::now().timestamp_nanos();
//!         // Wait until the value of the Mutex is set to true
//!         while !*started {
//!             started = cvar.wait(started).unwrap();
//!         }
//!         println!(
//!             "Thread 2 started after waiting for {} ns",
//!             Utc::now().timestamp_nanos().checked_sub(start).unwrap()
//!         );
//!         println!("work done after wait");
//!     });
//!     let elapsed = chrono::Utc::now().timestamp_nanos().checked_sub(start);
//!     println!("{} ns elapsed in running thread 2", elapsed.unwrap());
//!
//!     // wait for the threads to finish
//!     thread1.join().unwrap();
//!     thread2.join().unwrap();
//! }
//! ```
//!
//! This code creates an `Arc` containing a tuple of a `Mutex` and a `Condvar`. It then creates two threads, `thread1` and `thread2`, that both have access to the `Arc`.
//!
//! In `thread1`, the code locks the `Mutex` and sets the value to `true`. It then prints a message indicating that work has been done in `thread1`. Finally, it notifies the `Condvar` that the value has been set.
//!
//! In `thread2`, the code locks the `Mutex` and waits until the value of the `Mutex` is set to `true` by calling `cvar.wait()`. This method releases the lock on the `Mutex` and waits for a notification from `thread1` that the value has been set. Once the notification is received, `wait()` re-acquires the lock on the `Mutex` and returns the new value of `started`. The code then prints a message indicating that `thread2` has started and that work has been done after the wait.
//!
//! After both threads have been spawned, the code waits for them to finish by calling `join()` on each thread.
//!
//! Overall, this code demonstrates how to use a `Mutex` and a `Condvar` to coordinate the execution of multiple threads. The `Mutex` allows the threads to synchronize access to shared data, while the `Condvar` allows the threads to wait for notifications from each other.
//!

// TODO: Understand more deeply in terms of spawning more threads & check their output

use chrono::Utc;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    // Create an Arc containing a tuple of a Mutex and a Condvar
    let data = Arc::new((Mutex::new(false), Condvar::new()));

    let start = chrono::Utc::now().timestamp_nanos();
    // create the first thread
    let data1 = data.clone();
    let thread1 = thread::spawn(move || {
        // Lock the Mutex and set the value to true
        let &(ref lock, ref cvar) = &*data1;
        let mut started = lock.lock().unwrap();
        *started = true;
        println!("work done in thread 1");
        // Notify the Condvar that the value has been set
        cvar.notify_one();
    });
    let elapsed = chrono::Utc::now().timestamp_nanos().checked_sub(start);
    println!("{} ns elapsed in running thread 1", elapsed.unwrap());

    let start = chrono::Utc::now().timestamp_nanos();
    // create the second thread
    let data2 = data.clone();
    let thread2 = thread::spawn(move || {
        let &(ref lock, ref cvar) = &*data2;
        let mut started = lock.lock().unwrap();
        let start = Utc::now().timestamp_nanos();
        // Wait until the value of the Mutex is set to true
        while !*started {
            started = cvar.wait(started).unwrap();
        }
        println!(
            "Thread 2 started after waiting for {} ns",
            Utc::now().timestamp_nanos().checked_sub(start).unwrap()
        );
        println!("work done after wait");
    });
    let elapsed = chrono::Utc::now().timestamp_nanos().checked_sub(start);
    println!("{} ns elapsed in running thread 2", elapsed.unwrap());

    // wait for the threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
}
