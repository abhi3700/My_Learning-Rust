/* 
    - Using next method on `iter()`
*/

pub fn run() {
    let v = vec![5, 3, 8, 4];
    
    let mut it = v.iter();

    println!("{}", it.next().unwrap());
    println!("{}", it.next().unwrap());
    println!("{}", it.next().unwrap());
    println!("{}", it.next().unwrap());
    
    // throws error as 
    // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/../tuts/iterator/iter_3.rs:14:30
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    println!("{}", it.next().unwrap());     
}