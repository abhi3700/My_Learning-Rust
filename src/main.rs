/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../pro/two_sum/two_sum.rs"]
mod two_sum;

fn main() {
    // Creating a vector of numbers.
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let res = two_sum::run(nums, target);
    println!("{:?}", res);
}
