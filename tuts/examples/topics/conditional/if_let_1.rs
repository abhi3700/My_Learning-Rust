//! Shows `if-let` usage.
//!
//! The exact code is also achieved via `matches!` in `./matches.rs` file.

enum Color {
    Red,
    Blue,
    Green,
}

pub(crate) fn main() {
    let my_color = Color::Red;
    if let Color::Red = my_color {
        println!("It's red. 🎉")
    } else {
        println!("Not red ☹️");
    }
}
