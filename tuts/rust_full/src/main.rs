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
    println!("Max u32: {}", std::u32::MAX);
    println!("Max u64: {}", std::u64::MAX);
    println!("Max u128: {}", std::u128::MAX);
    println!("Max usize: {}", std::usize::MAX);
    println!("Max f32: {}", std::f32::MAX);

    let is_true: bool = true;
    println!("{}", is_true);

    let my_grade = 'A';

    let num_1: f32 = 1.111111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111111);
    let num_2: f64 = 1.111111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111111);

    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("{} + {} = {}", num_3, num_4, num_3 + num_4);
    println!("{} - {} = {}", num_3, num_4, num_3 - num_4);
    println!("{} * {} = {}", num_3, num_4, num_3 * num_4);
    println!("{} / {} = {}", num_3, num_4, num_3 / num_4);
}
