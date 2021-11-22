/*
    Addition of 2 no.s
*/

// Addition function
fn add(x: i32, y: i32) -> i32 {
	return x + y;
}

// main function
fn main() {
	let x = 1;
	let y = 2;
	
	println!("Addition of {0} & {1} is {2}", x, y, add(x, y));
}