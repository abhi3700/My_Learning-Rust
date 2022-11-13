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
    let arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st element: {}", arr1[0]);
    println!("length: {}", arr1.len());
    println!("last element: {:?}", arr1.last().expect("empty array"));

    // v1: loop using `for`
    for i in arr1 {
        println!("{}", i);
    }

    let mut loop_idx = 0;

    println!("\n===");
    // v2: loop using `loop`
    loop {
        if (arr1[loop_idx] % 2 == 0) {
            loop_idx += 1;
            continue;
        }

        if (arr1[loop_idx] == 9) {
            break;
        }
        println!("{}th element: {}", loop_idx, arr1[loop_idx]);
        loop_idx += 1;
    }

    // v3: loop using `while`
    println!("\n===");
    let mut loop_idx_2 = 0;
    while loop_idx_2 < arr1.len() {
        println!("{}", arr1[loop_idx_2]);
        loop_idx_2 += 1;
    }
}
