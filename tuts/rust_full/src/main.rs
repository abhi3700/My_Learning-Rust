#[allow(unused_imports)]
use rand::Rng;
use std::io;
use std::io::Read;

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

#[allow(unused)]
fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.14;

    println!("Please enter your age: ");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");
    let mut age: u8 = age.trim().parse().expect("Not a number");
    age = age + 1;
    println!("I am {} and I want ${}", age, ONE_MIL);
}
