#[allow(unused)]
#[allow(unused_imports)]
use rand::Rng;
use std::io;
use std::io::Read;

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("What is your name?");
    let mut name = String::new();

    let greeting = "congrats";
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}! {}", name.trim(), greeting);
}
