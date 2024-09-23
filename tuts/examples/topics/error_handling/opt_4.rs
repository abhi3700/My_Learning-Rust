/* 
    Example for Option:
    
    In this eg:
    - if the given name is found in the record, then give the id.
    - The record is stored in HashMap type.
    - `match` pattern is used to printout the output when given string is found or not

    OBSERVATION:
    - `r.get_key_value(s).unwrap().1` -> 
        + is to find the value for given key.
        + is an iterator, so needs to dereferenced to get `u32` type, otherwise, `&u32` type
    - `user_id` for username not found can be of `None` type
*/

use std::collections::HashMap;

fn get_id_by_name(s: &str, r: &HashMap<&str, u32>) -> Option<u32> {

    if r.contains_key(&s) {
        Some(*r.get_key_value(s).unwrap().1)
    } else {
        None
    }
}

pub fn run() {
    let mut record: HashMap<&str, u32> = HashMap::new();

    record.insert("abhijit", 532);
    record.insert("suresh", 232);
    record.insert("jignesh", 890);

    let input = "suresh";

    // println!("{:?}", record.get_key_value(&input).unwrap().1);

    match get_id_by_name(&input, &record) {
        None => println!("User not found"),
        Some(i) => println!("User id found: {}", i)
    }


}