fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    let mut v2 = vec![1, 2, 3, 4];

    v2.push(34);
    v2.push(867);
    println!("stored at: {:p}", &v2);

    let second = &v2[1];
    println!("{}", second);
    println!("{}", &v2[1]);

    match v2.get(1) {
        Some(second) => println!("Found"),
        None => panic!("Not found"),
    }

    for i in v2.iter_mut() {
        println!("{}", i);
        *i *= 2;
    }

    println!("v2 modified as: {:?}", v2);
}
