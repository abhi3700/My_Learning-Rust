/* 
    Example for Option:
    
    In this eg:
    - if the given filename is found in the system, then get the word count.
    - `match` pattern is used to printout the result when given file is found or not
    - output for file not found shall have error which needs to be printed. Hence, the Result type is to be used instead of Some

    OBSERVATION:
    - `r.get_key_value(s).unwrap().1` -> 
        + is to find the value for given key.
        + is an iterator, so needs to dereferenced to get `u32` type, otherwise, `&u32` type
*/

use std::collections::HashMap;

// return whether a file is found or not in the given dir
fn is_file_found(file_name: &str, dir_path: &str) -> bool {
    let paths = std::fs::read_dir(&dir_path).unwrap();

    let mut found = false;

    for path in paths {
        if path.unwrap().path().file_name().unwrap() == file_name {
            found = true
        }
    }
    found

}

// get the filename count if it's found in the given dir
fn get_filename_count_from_file(file_name: &str, dir_path: &str) -> Result<u32, String> {
    // if the file is found, then count the string
    if is_file_found(&file_name, &dir_path) {
        Ok(file_name.chars().count().try_into().unwrap())
    } 
    // else if the file is not found
    else {
        Err("file is not found".to_string())
    }
}

pub fn run() {
    let fname = "rand.rs";
    let dir = "./tuts/error_handling/";

    match get_filename_count_from_file(&fname, &dir) {
        Ok(x) => println!("Word count in filename: {}", x),
        Err(y) => println!("Error: {}", y)
    }
}