/*
    Addition of 2 no.s
*/

// main function
fn main() {
	let x = 1;
	let y = 2;
	
	// println!("Addition of {0} & {1} is {2}", x, y, add(x, y));
	println!("Addition of {} & {} is {}", x, y, add(x, y));
}

// Addition function
fn add(x: i32, y: i32) -> i32 {
	x + y
}

