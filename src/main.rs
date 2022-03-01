/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/pointer/rc_1.rs"]
mod rc_1;


fn main() {
    rc_1::run();
}