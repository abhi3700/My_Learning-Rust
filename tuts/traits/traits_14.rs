//! Shows static & dynamic dispatching of traits

trait Speak {
    fn speak(&self);
}

struct Dog {}

impl Speak for Dog {
    fn speak(&self) {
        println!("Dog says: Woof!");
    }
}

struct Cat {}

impl Speak for Cat {
    fn speak(&self) {
        println!("Cat says: Meow!");
    }
}

/// M-1: This is a static dispatch
fn make_speak<T>(animal: &T)
where
    T: Speak,
{
    animal.speak();
}

/// M-2: This is a static dispatch
fn make_speak_2(animal: &impl Speak) {
    animal.speak();
}

/// This is a dynamic dispatch which is using borrowing
fn make_speak_dyn(animal: &dyn Speak) {
    animal.speak();
}

/// This is a dynamic dispatch which is taking ownership of the object
fn make_speak_dyn_ownership(animal: Box<dyn Speak>) {
    animal.speak();
}

fn main() {
    let dog = Dog {};
    let cat = Cat {};

    make_speak(&dog);
    make_speak(&cat);

    make_speak_2(&dog);
    make_speak_2(&cat);

    make_speak_dyn(&dog);
    make_speak_dyn(&cat);

    make_speak_dyn_ownership(Box::new(dog));
    make_speak_dyn_ownership(Box::new(cat));
}
