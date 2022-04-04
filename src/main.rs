/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/collection/tuple1.rs"]
mod tuple1;


fn main() {
    tuple1::run();
}