/*
	Define a struct & implement a new function
*/

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

fn main() {
	let greeting = Greeting::new("abhijit");

	println!("Hello, {}", greeting.name);
}