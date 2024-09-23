/*
    using `if let`
*/

pub fn main() {
    let target = "rustlings";
    let optional_target = Some(target);

    // TODO: Make this an if let statement whose value is "Some" type
    if let word = optional_target {
        assert_eq!(word.unwrap(), target);
    }
}
