/* 
    For no control over how the hashing happens, just use Hash trait with derive over the type

    SOURCE: https://docs.rs/sp-std/latest/sp_std/hash/index.html
*/

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
    phone: u64
}

pub fn run() {
    let p1 = Person{
        id: 1,
        name: "Alice".to_string(),
        phone: 232_2132_434
    };

    let p2 = Person{
        id: 1,
        name: "Bob".to_string(),
        phone: 344_234_345
    };

    println!("{:?}", calculate_hash(&p1));      // 4283489829773872957
    println!("{:?}", calculate_hash(&p2));      // 14474125696081620951
    assert!(calculate_hash(&p1) != calculate_hash(&p2));

}

fn calculate_hash<T: Hash>(t: T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}