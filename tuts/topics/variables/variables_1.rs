/*
	Addition of 2 variables
	- int var are automatically typed as `i32`

*/

fn main() {
	let x: i32 = -11;
	let y: i32 = 5;
	let z: i64 = (x + y).into();		// converting from i32 to i64

	println!("The sum of {} and {} is {}", x, y, z);
}