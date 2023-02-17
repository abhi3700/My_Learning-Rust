/*
    concat strings using push_str for String type var
*/

pub fn main() {
    let mut input = "abhijit".to_string();
    input.push_str(" roy is a ");
    for i in 0..6 {
        input.push_str("good");
    }
    input.push_str(" boy");
    println!("{}", input);
}
