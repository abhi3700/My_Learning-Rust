/* 
    - Using `into_iter()`
*/

pub fn run() {
    let v = vec![5, 3, 8, 4];
    
    let mut it = (v).into_iter();

    while let Some(i) = it.next() {
        println!("{}", i);
    }
}