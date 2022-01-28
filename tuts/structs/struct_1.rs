/*
	Define a struct
*/

struct Greeting {
	name: String
}

fn main() {
	let greeting = Greeting { 
		name: "abhijit".to_string() 
	};

	println!("Hello, {}", greeting.name);
}