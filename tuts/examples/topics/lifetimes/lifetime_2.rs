/* 
Here, the function `smallest_number` is taken by the compiler as:

fn smallest_number<'a>(n: &'a [i32]) -> &'a i32 {
    ...
}
*/

fn smallest_number(n: &[i32]) -> &i32 {
    let mut s = &n[0];

    // iterate through the remaining array & 
    // check if the current number is smaller than the 0th index
    for r in &n[1..] {
        if r < s {
            s = r;
        }
    }

    s
}

pub fn run() {
    // Case-1
    // let s;
    // {
    //     let v = [3, 5, 2, 8];
    //     s = smallest_number(&v);        // throws error: v doesn't live long enough
    // }
    // println!("{}", s);

    // Case-2
    let s;
    let v = [3, 5, 2, 8];
    s = smallest_number(&v);        // throws error: v doesn't live long enough
    println!("{}", s);
    
}