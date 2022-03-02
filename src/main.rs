/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/print/fmt_1.rs"]
mod fmt_1;


fn main() {
    fmt_1::run();
}