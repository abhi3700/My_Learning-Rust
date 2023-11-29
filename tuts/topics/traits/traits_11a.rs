// M-1

// declare inside trait
trait Speak {
    fn say_hello(&self) -> String;
}

// define structs implement traits after defining accordingly
struct Person1 {}
impl Speak for Person1 {
    fn say_hello(&self) -> String {
        "Happy New Year 2023!".to_string()
    }
}

struct Person2 {}
impl Speak for Person2 {
    fn say_hello(&self) -> String {
        "Happy New Year 2023!".to_string()
    }
}

// main
pub fn main() {
    let p1 = Person1 {};
    println!("Person1: {}", p1.say_hello());
    let p2 = Person2 {};
    println!("Person2: {}", p2.say_hello());
}
