/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/traits/ops_1.rs"]
mod ops_1;


fn main() {
    ops_1::run();
}