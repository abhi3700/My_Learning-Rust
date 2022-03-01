/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/traits/cmp_4.rs"]
mod cmp_4;


fn main() {
    cmp_4::run();
}