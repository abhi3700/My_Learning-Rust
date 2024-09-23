/*
    filter out negative even numbers from a list starting from -100.
*/

pub fn main() {
    // FIXME: can we add 2 filters here positive & even. Currently, only even no.s are filtered out.
    let v: Vec<i32> = (-100..).filter(|x| x % 2 == 0).take(5).collect();
    println!("{:?}", v);
}
