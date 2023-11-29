// M-2

// define inside trait
trait Speak {
    fn say_hello(&self) -> String {
        format!("Happy New Year 2023!")
    }
}

// define structs implement traits accordingly
struct Person1 {}
impl Speak for Person1 {}

struct Person2 {}
impl Speak for Person2 {}

// main
pub fn main() {
    let p1 = Person1 {};
    println!("Person1: {}", p1.say_hello());
    let p2 = Person2 {};
    println!("Person2: {}", p2.say_hello());
}
