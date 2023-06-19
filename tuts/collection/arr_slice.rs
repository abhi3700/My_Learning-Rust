//! Copy the string as bytes into a defined array of zeros (initialized).
//!
//! Basically, you need to copy the "cdhsvbfdjvbfd" into "00000000000000000000000000000"
//! --> "cdhsvbfdjvbfd0000000000000000"
//!
//! Source: from Borsh demo folder
//! "libs/borsh/demo/src/tut_2.rs"

pub fn main() {
    // give a description & add zeros to fill the array
    let block_data_str = "set_value(u8)";
    let mut block_data_bytes = [0u8; 32]; // Initialize an array of zeros.
    block_data_bytes[..block_data_str.len()].copy_from_slice(block_data_str.as_bytes());
    // Copy the bytes of the string into the array.

    println!("{:?}", block_data_bytes);
}
