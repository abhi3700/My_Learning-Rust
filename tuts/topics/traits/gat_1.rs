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

/// ❌ Can't implement the same trait for different types. For this follow next example - ./gat_2.rs
impl MyTrait for Numbers {
    type Item = String;
    fn get(&self) -> Self::Item {
        10
    }
}

pub(crate) fn main() {
    let n = Numbers {};
    println!("{}", n.get());
}
