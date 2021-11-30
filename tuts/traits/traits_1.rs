/*
    Implement Dog for Animal & Speaks traits types.
*/

trait Speaks {
    fn speak(&self);

    fn noise(&self) -> &str;
}

trait Animal {
    fn animal_type(&self) -> &str;
}

struct Dog {}

impl Animal for Dog {
    fn animal_type(&self) -> &str {
        "dog"
    }
}

impl Speaks for Dog {
    fn speak(&self) {
        println!("The dog speaks {}", self.noise());
    }

    fn noise(&self) -> &str {
        "woof"
    }
}


fn main() {
    let dog = Dog {};
    dog.speak();
}