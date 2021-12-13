/*
	vector
	- initialize
	- push()
	- pop()
	- len()
	- is_empty()
*/

fn main() {
	let mut v = Vec::new();
	assert!(v.is_empty());			// check the v is empty

	// push & pop
	v.push(1);
	v.push(2);
	v.push(32);
	v.push(24);
	v.pop();

	// assertion
	assert_eq!(v.len(), 3);
	assert_eq!(v, [1, 2, 32]);
	v[0] = 7;
	assert_eq!(v, [7, 2, 32]);
	assert_eq!(v[1], 2);
	assert!(!v.is_empty());

	// print the elements
	for x in &v {
		println!("{} ", x);
	}

}