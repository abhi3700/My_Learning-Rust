/* 
    Using `unwrap` with Option
    - Before
    - After

    In simple words, `unwrap` i.e. `unwrap_ok`/`unwrap_some` means unfold the value within Some or Ok
*/

fn get_optional_value() -> Option<&'static str> {
    if true {
        Some("abc")
    } else {
        None
    }
}

pub fn run() {
    // M-1: Before using `unwrap()`
    // let x;

    // match get_optional_value() {
    //     Some(v) => x = v,
    //     None => panic!()
    // }

    // println!("{}", x);

    //===================================
    // M-2: After using `unwrap()`
    let x = get_optional_value().unwrap();

    println!("{}", x);
}