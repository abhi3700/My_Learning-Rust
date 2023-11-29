use std::fs::File;

use std::fs::File;

pub fn main() {
    let _f = File::open("hello.txt").map_err(|error| println!("{}", error));
}
