/*
    type_of is a function that takes a generic type T and returns a string slice
*/

//! define type_of function
fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn run() {
    const ID: i32 = 156;
    println!("{}", type_of(ID));

    let age = 18;
    println!("{}", type_of(age));
    println!("{}", type_of(&age));

    let is_active = true;
    println!("{}", type_of(is_active));
    println!("{}", type_of(&is_active));

    let name = "John";
    println!("{}", type_of(name));

    let name1 = String::from("John");
    // println!("{}", type_of(name1));  // --> alloc::string::String
    println!("{}", type_of(&name1)); // --> &alloc::string::String

    let names = vec!["John", "Doe"];
    println!("{}", type_of(names));
}
