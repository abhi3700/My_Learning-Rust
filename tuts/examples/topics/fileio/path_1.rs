//! About Path type

use std::path::Path;

pub(crate) fn main() {
	let path: &Path = Path::new("foo.txt");
	assert_eq!(path.to_str(), Some("foo.txt"));
}
