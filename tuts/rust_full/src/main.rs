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
    if (age >= 1) && (age <= 18) {
        println!("Imp. birthdays");
    } else if (age == 21) && (age == 50) {
        println!("Imp. birthday");
    } else {
        println!("Non imp. bday");
    }
}
