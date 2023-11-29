/* 
    - Using `into_iter()`

    - into_iter(), which iterates over T.
*/

pub fn run() {
    let v = vec![5, 3, 8, 4, 7];
    
    let mut it = (v).into_iter();
    println!("first element: {}", it.next().unwrap());

    for i in it {
        println!("{}", i);
    }

    // while let Some(i) = it.next() {
    //     println!("{}", i);
    // }
}