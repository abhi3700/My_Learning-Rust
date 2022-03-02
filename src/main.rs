/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/traits/asref_1.rs"]
mod asref_1;


fn main() {
    asref_1::run();
}