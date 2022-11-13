use core::num;
#[allow(unused_imports)]
use rand::Rng;
use std::io;
use std::io::Read;

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

#[allow(unused)]
fn main() {
    let t: (u8, String, f64) = (21, "Vikram".to_string(), 55_000.00);
    println!("t: {:?}", t);

    let (a, b, c) = t;
    println!("age: {}", a);
    println!("name: {}", b);
}
