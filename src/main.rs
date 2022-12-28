/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/generics/generics_1.rs"]
mod generics_1;

fn main() {
    generics_1::run();
}
