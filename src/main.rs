/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/traits/borrow_1.rs"]
mod borrow_1;


fn main() {
    borrow_1::run();
}