//! Rust By Practice
//! ================
//! https://practice.rs/generics-traits/generics.html#struct-and-impl

// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct Val<T> {
    val: T,
}

// DONE: Added 'T' to the function return type
impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

pub fn main() {
    let x = Val { val: 3.0 };
    let y = Val {
        val: "hello".to_string(),
    };
    println!("{}, {}", x.value(), y.value());
}
