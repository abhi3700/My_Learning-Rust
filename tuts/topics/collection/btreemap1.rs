//! A BTreeMap example

use std::collections::BTreeMap;

pub(crate) fn main() {
    let mut btm1 = BTreeMap::new();

    // insertion
    btm1.insert("alice", 100);
    btm1.insert("bob", 45);

    // read
    assert_eq!(*btm1.get("alice").unwrap(), 100);

    // update
    btm1.insert("alice", 101);
    assert_eq!(*btm1.get("alice").unwrap(), 101);

    // deletion
    btm1.remove("alice");
    assert_eq!(btm1.get("alice"), None);
}
