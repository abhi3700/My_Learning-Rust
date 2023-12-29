//! Use match to its fullest i.e. using if as well
//!
//! NOTE: Please view the use of `if` inside one of the match arm.

pub(crate) fn main() {
    let x = Some(3);
    let rumor = true;
    match x {
        Some(1..=4) => println!("the x is b/w [1, 4]"),
        Some(6) if rumor => println!("conditional 6"),
        _ => println!("last value"),
    }
}
