/*
   Collect the resultant array after operation (like division) with each element via collect() method.

   In this case, it is Vec<Result<i32, DivisionError>>
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
        .collect::<Vec<Result<i32, DivisionError>>>();
    // TODO: can be shortened.
    let division_results = division_results
        .into_iter()
        .map(|n| n.unwrap())
        .collect::<Vec<i32>>();
    println!("{:?}", division_results);
}
