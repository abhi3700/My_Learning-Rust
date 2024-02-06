//! In the YT video: https://www.youtube.com/watch?v=Qi4mBTyg2SA, the implementation is about building 3 projects for 3 clients.
//!
//! 👶🏼 - Project-1
//! 🧓🏻 - Project-2
//! 🧑 - Project-3
//!
//! So, the project could be done in 2 ways:
//!
//! [project-1a] -> [project-1b] -> [project-1c] -> [project-2a] -> [project-2b] -> [project-2c] ->
//! [project-3a] -> [project-3b] -> [project-3c]
//!
//! In this, latter clients are unhappy as their projects are finished late, given all 3 clients
//! approached at the same time.
//!
//! So, instead let's go in this fashion:
//! [project-1a] -> [project-2a] -> [project-3a] -> [project-1b] -> [project-2b] -> [project-3b] ->
//! [project-1c] -> [project-2c] -> [project-3c].
//!
//! So, relatively, they are happy.
//!
//! But, the best way would be with 3 workers who handle their individual project concurrently.
//!
//!
//! ## Cons
//!
//! Using futures' `join!` macro, the result is returned when all the tasks are completed.

use futures::join;
use std::{
	thread::sleep,
	time::{Duration, Instant},
};

async fn num1() -> u8 {
	sleep(Duration::from_secs(1)); // represents some computing that takes 1 sec.
	10
}
async fn num2() -> u8 {
	sleep(Duration::from_secs(3)); // represents some computing that takes 3 sec.
	20
}
async fn num3() -> u8 {
	sleep(Duration::from_secs(5)); // represents some computing that takes 5 sec.
	30
}

fn main() {
	let start = Instant::now();
	let num1 = num1();
	let num2 = num2();
	let num3 = num3();

	let result = smol::block_on(async { join!(num1, num2, num3) });
	println!("elapsed: {}s", start.elapsed().as_secs()); // total time: 9s

	assert_eq!(result, (10, 20, 30));
}
