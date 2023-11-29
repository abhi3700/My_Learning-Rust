/*
   Collect the resultant array after operation (like division) with each element via collect() method.
   In this case, it is Result<Vec<i32>, DivisionError>  [RECOMMENDED] bcoz any function would result some value or error.
*/

#[derive(Debug)]
struct DivisionError;

fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        return Err(DivisionError);
    }
    Ok(a / b)
}

fn main() {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers
        .into_iter()
        .map(|n| divide(n, 27))
        .collect::<Result<Vec<i32>, DivisionError>>();
    println!("{:?}", division_results.unwrap()); // -> [1, 11, 1426, 3]
}
