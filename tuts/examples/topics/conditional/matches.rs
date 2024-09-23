//! show `matches!` usage

enum Color {
    Red,
    Blue,
    Green,
}

pub(crate) fn main() {
    let my_color = Color::Red;
    if matches!(my_color, Color::Red) {
        println!("It's red. 🎉")
    } else {
        println!("Not red ☹️");
    }
}
