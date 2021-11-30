/*
    Implement Dog & Cat for Animal & Speaks traits types.

    Changes:
    - noise() shifted to Animal
    - implemented Speaks for Animal
    - Now, just change the Animal as dog, cat, etc. 
*/

trait Speaks {
    fn speak(&self);
}

trait Animal {
    fn animal_type(&self) -> &str;
    fn noise(&self) -> &str;
}

impl<T> Speaks for T where T: Animal {
    fn speak(&self) {
        println!("The {} speaks {}", self.animal_type(), self.noise());
    }
}

struct Dog {}
struct Cat {}

impl Animal for Dog {
    fn animal_type(&self) -> &str {
        "dog"
    }

    fn noise(&self) -> &str {
        "woof"
    }
}


impl Animal for Cat {
    fn animal_type(&self) -> &str {
        "cat"
    }

    fn noise(&self) -> &str {
        "meow"
    }
}

fn main() {
    let dog = Dog {};
    dog.speak();

    let cat = Cat {};
    cat.speak();
}