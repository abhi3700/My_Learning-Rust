/*
	Tuple
	- read tuple into different variables.
	- display variables
*/

fn main() {
	let mut x: (char, u32, f64, &str) = ('a', 2, 3.4, "abhijit");

	println!("{:?}", x);		// diplay horizontally
	println!("{:#?}", x);		// diplay vertically

	let (a, _, b, _) = x;
	println!("{}, {}", a, b);
}