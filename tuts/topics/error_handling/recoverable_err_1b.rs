/*
    Continuing from the previous example, if the file is not found then we can just
    create the file
        - Ok => print file info
        - Err => panic with the error msg
*/
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(s) => println!("File found, {:?}", s),
        Err(error) => {
            println!("error: {}", error);
            let f = File::create("hello.txt");
            match f {
                Ok(s) => println!("file created at {:?}", s),
                Err(e) => panic!("file could not be created because of: {}", e),
            };
        }
    }
}
