// simple
fn div(a: f32, b: f32) -> f32 {
    a / b
}

// using Option
fn div1(a: f32, b: f32) -> Option<f32> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

// using Result
fn div2(a: f32, b: f32) -> Result<f32, &'static str> {
    if b == 0.0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

// using Result + Option
fn div3(a: f32, b: f32) -> Result<Option<f32>, &'static str> {
    if b == 0.0 {
        Err("Division by zero")
    } else {
        Ok(Some(a / b))
    }
}

pub fn main() {
    // println!("Division result: {}", div(1.0, 2.0));
    // println!("Division result: {}", div1(1.0, 2.0).unwrap());
    // if div2(1.0, 0.0).is_err() {
    //     println!("Division result: not divisible");
    // }
    println!("Division result: {}", div3(1.0, 2.0).unwrap().unwrap());
}
