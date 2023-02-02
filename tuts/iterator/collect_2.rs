/*
    Collect method for vec (of char) into a string.
*/

use std::vec;

fn collated_char(v: &Vec<char>) -> String {
    // v.iter().map(|num| num * 2).collect()    // M-1
    v.iter().collect::<String>() // M-2
}

pub fn main() {
    let v = vec!['a', 'b', 'c', 'd'];
    println!("collated v as: {:?}", collated_char(&v)); // --> abcd
}
