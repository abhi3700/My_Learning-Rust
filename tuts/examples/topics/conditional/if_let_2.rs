//! Shows `if-let` usage with custom enum variant.
//!
//! Objective: Check with "./if_let_1.rs" & "./matches.rs"

enum Color {
    RGB(u8, u8, u8),
    Named(String),
}

pub(crate) fn main() {
    let my_color = Color::RGB(255, 0, 0);

    if let Color::RGB(r, g, b) = my_color {
        println!("It's red. 🎉 with color code: ({r}, {g}, {b})",)
    } else {
        println!("Not an RGB color");
    }
}
