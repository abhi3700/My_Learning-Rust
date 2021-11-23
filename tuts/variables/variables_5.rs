/*
	Addition of 2 float32 type variables.

	Observation:
	- float var defined in 2 methods
	- auto type deduction in `z`
*/

fn main() {
	let x: f32 = 1.35;
	let y = 3.4f32;
	let z = x + y;

	println!("x + y i.e. {} + {} is {}", x, y, z);
}