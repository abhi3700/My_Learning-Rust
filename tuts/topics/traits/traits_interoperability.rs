//! This shows the feature of interoperability of Rust traits.

trait Speak {
    fn speak(&self);
}

struct Human {
    name: String,
}

impl Speak for Human {
    fn speak(&self) {
        println!("Hello, my name is {}!", self.name);
    }
}

pub fn main() {
    let person = Human {
        name: String::from("Bob"),
    };
    person.speak();
}
