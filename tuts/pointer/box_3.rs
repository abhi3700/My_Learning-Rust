//! Here, using a Box smart pointer, we create a vector with different
//! items where each of the item implements a shared trait - `Speak`
//!
//! Run a for-loop in order to print the `speak()` as output.

trait Speak {
    fn speak(&self);
}

pub struct Dog;
impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

pub struct Cat;
impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

pub fn main() {
    let my_animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in my_animals {
        animal.speak();
    }
}
