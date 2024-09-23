/* 
    Similar to `unwrap()`, but can set a custom message for None in Option
    - Apply for Option
    - 'n' denotes for 'None'

    In simple words, `expect` or `expect_ok`/`expect_some` throws error if the value is not Some or Ok
*/

pub fn run() {
    let n: Option<&str> = None;

    n.expect("abhijit expected");
}