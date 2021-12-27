/*

	Tuple
	- Append a tuple into another tuple 
*/

fn main() {
	let x = (1, 3.4, 's', "horse");

	println!("{:?}", x);

	let h = (x, (1, 2), (4.5, 'r'));
	println!("{:?}", h);
	println!("{:#?}", h);

}