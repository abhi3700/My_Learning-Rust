/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/string/str_6.rs"]
mod str_6;

fn main() {
    str_6::run();
}
