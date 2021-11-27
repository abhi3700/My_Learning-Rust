/*
	This is the square of a no. The arg is signed, but the outpur is unsigned.
*/
use std::convert::TryInto;

fn main() {
	println!("The square of {} is {}", -3, square(-3));
}

// square of signed is unsigned
fn square(num: i32) -> u32 {
	return (num * num).try_into().unwrap();
}