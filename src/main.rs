/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/crypto/hash_1.rs"]
mod hash_1;


fn main() {
    hash_1::run();
}