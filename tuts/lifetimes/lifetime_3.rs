/* 
    2 different lifetimes rather than 1 lifetime for same struct
*/


// Case-1
// INCORRECT ❌
// struct Point<'a> {
//     x: &'a u32,
//     y: &'a u32
// }


// Case-2
// CORRECT ✅
struct Point<'a, 'b> {
    x: &'a u32,
    y: &'b u32
}


pub fn run() {
    // Case-1
    // Throws error:
    // `y` does not live long enough
    // borrowed value does not live long enoughrustcE0597
    // lifetime_3.rs(31, 5): `y` dropped here while still borrowed
    // lifetime_3.rs(33, 20): borrow later used here
    
    // let x = 3;
    // let mut res;
    // {
    //     let y = 5;
    //     let p = Point{
    //         x: &x,
    //         y: &y
    //     };
    //     res = &p;
    // }
    // println!("Point is: ({}, {})", res.x, res.y);

    // ======================================
    // Case-2
    let x = 3;
    let mut res;
    let y = 5;
    let mut p;
    {
        p = Point{
            x: &x,
            y: &y
        };

        res = &p;

    }

    println!("Point is: ({}, {})", res.x, res.y);
}