/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/error_handling/res_4.rs"]
mod res_4;


fn main() {
    res_4::run();
}