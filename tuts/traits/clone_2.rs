/* 
    - Using macro for the other types like struct when it is to be copied into.
    - clone() for a struct can only be used when the Clone trait is applied via `derive` macro.

    NOTE: `Debug` trait applied for struct in order to make it printable.

*/

#[derive(Debug, Clone)]
struct Morpheus {
    blue_pill: f32,
    red_pill: i64
}

pub fn run() {
    let m = Morpheus {
        blue_pill: 74.5,
        red_pill: -467
    };

    let m_copy = m.clone();

    println!("{:?}", m_copy);
    println!("{:?}", m);
}