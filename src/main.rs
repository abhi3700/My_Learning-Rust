/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../pro/two_sum/two_sum.rs"]
mod two_sum;

fn main() {
    // Creating a vector of numbers.
    let nums = vec![1, 5, 8, 10];
    let target = 13;
    let res = two_sum::run(nums, target);
    println!("{:?}", res);
}
