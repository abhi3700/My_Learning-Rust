macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

macro_rules! add {
    ($x: expr, $y: expr) => {
        $x + $y
    };
}

pub fn main() {
    assert_eq!(25, five_times!(2 + 3));
    println!("{}", five_times!(4));
    println!("{}", add!(4, 5));
}
