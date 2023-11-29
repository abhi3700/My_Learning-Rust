/**
 * output_sorted.iter_mut().enumerate() (to modify)
 */
use std::vec;

pub fn run() {
    let mut v1 = vec!["abhi", "sam", "gary"];

    for (pos, element) in v1.iter_mut().enumerate() {
        println!("{pos}: {element}");
    }

    println!("Before modify: {:?}", v1); // Error: borrow of moved value: `v1`. Hence, not able to access the v1 after use via `.into_iter().enumerate()`
    v1.push("alice");
    println!("After modify: {:?}", v1);
}
