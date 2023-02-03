/*
    Learning mutable reference via `&mut` with compound data type like struct, enum
*/

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

pub fn run() {
    // Case-1
    // let p = Person{
    //     first_name: "Abhijit".to_string(),
    //     last_name: "Roy".to_string(),
    //     age: 28
    // };

    // let r = &p;         // Ok
    // r.age = 29;                 // Throws error: cannot assign to `r.age`, which is behind a `&` reference `r` is a `&` reference, so the data it refers to cannot be written

    //---------------------------------------------------
    // Case-2
    // let mut p = Person{
    //     first_name: "Abhijit".to_string(),
    //     last_name: "Roy".to_string(),
    //     age: 28
    // };

    // let r = &mut p;         // for this, declare `p` as mut
    // r.age = 29;

    //---------------------------------------------------
    // Case-3
    // let p = Person{
    //     first_name: "Abhijit".to_string(),
    //     last_name: "Roy".to_string(),
    //     age: 28
    // };

    // let r = &p;
    // let r2 = &p;         // if p is mut, then multiple mutable references is not possible, as the p can change at any moment.
    // let r3 = &p;
    // let r4 = &p;
    // let r5 = &p;
    // let r6 = &p;
    // let r7 = &p;

    //---------------------------------------------------
    // Case-4
    let p = Person {
        first_name: "Abhijit".to_string(),
        last_name: "Roy".to_string(),
        age: 28,
    };

    let r = &p;
    let r2 = &r;
    let r3 = &r2;
    let r4 = &r3;
    let r5 = &r4;
    let r6 = &r5;
    let r7 = &r6;
}
