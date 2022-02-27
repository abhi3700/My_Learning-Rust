/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/arg/arg_1.rs"]
mod arg_1;


fn main() {
    arg_1::run();
}