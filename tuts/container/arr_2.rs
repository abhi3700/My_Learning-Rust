/*
	Use array
	- define fixed size with values, 
	- define zero, value array
	- set elements
	- print elements via for-loop
*/

fn main() {
	let mut x: [i32; 5] = [1, 2, 3, 4, 5];

	// define zero array
	// let mut d: [0; 5];

	// define value array
	// let mut e: ["x"; 5];

	// modify elements, but can't change the size even if it of `mut`
	x[0] = 11;
	x[1] = 21;
	x[2] = 31;
	x[3] = 41;
	x[4] = 51;

	// M-1
	for i in 0..x.len() {
		println!("{}", x[i]);
	}

	// M-2
	println!("{:?}", x);

	// M-3
	println!("{:#?}", x);
}