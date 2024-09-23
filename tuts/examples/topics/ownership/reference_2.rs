/*
    Learning mutable reference via `&mut`
*/

pub fn main() {
    let mut x = 3;
    let r = &mut x;
    *r = 5; // this assigns 5 to x using the reference
            // x = 5; // this assigns 5 to x w/o using the reference

    // Case-1: Throws error, can't be used both to get printed at the same time.
    // REASON: x is borrowed mutably & then borrowed immutably (implicitly done by println macro)
    // Also, you can't use a variable 'x' & its reference at the same time via `println!`.
    // println!("x: {}", x);
    // println!("r is at: {}", &r);

    // Case-2: SUCCESS
    println!("x: {}", x);
    println!("x is at: {:p}", &x); // get its address -> 0x16f8ce0bc

    // Case-3: SUCCESS
    // println!("r: {}", r);
    // println!("r is at: {:p}", &r); // get its address -> 0x16f58a0c0
}
