/* 
    - Using `iter()`
*/

pub fn run() {
    let v = vec![5, 3, 8, 4];
    
    let mut it = v.iter();
    // println!("{}", it.next().unwrap());

    for i in it {
        println!("{}", i);
    }

}