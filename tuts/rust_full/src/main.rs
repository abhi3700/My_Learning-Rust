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
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Guess the number!");
    let mut guess_num = String::new();
    io::stdin()
        .read_line(&mut guess_num)
        .expect("Failed to read line");

    let guess_num: u32 = guess_num.trim().parse().expect("Please type a number!");
    if guess_num == random_num {
        println!("You win!");
    } else {
        println!("You lose!");
        println!("The secret number is: {}", random_num);
    }
}
