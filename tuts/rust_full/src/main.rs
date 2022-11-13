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
    println!("Enter your age:");
    let mut age: String = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Please enter a valid age");

    let age: u8 = age.trim().parse().expect("Please enter a valid age");
    let is_adult = if (age > 18) { true } else { false };
    println!("Is adult: {}", is_adult);
}
