/* 
    Result<Ok, Err>
    E.g. If a function can output `u8` with Error as String type.
    
    It has also built_in methods like `is_ok` & `is_err`
*/

pub fn run() {
    let x = Some("Hello World!");

    assert_eq!(x.is_some(), true);
    assert_eq!(x.is_none(), false);

    let y: Result<u8, String> = Ok(10);
    assert_eq!(y.is_ok(), true);
    assert_eq!(y.is_err(), false);
}