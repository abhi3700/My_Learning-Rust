//! Continuing from "./2_3projects_join.rs" code...
//!
//! Using futures' `join!` macro, the result is returned when all the tasks are completed.
//! But using `select!` macro, the result is returned whenever any task is finished.

use futures::{pin_mut, select, FutureExt};
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
	let num1 = num1().fuse();
	let num2 = num2().fuse();
	let num3 = num3().fuse();

	pin_mut!(num1, num2, num3);

	smol::block_on(async {
		loop {
			select! {
				a = num1 => println!("num1 is completed with value: {}", a),
				b = num2 => println!("num2 is completed with value: {}", b),
				c = num3 => println!("num3 is completed with value: {}", c),
				complete => {
					println!("All are completed!");
					break;
				}
			}
		}
	});
	println!("elapsed: {}s", start.elapsed().as_secs()); // total time: 9s
}
