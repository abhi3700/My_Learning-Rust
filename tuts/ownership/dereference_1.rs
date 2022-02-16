/* 
    Learning:
    - How to use  dereference a reference

    SOURCE: https://blog.thoughtram.io/references-in-rust/
*/

pub fn run() {
    let x = 10;
    let r = &x;

    if *r == 10 {
        println!("Same");
    }
}