/*
    find index of character 1st found in a string

*/

pub fn main() {
    let s1 = String::from("Lifeline provides most discount");
    let first_idx = s1.find(|x| x == 'i'); // first index of the character

    println!("{}", first_idx.unwrap_or(""));
}
