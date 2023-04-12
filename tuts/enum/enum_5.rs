// Fix the errors
#[derive(Debug)]
enum Number {
    Zero,
    One,
    Two,
}

#[derive(Debug, PartialEq)]
enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
#[derive(Debug, PartialEq)]
enum Number2 {
    Zero = 0.0 as isize,
    One = 1.0 as isize,
    Two = 2.0 as isize,
}

pub fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8, Number1::One as u8); // DONE:
    assert_eq!(Number1::One as u8, Number2::One as u8); // DONE:

    println!("Success!");
}
