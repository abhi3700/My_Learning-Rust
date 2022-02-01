/* 
    Primitive str = immutable fixed-length string somewhere in memory
    String = Growable, heap-allocated data structure - Use when you need to modify or own string data

    Here, it is defined as Growable string. So, its type is `String`
*/

fn main() {
    let mut  hello = String::from("Hello ");
    println!("{}", hello.len());
    hello.push('W');
    hello.push_str("orld");

    println!("{}", hello);
}