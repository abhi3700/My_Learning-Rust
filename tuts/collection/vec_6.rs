// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = Vec::from(a); // TODO: declare your vector here with the macro for vectors

    (a, v)
}

/// Run this command `$ cargo test`,
/// no need to use `vec_6::array_and_vec()` in the `src/main.rs`

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
