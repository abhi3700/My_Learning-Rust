fn might_fail() -> Result<(), String> {
	// Some operation that might fail
	Err("Oops, something went wrong!".to_string())
}

pub(crate) fn main() {
	might_fail().expect("Failed to complete the operation");
	println!("This line won't run if `might_fail` returns an Err, as the program will panic.");
}
