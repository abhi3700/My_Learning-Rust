

fn f<'a>(s: &'a str, k: &'a str) -> &'a str {
    if s.len() > 0 {
        s
    } else {
        k
    }
}

pub fn main() {
    println!("{}", f("abhijit", "gyanesh"));
}
