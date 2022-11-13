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
    let num1: u8 = 32;
    let num2: u16 = 56;
    let num3: u32 = (num1 as u32) + (num2 as u32);

    println!("{}", num3);
}
