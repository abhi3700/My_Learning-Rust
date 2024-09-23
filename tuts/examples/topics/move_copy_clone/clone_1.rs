/*
    The .clone() method is available to all types deriving the Clone trait.
    By default, all the primitive types (str, i8,u8, char,array, bool, etc), have access to the .clone() method.

    1. Generating a duplicate with the same type (T -> T)
        [valid for all]
    2. Generating a duplicate from borrowed type (&T -> T)
        [valid for primary scalar, tuples, fixed array]

    3. Generating a duplicate from borrowed type (&T -> &T)
        [array without defined types, vec, struct] except {2} mentioned above
*/

pub fn main() {
    let s = "a"; // &str
    let cloned_str = s.clone(); // &str

    // ****************** //
    let number: u8 = 10; // type u8
    let cloned_number = number.clone(); // type u8

    let borrowed_number: &u8 = &10; // type &u8
    let cloned_borrowed_number = borrowed_number.clone(); // type u8

    // ****************** //
    let array: &[&str] = &["a", "b", "c"]; // type &[&str]
    let cloned_array = array.clone(); // type &[&str]

    let fixed_array: [&str; 3] = ["a", "b", "c"]; // type [&str; 3]
    let cloned_fixed_array = array.clone(); // type [&str; 3]

    let borrowed_fixed_array: &[&str; 3] = &["a", "b", "c"]; // type &[&str; 3]
    let cloned_borrowed_fixed_array = borrowed_array.clone(); // type [&str; 3]

    let string: String = String::from("Hello, world!"); // type String
    let cloned_string = string.to_owned(); // type String
}
