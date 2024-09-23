/*
    Continuing from the previous example, if the file is not found then we can just
    create the file
        - Ok => print file info
        - Err => panic with the error msg

    This is the without using match pattern, hence the entire code is very small/idiomatic.
*/

use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error| panic!("error creating the file: {}", error))
        } else {
            panic!("Error opening the file: {:?}", error);
        }
    });
}
