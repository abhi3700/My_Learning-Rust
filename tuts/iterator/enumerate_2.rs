/**
 * output_sorted.into_iter().enumerate() (to consume)
 */
use std::vec;

pub fn run() {
    let v1 = vec!["abhi", "sam", "gary"];

    for (pos, element) in v1.into_iter().enumerate() {
        println!("{pos}: {element}");
    }

    // println!("{:?}", v1); // Error: borrow of moved value: `v1`. Hence, not able to access the v1 after use via `.into_iter().enumerate()`
}
