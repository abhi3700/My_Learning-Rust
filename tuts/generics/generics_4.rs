/* 
    Apply generics over Enum
*/

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

pub fn run() {

}