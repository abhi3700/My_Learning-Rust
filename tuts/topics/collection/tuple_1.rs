/*
	Tuple
	- create alongwith `mut` type
	- display
	- tuple max length is `12`
*/

fn main() {
	// M-1
	// let x = ('a', 2, 3.4, "abhijit");
	// M-2
	// let x: (char, u32, f64, &str) = ('a', 2, 3.4, "abhijit");

	let mut x: (char, u32, f64, &str) = ('a', 2, 3.4, "abhijit");
	x.0 = 'b';
	x.2 = 5.6;

	println!("{:?}", x);		// display horizontally
	println!("{:#?}", x);		// display vertically

	let (a, _, b, _) = x;
	println!("{}, {}", a, b);

}