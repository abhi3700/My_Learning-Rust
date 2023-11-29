/* 
    Primitive str = immutable fixed-length string somewhere in memory
    - By default string is automatically defined as `&str`

*/

fn main() {
    let x = "hello";

    println!("{:?}", (x, x.len()) );
}