//! A. use traits to obtain polymorphism

// Define a struct called Dog.
struct Dog;

// Define a struct called Bird.
struct Bird;

// Define a trait called Walk, which has a method called walk.
trait Walk {
    fn walk(&self);
}

// Implement the Walk trait for the Dog struct.
impl Walk for Dog {
    fn walk(&self) {
        println!("Dog is walking");
    }
}

// Implement the Walk trait for the Bird struct.
impl Walk for Bird {
    fn walk(&self) {
        println!("Bird is walking");
    }
}

// Define a function called activity_of that takes a reference to an object that implements the Walk trait as an argument.
// Using Method 1:
// Using Trait Bound Syntax with `impl Trait` i.e. Static Dispatch:
// fn activity_of(item: &impl Walk) {
//     // Call the walk method on the input item.
//     item.walk();
// }

// Using Method 2:
// Using Trait Bound Syntax with `dyn Trait` i.e. Dynamic Dispatch:
// fn activity_of(item: &dyn Walk) {
//     // Call the walk method on the input item.
//     item.walk();
// }

// Using Method 3.1:
// Using Generics:
// This is another way of defining a function with trait bound syntax. It works like Method 1, but uses Generic Type Syntax rather than dynamic dispatch.
// fn activity_of<T: Walk>(item: &T) {
//     // Call the walk method on the input item.
//     item.walk();
// }

// Using Method 3.2:
// This is similar to Method 3.1, but use `where` clause to bind the generic type parameters.
fn activity_of<T>(item: &T)
where
    T: Walk,
{
    // Call the walk method on the input item.
    item.walk();
}

// Define the entry point of the program.
pub fn main() {
    // Create an instance of the Dog struct.
    let dog = Dog;
    // Call activity_of with a reference to the dog instance.
    activity_of(&dog);

    // Create an instance of the Bird struct.
    let bird = Bird;
    // Call activity_of with a reference to the bird instance.
    activity_of(&bird);
}
