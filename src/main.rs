/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/ownership/lifetime_3.rs"]
mod lifetime_3;


fn main() {
    lifetime_3::run();
}