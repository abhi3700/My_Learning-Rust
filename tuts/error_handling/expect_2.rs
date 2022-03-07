/* 
    Similar to `unwrap()`, but can set a custom message for Err in Result
    - App
*/

pub fn run() {
    let e: Result<i8, &str> = Err("message");

    e.expect("Some other message");
}