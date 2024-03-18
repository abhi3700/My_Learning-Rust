fn might_fail() -> Result<(), String> {
	// Some operation that might fail
	Err("Oops, something went wrong!".to_string())
}

pub(crate) fn main() -> Result<(), String> {
	might_fail()?;
	println!("This line won't run if `might_fail` returns an Err.");
	Ok(())
}
