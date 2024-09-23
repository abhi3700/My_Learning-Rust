/*
    Using error msg to be shown in case of any error thrown
*/

use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("File not found");
}
