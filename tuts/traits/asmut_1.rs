/* 
    AsMut
    - like AsRef
    - take input as mutable by using `&mut Foo`
    - Required method: `as_mut()` does the conversion
*/

// M-1: w AsMut
fn add_one<T: AsMut<u64>>(num: &mut T) {
    *num.as_mut() += 1;
}

// M-2: w/o AsMut
// fn add_one(num: &mut u64) {
//     *num += 1
// }

pub fn run() {
    let mut boxed_num = Box::new(56);
    add_one(&mut boxed_num);
    println!("{}", boxed_num);
}