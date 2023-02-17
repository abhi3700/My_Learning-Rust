/*
    concat strings using `format!`
*/

pub fn main() {
    let name = "abhijit roy".to_string();

    let s = format!("{name} is a {} boy", "good".repeat(5));
    println!("{}", s);
}
