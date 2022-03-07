/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/iterator/iter_4.rs"]
mod iter_4;


fn main() {
    iter_4::run();
}