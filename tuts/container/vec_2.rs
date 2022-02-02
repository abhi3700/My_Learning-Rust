/* 
    iterate via mutating
*/

fn main() {
    let mut x = vec![3, 5, 7, 34, 78, 90];
    println!("{:?}", x);

    // here *i denotes the value pointer by the iterator. 
    // So, i is actually an iterator iterating across the vector's elements.
    // Hence, *i would mean value at the pointer like in C++
    for i in x.iter_mut() {
        *i *= 2;
    }

    println!("{:?}", x);
}