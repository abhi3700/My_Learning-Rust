//! trait bounds
//! Here, we want to add an additional parameter of 'name' in `greetings` function.

trait Speak {
    fn say_hello(&self) -> String {
        "Hello".to_string()
    }

    fn celebrate_bday(&self) -> String {
        "Happy B'day".to_string()
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
fn greeting<T>(p: T, age: i32, person_name: String)
where
    T: Speak,
{
    if age == 18 {
        println!("{} {}!", p.celebrate_bday(), person_name);
    } else {
        println!("{} {}!", p.say_hello(), person_name);
    }
}

struct Person {}

impl Speak for Person {}

// main
pub fn main() {
    let p = Person {};
    greeting(p, 19, "Alice".to_string());
}
