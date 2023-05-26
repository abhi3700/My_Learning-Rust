//! Associated types

trait MyTrait {
    type Item;
    fn get(&self) -> Self::Item;
}

struct Numbers {}

impl MyTrait for Numbers {
    type Item = i32;
    fn get(&self) -> Self::Item {
        10
    }
}

impl MyTrait for Numbers {
    type Item = String;
    fn get(&self) -> Self::Item {
        10
    }
}
struct Article {}

impl MyTrait for Article {
    type Item = String;
    fn get(&self) -> Self::Item {
        "Hello".to_string()
    }
}

pub(crate) fn main() {
    let n = Numbers {};
    println!("{}", n.get());
}
