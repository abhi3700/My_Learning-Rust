/* 
    Try vector with: 
    1. for-range
    2. iterator
*/

fn main() {
    // let names = vec!("Abhi", "Sam", "McLaren", "Gary");      // OR
    let names = vec!["Abhi", "Sam", "McLaren", "Gary"];

    //--------------------------------------------------------------
    // M-1
    for name in names {
        println!("{}", name);
    }

    //--------------------------------------------------------------
    // M-2
    // If this line doesn't exist, then it throws error like 'can't be used' as already used before in `for-range`
    let names = vec!["Abhi", "Sam", "McLaren", "Gary"];
    let mut iterator = (names).into_iter();
    
    while let Some(name) = iterator.next() {
        println!("{}", name);
    }

}