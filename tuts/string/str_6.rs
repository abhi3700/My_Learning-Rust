/*
    Use of raw string like single, double quotes inside a string.
    E.g. define JSON object in a string
*/

pub fn run() {
    // let x = "\"abhijit\" is a good boy";    // M-1
    let x = r#""abhijit" is a good boy"#; // M-2
    println!("{}", x);
}
