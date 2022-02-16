/* 
    Learning:
    - How to dereference a function
    - Implicit dereferencing and borrowing

    Notice that, even though we’re working with a reference, we didn’t have to use the *-operator to access the first_name property of r, which is actually a reference. What we’re experiencing here is another usability feature of the Rust compiler. It turns out the .-operator performs the dereferencing implicitly, if needed!

    The same code can be de-sugared to 'Case-2'

    SOURCE: https://blog.thoughtram.io/references-in-rust/
*/


struct Person {
    first_name: String,
    last_name: String,
    age: u8
}


pub fn run() {
    let p = Person {
        first_name: "Abhijit".to_string(),
        last_name: "Roy".to_string(),
        age: 28
    };

    let r = &p;
    // Case-1
    // println!("{}", r.first_name);

    // Case-2
    println!("{}", (*r).first_name);
}