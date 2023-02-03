/*
    The .clone() method for compound types like struct, enum.

    Here, these traits are implemented:
    - Default: to initialize the name, age with default empty values - "", 0
    - Debug: printable
    - Clone: to access .to_clone() method.
*/

use std::clone;

#[derive(Default, Debug, Clone)]
struct MyObject {
    name: String,
    age: u8,
}

impl MyObject {
    fn set_name(&mut self, n: String) {
        self.name = n;
    }

    fn set_age(&mut self, a: u8) {
        self.age = a;
    }
}

pub fn main() {
    let my_object = &MyObject::default();
    let cloned_object = my_object.clone(); // method `clone` not found for this struct.
                                           // Hence, implement Clone trait for struct

    println!("{:?}", cloned_object);
}
