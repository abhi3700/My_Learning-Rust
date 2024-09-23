//! Vector to Array conversion.
//! Usage: Normally it is used in case of decoding of decoded bytes into string.
//!
//! In the below example, `slice` is parsed into a string via `std::str::from_utf8(slice)`.
//! Thereby converting the slice into a string.

pub(crate) fn main() {
    // This array can be achieved via encoding a string into UTF-8 via Base64 crate.
    let vec: Vec<u8> = vec![72, 101, 108, 108, 111]; // "Hello" in UTF-8
    let slice: &[u8] = &vec;

    println!("{:?}", slice); // [72, 101, 108, 108, 111]
}
