/*


*/
// M-1
fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone()); // FIXME: here `.clone()` is used in order to not get it moved, but rather make a copy using `clone()`.

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// ******************************************************************************* //

// M-2

/*
fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0); // FIXME: borrow using &

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// FIXME: borrow & in arg
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.to_owned(); // FIXME: to owned type i.e. `Vec<i32>`. Read more [here](./README.md#)

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
 */
