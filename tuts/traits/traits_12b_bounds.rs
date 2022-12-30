//! trait bounds
//! Here, we want to add an additional parameter of 'name' in `greetings` function.

trait Speak {
    fn say_hello(&self) -> String {
        "Hello!".to_string()
    }
}

// M-1
// fn greeting(p: impl Speak) {
//     println!("{}", p.say_hello());
// }

// M-2
// fn greeting<T: Speak>(p: T) {
//     println!("{}", p.say_hello());
// }

// more production way of using trait bounds
// M-3
fn greeting<T>(p: T, person_name: String)
where
    T: Speak,
{
    println!("{} {}", p.say_hello(), person_name);
}

struct Person {}

impl Speak for Person {}

// main
pub fn main() {
    let p = Person {};
    greeting(p, "Abhi".to_string());
}
