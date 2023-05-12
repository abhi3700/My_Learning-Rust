extern crate derive_more;

use derive_more::Into;

#[derive(Into)]
struct Point2D {
    x: i32,
    y: i32,
}

pub(crate) fn main() {
    let p1 = Point2D { x: 1, y: 0 };
    let p2 = Point2D { x: 1, y: 0 };
    let p3 = Point2D { x: 0, y: 1 };
    assert!((1, 0) == p1.into());
}

/* extern crate derive_more;
use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, From, Add)]
struct MyInt(i32);

#[derive(PartialEq, From, Into)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(PartialEq, From, Add, Display)]
enum MyEnum {
    #[display(fmt = "int: {}", _0)]
    Int(i32),
    Uint(u32),
    #[display(fmt = "nothing")]
    Nothing,
}

pub(crate) fn main() {
    assert!(MyInt(11) == MyInt(5) + 6.into());
    assert!((5, 6) == Point2D { x: 5, y: 6 }.into());
    assert!(MyEnum::Int(15) == (MyEnum::Int(8) + 7.into()).unwrap());
    assert!(MyEnum::Int(15).to_string() == "int: 15");
    assert!(MyEnum::Uint(42).to_string() == "42");
    assert!(MyEnum::Nothing.to_string() == "nothing");
}
 */
