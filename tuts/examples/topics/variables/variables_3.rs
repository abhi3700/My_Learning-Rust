/*
	Changing value of an immutable variable.
*/

fn main() {
	let x = 5;
	println!("the value of x is {}", x);
	x = 6;			// ERROR as it's not a mutable var
	println!("the value of x is {}", x);
}