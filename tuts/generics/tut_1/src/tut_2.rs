/*
   create a generic function with all types covered.

   The problem we face is there is a missing of `PartialOrd` trait for each of the types,
   which has to be applied.
*/

// add 2 different traits: `PartialOrd` (for making a comparo with the max value i.e. list[0]) & `Copy` (for return type `T`)
fn find_largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &i in list {
        if i > max {
            max = i;
        }
    }
    max
}

pub fn main() {
    let x = [1, 56, 23, 14, 50];
    let x_largest = find_largest_generic(&x);
    println!("largest i32: {x_largest}");

    let x = ['a', 'j', 'b', 'x', 'i', 'z'];
    let x_largest = find_largest_generic(&x);
    println!("largest char: {x_largest}");
}
