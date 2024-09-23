/*
    try-catch is implemented like this in Rust.
    Simple Example!
*/

use std::fs::File;

fn main() {
    // the output is of type `Result<File, Error>`
    let f = File::open("hello.txt");
    match f {
        Ok(success) => println!("{:?}", success),
        Err(failure) => panic!("file is not found: {}", failure),
    };
}
