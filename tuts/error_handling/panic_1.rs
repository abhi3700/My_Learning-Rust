/* 
  panic macro
*/

pub fn run() {
    let mut username = String::new();

    if username.is_empty() {
        panic!("user is empty");
    }

    println!("{}", username);
}