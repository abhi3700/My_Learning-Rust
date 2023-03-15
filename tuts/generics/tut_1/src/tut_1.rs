/*
   The problem statement is to find the largest element in a list
   First we try to fit in the same function for 2 types: i32, char by writing it twice.

   Cons: But, this is not the productive way of handling multiple data types with same function logic.
*/

fn find_largest_i32(list: &[i32]) -> i32 {
    let mut max = list[0];

    for &i in list {
        if i > max {
            max = i;
        }
    }

    max
}

fn find_largest_char(list: &[char]) -> char {
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
    let x_largest = find_largest_i32(&x);
    println!("largest i32: {x_largest}");

    let x = ['a', 'j', 'b', 'x', 'i', 'z'];
    let x_largest = find_largest_char(&x);
    println!("largest char: {x_largest}");
}
