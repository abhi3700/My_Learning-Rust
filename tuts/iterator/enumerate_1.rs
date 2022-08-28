/**
 * Use output_sorted.iter().enumerate() (to consult)
 */
use std::vec;

pub fn run() {
    let v1 = vec!["abhi", "sam", "gary"];

    for (pos, element) in v1.iter().enumerate() {
        println!("{pos}: {element}");
    }

    println!("{:?}", v1); // able to access the v1 even after use via `.iter().enumerate()`
}
