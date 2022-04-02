/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/ownership/move_semantics5.rs"]
mod move_semantics5;


fn main() {
    move_semantics5::run();
}