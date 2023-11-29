/*
    Usage of ? question mark operator

    - (?) can only be used inside functions returning either Result<_, _> or Option<_>
    - can't be used on both Result<_, _> & Option<_> present in a function returning either type

*/

fn main() {
    let s1 = String::from("Kolkata has the best street food!");
    let idx = find_char_index_in_string(&s1, &'z');
    println!("{}", idx.unwrap_or(404));
}

// M-1
// fn find_char_index_in_string(s1: &String, char: &char) -> Option<usize> {
//     let index = s1.find(|x| &x == char)?;

//     Some(index)
// }

// M-2: shortest code
fn find_char_index_in_string(s1: &String, char: &char) -> Option<usize> {
    Some(s1.find(|x| &x == char)?)
}
