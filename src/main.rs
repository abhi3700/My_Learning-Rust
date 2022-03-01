/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../libs/time/time_2.rs"]
mod time_2;


fn main() {
    time_2::run();
}