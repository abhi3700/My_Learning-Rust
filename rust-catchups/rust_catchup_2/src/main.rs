//! Polymorphism with traits
//! ========================
//! E.g. we have used the example
//!     of a cat (land), fish (water), crocodile (land + water) surviving on different ecosystem.

#[path = "./v1.rs"]
mod v1;

#[path = "./v2.rs"]
mod v2;

#[path = "./v3.rs"]
mod v3;

fn main() {
    println!("===v1===");
    v1::main();

    println!("===v2===");
    v2::main();

    println!("===v3===");
    v3::main();
}
