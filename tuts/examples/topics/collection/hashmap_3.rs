//! HashMap
//! ================
//! - insert is for replacing a value if the key exists no matter what.
//! - entry is for conditionally replacing a value. If key exists, leave it as is.

use std::collections::HashMap;

fn main() {
	let mut h1 = HashMap::new();

	h1.insert(1, "a");
	h1.insert(2, "b");
	h1.insert(1, "c");
	h1.entry(1).or_insert("d");

	assert_eq!(h1.get(&1), Some(&"c"));
}
