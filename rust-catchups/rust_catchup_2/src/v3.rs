//! polymorphism using traits which implements other traits.

// Define a trait called Walk, which has a method called walk.
trait Walk {
    fn walk(&self) {
        println!("Default walking");
    }
}

// Define another trait called Fly, which has a method called fly.
trait Fly {
    fn fly(&self) {
        println!("Default flying");
    }
}

// Define a trait called WalkFly which is the combination of both Walk and Fly, inheriting their methods.
trait WalkFly: Walk + Fly {}

// Define a struct called Bird2
struct Bird2;

// Implement the WalkFly trait for the Bird2 struct.
impl WalkFly for Bird2 {}

// Implement the Walk trait for the Bird2 struct, overwriting the default method to print "Bird2 is walking".
impl Walk for Bird2 {
    fn walk(&self) {
        println!("Bird2 is walking");
    }
}

// Implement the Fly trait for the Bird2 struct, overwriting the default method to print "Bird2 is flying".
impl Fly for Bird2 {
    fn fly(&self) {
        println!("Bird2 is flying");
    }
}

// Define a generic function that takes an item of type T and prints the result of calling its walk and fly methods, using the where clause to ensure that T implements the WalkFly trait.
fn activity_of<T>(item: &T)
where
    T: WalkFly,
{
    item.walk();
    item.fly();
}

// Define the entry point of the program
pub fn main() {
    // Create an instance of Bird2 struct
    let bird2 = Bird2;
    // Call the activity_of function passing in the bird2 instance as an argument
    activity_of(&bird2);
}
