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

//! error: lifetime may not live long enough
//! label: lifetime `'b` defined here
// fn f<'a, 'b>(s: &'a str, k: &'b str) -> &'a str {
//     if s.len() > 0 {
//         s
//     } else {
//         k
//     }
// }

//! fixed via defining 'b with type 'a
fn f<'a, 'b: 'a>(s: &'a str, k: &'b str) -> &'a str {
    if s.len() > 0 {
        s
    } else {
        k
    }
}

pub fn main() {
    println!("{}", f("abhijit", "gyanesh"));
}
