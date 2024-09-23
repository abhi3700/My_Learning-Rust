//! # Lifetimes
//! Uses into a function to ensure the function lives as long as its arguments

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longer(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result); // ✅
    }
    println!("The longest string is {}", result); // ❌
}
