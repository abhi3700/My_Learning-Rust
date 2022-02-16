/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/iterator/iterator_3.rs"]
mod iterator_3;


fn main() {
    iterator_3::run();
}