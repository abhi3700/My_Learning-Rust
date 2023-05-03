/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "./lesson_2.rs"]
mod lesson_2;

fn main() {
    lesson_2::main();
}
