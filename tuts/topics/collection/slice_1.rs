/* 
    Get a portion of array/vec/tuple, etc.
*/

fn main() {
    let a = vec!(1, 4, 5, 7, 8, 34, 68);
    let slice = &a[1..5];           // index: 1 to 4

    println!("{:?}", slice);
}