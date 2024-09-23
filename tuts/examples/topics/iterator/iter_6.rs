/*
    `iter_mut()` allows the programmer to iterate each element
    as a mutable reference.
*/

pub fn run() {
    let mut v1 = vec![1, 5, 6, 8];

    // for i in v1.iter_mut() {
    //     *i += 1;
    // }

    // OR in just one line
    v1.iter_mut().for_each(|n| *n += 1);

    println!("{:?}", v1);
}
