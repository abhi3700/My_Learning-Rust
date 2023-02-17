/*
    String to str
*/

pub fn main() {
    let s = String::from("Abhijit");
    find_if_word(&s); // String to &str
}

fn find_if_word(s: &str) {
    println!("{}", s);
}
