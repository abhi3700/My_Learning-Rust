/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/error_handling/expect_err_1.rs"]
mod expect_err_1;


fn main() {
    expect_err_1::run();
}