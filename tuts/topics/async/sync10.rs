//! [Intermediate level]
//! **Shared State with Mutex**:
//! Q. Create a shared counter using `tokio::sync::Mutex` and increment
//! it from multiple async tasks. Ensure that the final count is correct
//! by properly locking and unlocking the mutex.
//!
//! M-1 vs M-2:
//! - no. of threads: 1 & 3 respectively
//! - As the former has single thread, so no Arc smart pointer needed unlike the latter.
//! - The former has 2 different blocks.
//! - The former could use the same counter local variable in the main thread. But, the latter
//!   required to clone the local variable `counter`. Hence, Arc smart pointer is used for using its
//!   clones in multiple threads.
//!
//! In short, M-1 can't be considered as the solution for the given problem as the Q. mentioned
//! async task(s).

use std::sync::Arc;
use tokio::{join, sync::Mutex, time::Instant};

// M-1
// Here,
//  - no. of threads = 1
//  - As single thread, so
//      - no Arc smart pointer required
//      - `counter` can be directly referenced.
// #[tokio::main]
// pub(crate) async fn main() {
//     // define a shared counter
//     let counter = Mutex::new(0);

//     let start = Instant::now();
//     {
//         // sleep for 2s
//         tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
//         let mut counter = counter.lock().await;
//         *counter += 1;
//     }

//     {
//         // sleep for 4s
//         tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
//         let mut counter = counter.lock().await;
//         *counter += 1;
//     }

//     let elapsed = start.elapsed();
//     println!("took: {}s", elapsed.as_secs());
//     println!("counter: {:?}", counter.as_ref().lock().await);
// }

// M-2
// By using `move`, you are effectively transferring the ownership of `counter_clone`
// from the current function to the async block. This ensures that `counter_clone`
// will remain valid for the duration of the async block, regardless of the lifetime
// of the function where the async block was defined.
// The unlock on mutex automatically happens once the thread returns into the main thread.
#[tokio::main]
pub(crate) async fn main() -> eyre::Result<()> {
	let counter = Arc::new(Mutex::new(0));

	// create clone
	let counter_clone = counter.clone();

	// clone of pointer
	let start = Instant::now();
	let task_1 = tokio::spawn(async move {
		println!("thread-1 spawned...");
		let mut counter = counter_clone.lock().await;
		// sleep for 4s
		tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
		*counter = 32;
		println!("task-1 completed with counter = {}", *counter);
		*counter
	});

	// another clone of pointer
	let counter_clone = counter.clone();

	let task_2 = tokio::spawn(async move {
		println!("thread-2 spawned...");
		let mut counter = counter_clone.lock().await;
		// sleep for 2s
		tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
		*counter = 77;
		println!("task-2 completed with counter = {}", *counter);
		*counter
	});

	// M-2.1
	// Cons: Run sequentially (relatively) to some extent (even for millisecs).
	//      This means that `task_1` would always begin first.
	// Here, as this is lock activity, so, the task that starts first will end first.
	// task_1.await.unwrap();
	// task_2.await.unwrap();

	// M-2.2
	// Run concurrently as they are put together. This means that `task_1` or `task_2` might start
	// first. Use `join!` when you need to run multiple futures concurrently and
	// want to wait for all of them to complete before returning to the main thread.
	// Although here, as this is lock activity, so, the task that starts first will end first.
	// But, result would be thrown all at once for all tasks.
	// get the values as tuple respectively for task-1 & task-2
	let (first, second) = join!(task_1, task_2);
	let elapsed = start.elapsed();
	println!("took: {} ms", elapsed.as_millis());

	// Now, getting the 2 values from both the tasks, we want to add
	let sum_result = first? + second?;
	println!("Their sum from 2 threads = {}", sum_result);

	let result: tokio::sync::MutexGuard<'_, i32> = counter.lock().await;
	let result: i32 = *result;
	println!("Finally, counter: {:?}", result);

	Ok(())
}
