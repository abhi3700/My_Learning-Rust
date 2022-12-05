/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/type_of/type_of1.rs"]
mod type_of1;

fn main() {
    type_of1::run();
}
