/* 

PartialOrd

Corollaries
===========
The following corollaries follow from the above requirements:

- irreflexivity of < and >: !(a < a), !(a > a)
- transitivity of >: if a > b and b > c then a > c
- duality of partial_cmp: partial_cmp(a, b) == partial_cmp(b, a).map(Ordering::reverse)

In this eg,
to find out the comparison first implement PartialOrd & then define the atribute for Ord.
And this is done by implementing `partial_cmp` by using `cmp` 

SOURCE: https://docs.rs/sp-std/latest/sp_std/cmp/trait.PartialOrd.html

*/

use std::cmp::{PartialOrd, Ordering};

#[derive(Eq)]
struct Person {
    id: u32,
    name: String,
    height: u32
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

pub fn run() {
    let p1 = Person {
        id: 1, 
        name: "Alice".to_string(),
        height: 5
    };

    let p2 = Person {
        id: 2, 
        name: "Bob".to_string(),
        height: 6
    };

    assert!(p1 < p2);
    assert!(p2 > p1);           // it has the duality for same type
    assert!(p1 != p2);          // as their height differs

}