/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/error_handling/asserteq_2.rs"]
mod asserteq_2;


fn main() {
    asserteq_2::run();
}