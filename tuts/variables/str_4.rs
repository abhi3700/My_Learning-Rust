/* 
    String = Growable, heap-allocated data structure - Use when you need to modify or own string data

    Here, it is defined as Growable string. So, its type is `String`
    - Hence, methods like `push`, `push_str` are usable for this type, but not `&str`.
*/

fn main() {
    // convert the default `&str` type to `String` & then methods like `push`, `push_str` can be used.
    let mut  hello = String::from("Hello ");
    println!("{}", hello.len());
    hello.push('W');
    hello.push_str("orld");

    println!("{}", hello);

    // in bytes
    println!("{}", hello.capacity());

    // Check if empty
    println!("{}", hello.is_empty());

    // Contains
    println!("{}", hello.contains("Hello"));

    // Replace
    println!("{}", hello.replace("World", "there"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create a string with capacity
    let mut s = String::with_capacity(10);      // in bytes
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    // Nothing happens if pass, but shows error message if fails.
    assert_eq!(3, s.len());
}