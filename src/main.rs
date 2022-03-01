/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/pointer/box_1.rs"]
mod box_1;


fn main() {
    box_1::run();
}