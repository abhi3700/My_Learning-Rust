/*
    Collect method for vec (of number) which is modified
    into a vector with number multiplied by 2.
*/

use std::vec;

fn updated_vec(v: &Vec<i32>) -> Vec<i32> {
    // v.iter().map(|num| num * 2).collect()    // M-1
    v.iter().map(|num| num * 2).collect::<Vec<i32>>() // M-2
}

pub fn main() {
    let v = vec![1, 3, 4, 7, 8, 9];
    println!("updated v: {:?}", updated_vec(&v));
}
