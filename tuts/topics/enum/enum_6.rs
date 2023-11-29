//! This enum eg teaches us to implement Display trait for enum
//! There is a crate: `derive_more` which can help add `#[derive(Display)]` to enum
//! Refer this example: ../libs/crates_derive_more
/*

#[derive(Display)]
enum MyEnum {
    #[display(fmt = "MyInt({})", _0)]
    MyInt(i32),
    MyFloat(f64),
    // #[display(fmt = "Nothing")]
    Nothing,
}

pub fn main() {
    let x = MyEnum::MyInt(10);
    println!("{}", x);
    println!("{}", MyEnum::Nothing);
}

 */
