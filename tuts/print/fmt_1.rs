/* 
    format gives output as String

    In this eg, format the name with title - Mr./Mrs.
*/

pub fn run() {
    let mut person = "Abhijit Roy".to_string();
    person = format!("Mr. {person}");

    println!("{person}");
}