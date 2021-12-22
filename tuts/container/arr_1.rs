/*
	Use array
	- define array with elements
	- print elements via for-loop
*/

fn main() {
	let x: [i32; 5] = [41, 53, 45, 37, 89];

	// M-1
	for i in 0..x.len() {
		println!("{}", x[i]);
	}

	// M-2
	println!("{:?}", x);

	// M-3
	println!("{:#?}", x);
}