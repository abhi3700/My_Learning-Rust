/*
    Used collect() on top of enumerate() in order to get (index, value).
    Observe that it has been implemented differently for different vector type.
*/

use std::vec;

fn enumerated_vec_string(v: &Vec<String>) -> Vec<(usize, &String)> {
    v.iter().enumerate().collect::<Vec<(usize, &String)>>()
}

fn enumerated_vec_num(v: &Vec<i32>) -> Vec<(usize, i32)> {
    // v.iter().enumerate().collect::<Vec<(usize, i32)>>()
    println!("enumerated: {:?}", v.iter().enumerate());
    v.iter()
        .enumerate()
        .map(|(i, x)| (i, *x))
        .collect::<Vec<(usize, i32)>>()
}

pub fn main() {
    let v = vec![1, 3, 4, 7, 8, 9];
    let vs = vec![
        "abhijit".to_string(),
        "alex".to_string(),
        "charlie".to_string(),
    ];
    println!(
        "enumerated v & then collected as: {:?}",
        enumerated_vec_num(&v)
    );
    println!(
        "enumerated vs & then collected as: {:?}",
        enumerated_vec_string(&vs)
    );
}
