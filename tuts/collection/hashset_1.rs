//! Implement a Rust function named remove_duplicates that takes a vector of
//! integers as an argument and returns a new vector with all duplicates removed.

use std::collections::HashSet;

/// remove duplicates from a vector using hashset.
fn remove_duplicates(v: Vec<i32>) -> Vec<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    let mut result: Vec<i32> = Vec::new();
    for i in v {
        if set.insert(i) {
            result.push(i);
        }
    }
    result
}

/// main function
fn main() {
    let v = vec![1, 2, 2, 3, 4, 4, 5, 6, 7, 7];
    println!("removed duplicates: {:?}", remove_duplicates(v));
}
