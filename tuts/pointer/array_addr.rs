//! Here, each element is of 4 bytes i.e. 32 bit hence, each slot is 4 bytes apart.
//! So, the addresses difference shall be 4 bytes.

pub fn main() {
    let v = vec![1, 15, 3, 4, 5];
    println!("1st element: {}", &v[0]); // 1
    println!("2nd element: {}", &v[1]); // 15
    println!("The address of 1st element in the vector is: {:p}", &v[0]); // :p is for printing the address of the pointer      0x155606e30
    println!("The address of 2nd element in the vector is: {:p}", &v[1]); // :p is for printing the address of the pointer      0x155606e34
}
