//! Add Display trait to enum via `derive_more` crate
//! This enum eg teaches us to implement Display trait for enum
extern crate derive_more;

use derive_more::Display;

#[derive(Display)]
enum MyEnum {
    #[display(fmt = "MyInt({})", _0)]
    MyInt(i32),
    MyFloat(f64),
    #[display(fmt = "nothing")]
    Nothing,
}

pub fn main() {
    let x = MyEnum::MyInt(10);
    println!("{}", x); // -> MyInt(10)
    println!("{}", MyEnum::Nothing); // -> nothing
}
