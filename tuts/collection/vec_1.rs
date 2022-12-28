/*
    - Vector: Resizeable arrays
    Try vector with:
    1. for-range
    2. iterator
*/

fn main() {
    // let names = vec!("Abhi", "Sam", "McLaren", "Gary");      // OR
    let mut names = vec!["Abhi", "Sam", "McLaren", "Gary"];
    names.push("Jagjeet");
    names.push("Javed");
    names.push("Parmaneet");
    println!("{:?}", names);

    // remove the last value
    names.pop();

    //--------------------------------------------------------------
    // M-1
    println!("{:?}", names);

    //--------------------------------------------------------------
    // M-2
    for name in &names {
        println!("{}", name);
    }

    //--------------------------------------------------------------
    // M-3
    // If this line doesn't exist, then it throws error like 'can't be used' as already used before in `for-range`
    let names: Vec<&str> = vec!["Abhi", "Sam", "McLaren", "Gary"];
    let mut iterator = (&names).into_iter();

    while let Some(name) = iterator.next() {
        println!("{}", name);
    }
    //--------------------------------------------------------------
    for name in names.iter() {
        println!("{}", name);
    }

    //--------------------------------------------------------------
    println!("length: {}", names.len()); // number of elements in the vector i.e. <= capacity
    println!("capacity: {}", names.capacity()); // memory allocated for the vector without resizing i.e. >= length
    println!("Vector occupies: {} bytes", std::mem::size_of_val(&names));

    let slice = &names[1..3];
    println!("Slice: {:?}", slice);
}
