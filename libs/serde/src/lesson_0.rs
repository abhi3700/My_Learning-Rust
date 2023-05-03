//! # Lesson 0: Basic Serialization
//! `serde` has the traits `Serialize` and `Deserialize` which are applied
//! to the data types (structs) during serialization and deserialization using
//! `serde_json` methods - `to_string` and `from_str` respectively.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Address {
    street: String,
    city: String,
    pincode: u32,
    country: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
    address: Address,
}

pub fn main() {
    let person = Person {
        name: String::from("John Doe"),
        age: 32,
        address: Address {
            street: String::from("123, Rust Street"),
            city: String::from("Rustville"),
            pincode: 12345,
            country: String::from("Rustland"),
        },
    };
    // print the original object
    println!("Person: {:?}", person);

    // serialize the person struct to a JSON string
    // Need to transfer the data over the network or store it in a DB
    let serialized_person = serde_json::to_string(&person).unwrap();
    println!("Serialized Person: {}", serialized_person);

    // deserialize the JSON string back to a person struct
    // Need to read the data from the network or DB
    let deserialized_person: Person = serde_json::from_str(&serialized_person).unwrap();
    println!("Deserialized Person: {:?}", deserialized_person);

    // the original and deserialized person structs should be equal
    assert_eq!(person, deserialized_person);
}
