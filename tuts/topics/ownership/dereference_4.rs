/* 
    Learning:
    - How to dereference a function
    - Implicit dereferencing and borrowing

    Notice that, even though we’re working with a reference, we didn’t have to use the *-operator to access the first_name property of r, which is actually a reference. What we’re experiencing here is another usability feature of the Rust compiler. It turns out the .-operator performs the dereferencing implicitly, if needed!

    The same code can be de-sugared to 'Case-2' & 'Case-3'

    SOURCE: https://blog.thoughtram.io/references-in-rust/
*/


pub fn run() {
    let mut numbers = [3, 1, 2];
    let r = &mut numbers;

    // Case-1
    // r.sort();

    // Case-2
    // (*r).sort();

    // println!("{:?}", r);
    // Case-3
    (&mut numbers).sort();
    println!("{:?}", numbers);
}