/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/traits/asmut_1.rs"]
mod asmut_1;


fn main() {
    asmut_1::run();
}