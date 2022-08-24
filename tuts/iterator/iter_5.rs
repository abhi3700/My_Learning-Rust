pub fn run() {
    let v1 = vec![1, 5, 6, 8];

    // M-1
    // for i in v1.into_iter() {
    //     println!("{}", i);
    // }

    // can't be used here as it is moved into loop because of use of `into_iter()`
    // println!("{:?}", v1);    // Comment this to avoid error

    // M-2
    for i in v1.iter() {
        println!("{}", i);
    }

    println!("{:?}", v1);
}
