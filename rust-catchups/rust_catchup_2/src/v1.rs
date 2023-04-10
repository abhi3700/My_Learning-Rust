//! A. use functions to obtain polymorphism

// Define a struct called Dog.
struct Dog;

// Define a struct called Bird.
struct Bird;

// Define a function called activity_dog that takes a Dog as an argument and prints a message to the console.
fn activity_dog(_item: Dog) {
    println!("Dog can walk, but can't fly");
}

// Define a function called activity_bird that takes a Bird as an argument and prints a message to the console.
fn activity_bird(_item: Bird) {
    println!("Bird can both fly and walk");
}

// Define the entry point of the program.
pub fn main() {
    // Create an instance of the Dog struct.
    let dog = Dog;
    // Call the activity_dog function with the dog instance.
    activity_dog(dog);

    // Create an instance of the Bird struct.
    let bird = Bird;
    // Call the activity_bird function with the bird instance.
    activity_bird(bird);
}
