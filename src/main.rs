/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/loop/for_2.rs"]
mod for_2;

fn main() {
    for_2::run();
}
