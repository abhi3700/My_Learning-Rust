/* 
    Option can have either Some or None as output
    It has also built_in methods like `is_some` & `is_none`

    An optional value can have either Some value or no value/ None.
    The Option type is a way to use Rust’s type system to express the possibility of absence.
    
    In this eg,
    - `Option<String>` is used as function's o/p, instead of `String` because the output could be None as well.
    - reference is being used because in the 1st println!, a is used twice.
    - In the 2nd println!, `unwrap` is used.
*/

fn get_name(s: &String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

pub fn run() {
    let a = "Abhijit Roy".to_string();
    println!("{:?}", get_name(&a));

    // unwrap the string from Option type i.e. `Some`
    println!("{}", get_name(&a).unwrap());

}