/*
	- Define a struct
	- define display so as to print the greeting var directly
*/

use std::fmt;

struct Greeting {
	name: String
}

impl Greeting {
	fn new(name: &str) -> Self {
		Greeting {
			name: name.to_string(),
		} 
	}
}

impl fmt::Display for Greeting {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Hello, {}", self.name)
	}
}

fn main() {
	let greeting = Greeting::new("abhijit");

	println!("{}", greeting);
}