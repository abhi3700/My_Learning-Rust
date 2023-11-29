use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    match read_file_to_string("example.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
