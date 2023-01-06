/*
    function defined with lifetime
*/

//! throws error: missing lifetime specifier
//! this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `s` or `k`
// fn f(s: &str, k: &str) -> &str {
//     if s.len() > 0 {
//         s
//     } else {
//         k
//     }
// }

//! fixed via adding single lifetime 'a
fn f<'a>(s: &'a str, k: &'a str) -> &'a str {
    if s.len() > 5 {
        s
    } else {
        k
    }
}

pub fn main() {
    println!("{}", f("abhijit", "gyanesh"));
}
