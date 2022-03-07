/* 
    Using `unwrap` with Result
    - Before
    - After
*/

fn function_w_error() -> Result<u8, String> {
    if true {
        Ok(10)
    } else {
        Err("invalid value".to_string())
    }
}

pub fn run() {
    // M-1: Before using `unwrap()`
    // let x;

    // match function_w_error() {
    //     Ok(v) => x = v,
    //     Err(e) => panic!("{e}")
    // }

    // println!("{}", x);

    //===================================
    // M-2: After using `unwrap()`
    let x = function_w_error().unwrap();

    println!("{}", x);
}