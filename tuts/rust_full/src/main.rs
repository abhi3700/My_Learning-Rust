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

    //v1
    // Here, the simple match pattern is used
    let can_vote = match age {
        1..=17 => false,
        _ => true,
    };
    println!("Can vote: {}", can_vote);

    // v2
    // Here, the Ordering built-in is used
    let voting_age = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    }
}
