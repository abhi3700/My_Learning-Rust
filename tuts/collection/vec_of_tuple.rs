/*
    Implement CRUD on top of vector of tuples of type (&str, u32).
*/

pub fn run() {
    let mut v1: Vec<(&str, u32)> = vec![("abhijit", 29)];
    v1.push(("Adi", 24));

    println!("{:?}", &v1);

    // read item
    println!("{}", &v1[0].0);

    // update & display
    v1[0].0 = "abhi";
    println!("{:?}", &v1);

    // delete & display
    v1.pop();
    println!("{:?}", &v1);
}
