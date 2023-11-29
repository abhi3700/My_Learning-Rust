/* 
    Add clone trait for types that cannot be "implicitly copied"

    - String trait by default has `clone()` method.

*/

pub fn run() {
    let s = "hello".to_string();
    let s_copy = s.clone();


    println!("{}", s_copy);     // copy of s using clone()
    println!("{}", s);          // using even after copied => s var still has the ownership of the value: "hello"
}