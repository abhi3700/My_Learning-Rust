/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/ownership/dereference_4.rs"]
mod dereference_4;


fn main() {
    dereference_4::run();
}