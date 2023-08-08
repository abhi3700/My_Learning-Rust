//! Implement multiple traits with same functions for a struct type

struct Person;

trait Shop1 {
    fn buy();
}

trait Shop2 {
    fn buy();
}

impl Shop1 for Person {
    fn buy() {
        println!("Purchased from Shop1");
    }
}
impl Shop2 for Person {
    fn buy() {
        println!("Purchased from Shop2");
    }
}

pub(crate) fn main() {
    <Person as Shop1>::buy();
    <Person as Shop2>::buy();
}
