/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/loop/for_1.rs"]
mod for_1;

fn main() {
    for_1::run();
}
