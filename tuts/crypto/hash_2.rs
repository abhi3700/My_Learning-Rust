/* 
    For more control over how the hashing happens, use impl for type with modifying function hash()
*/

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Person {
    id: u32,
    name: String,
    phone: u64
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.phone.hash(state);
    }
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

    println!("{:?}", calculate_hash(&p1));          // 1421399677539868452
    println!("{:?}", calculate_hash(&p2));          // 1248121116834261347
    assert!(calculate_hash(&p1) != calculate_hash(&p2));

}

fn calculate_hash<T: Hash>(t: T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}