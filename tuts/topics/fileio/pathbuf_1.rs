//! Show PathBuf example

use std::path::PathBuf;

pub(crate) fn main() {
	let mut pathbuf = PathBuf::new();
	pathbuf.push("school");
	pathbuf.push("class-6");
	pathbuf.push("result.txt");
	assert_eq!(pathbuf.to_str(), Some("school/class-6/result.txt"));
}
