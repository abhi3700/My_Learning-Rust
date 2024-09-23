//! trait bounds

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
fn greeting<T>(p: T)
where
    T: Speak,
{
    println!("{}", p.say_hello());
}

struct Person {}

impl Speak for Person {}

// main
pub fn main() {
    let p = Person {};
    greeting(p);
}
