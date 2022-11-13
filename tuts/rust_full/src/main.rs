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
    let mut s = String::new();
    s.push('A');
    s.push_str("Abhijit");
    s.push_str(" Abhishek");

    // print all the words separate by whitespace " "
    for i in s.split_whitespace() {
        println!("{}", i);
    }

    println!("\n==");
    println!("whole string: {}", s);

    let s2 = s.replace('A', "Another ");
    println!("new string: {}", s2);

    let mut v1: Vec<char> = s.chars().collect();
    println!("original vec: {:?}", v1);

    v1.sort();
    v1.dedup();
    println!("After sort & remove duplicate: {:?}", v1);

    let s3: &str = "Random string";
    let mut s4 = s3.to_string();
    let byte_arr1 = s4.as_bytes();
    println!("{:?}", byte_arr1);
    let s5 = &s4[0..3];
    println!("{:?}", s5);
    s4.clear();
    println!("s4 after clear: {}", s4);

    let s6 = String::from("Abhijit is ");
    let s7 = String::from("a good boy");

    // combine string
    let s8 = s6 + &s7;
    println!("{}", s8);

    for char in s8.bytes() {
        print!("{} ", char);
    }
}
