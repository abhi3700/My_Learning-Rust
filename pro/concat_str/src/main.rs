/*
    It's better to use the format! operator to join strings than to use the + operator or push, push_str method.

    Using format! is usually the most succinct and readable way to combine strings.
*/
fn main() {
    let s1 = "Hello";
    println!("{}", format!("{} World!", s1));
}
