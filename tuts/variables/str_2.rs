/*
	Use `str`
	- assign as &str

	> NOTE: `&str` in Rust is similar to `char*` in C++
*/

fn main() {
	// let mut x: &str = "abhijit";		// throws warning: the value not used.
	
	let mut x: &str;

	x = "raman".into();		// throws warning: the value not used.
	x = "raman_2".into();

	println!("{}", x);
}