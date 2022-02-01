/*
	Use `String`
	- assign as String
*/

fn main() {
	// let mut x: String = String::from("abhijit");		// throws warning: the value not used.
	
	let x: String;

	let _x: String = "raman".into();		// throws warning: the value not used.
	x = "raman_2".into();

	println!("{}", x);
}