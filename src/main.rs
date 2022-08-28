/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/iterator/enumerate_3.rs"]
mod enumerate_3;

fn main() {
    enumerate_3::run();
}
