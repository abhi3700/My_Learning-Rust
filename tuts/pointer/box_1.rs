/* 
    Box: A pointer type for heap allocation
    - Boxes provide ownership for this allocation, and drop their contents when they go out of scope. 
    - Boxes also ensure that they never allocate more than isize::MAX bytes.

    SOURCE: https://docs.rs/sp-std/latest/sp_std/boxed/index.html
*/

pub fn run() {
    let val: u8 = 5;
    let boxed: Box<u8> = Box::new(val);

    println!("{:?}", *boxed);       // 5

    let boxed2: Box<u8> = Box::new(78);
    let val2 = *boxed2;

    println!("{:?}", val2);       // 78

}