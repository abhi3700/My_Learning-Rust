/* 
    Get memory of array container
*/

fn main() {
    let arr1: [i32; 5] = [4, 6, 67, 23, 78];
    println!("Array occupies {} bytes", std::mem::size_of_val(&arr1));
}