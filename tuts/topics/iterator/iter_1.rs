/* 
    - Using `iter()`
    
    - iter(), which iterates over &T.
*/

pub fn run() {
    let v = vec![5, 3, 8, 4];
    
    // mut usage is optional. 
    // It's needed only if the `it` is going to 
    // change its position to next element like in next line
    let mut it = v.iter();
    println!("first element: {}", it.next().unwrap());      // here, `it` is changing the pointer to next element. So, define `it` as mut.

    // for loop
    for i in it {
        println!("{}", i);
    }

    // white loop
    // while let Some(i) = it.next() {
    //     println!("{}", i);
    // }

}