/*
    collect all the words from a sentence into a vector of &str.
*/

pub fn main() {
    let s1 = "Kolkata has the best food";

    let list: Vec<&str> = s1.split(" ").collect();
    println!("{:?}", list);
}
