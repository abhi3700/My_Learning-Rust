//! Generic Associated types
//!
//! TODO: Still unstable: https://github.com/rust-lang/rust/issues/29661
//! Solution: As of now, we can use generic trait as shown in "/tuts/generics/g_trait_struct.rs"

trait MyTrait<T> {
    type Item = T;
    fn get(&self) -> Self::Item;
}

// struct Numbers {}

// impl MyTrait for Numbers {
//     type Item = i32;
//     fn get(&self) -> Self::Item<i32> {
//         10
//     }
// }

// impl MyTrait for Numbers {
//     type Item = String;
//     fn get(&self) -> Self::Item<String> {
//         "Hello".to_string()
//     }
// }

// pub(crate) fn main() {
//     let n = Numbers {};
//     println!("{}", <Numbers as MyTrait<i32>>::get(&n));
// }
