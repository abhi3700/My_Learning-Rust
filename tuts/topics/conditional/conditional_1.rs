/*
    if even no., print even
    else, print odd
*/


// main function
fn main() {
	let num = 23;
	
	if num % 2 == 0 {
		println!("The number {} is even", num);
	} else {
		println!("The number {} is odd", num);
	}
}