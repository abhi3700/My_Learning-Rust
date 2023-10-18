use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[cfg(target_os = "macos")]
// #[cfg(feature = "my_feature")]
fn calculate_hash<T: Hash>(t: &mut T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

use eyre::{eyre, Result};

fn divide_by_zero_1(a: u32, b: u32) -> Result<u32> {
    if b == 0 {
        return Err(eyre!("Cannot divide by zero"));
    } else {
        Ok(a / b)
    }
}

use color_eyre::eyre::{self, Report};

fn divide(dividend: f64, divisor: f64) -> Result<f64, Report> {
    if divisor == 0.0 {
        Err(eyre!("Attempted to divide by zero"))
    } else {
        Ok(dividend / divisor)
    }
}

#[cfg(feature = "main1")]
fn main() -> std::result::Result<(), Report> {
    color_eyre::install()?;

    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {:#?}", e),
    }

    Ok(())
}

// #[cfg(feature = "main2")]
fn main() -> Result<()> {
    // let mut hasher = DefaultHasher::new();
    // 7930_i32.hash(&mut hasher);
    // hasher.finish();
    let res = calculate_hash(&mut 7930_i32);
    println!("{:?}", res);

    // let _ = divide_by_zero(3, 0);
    match divide_by_zero_1(3, 0) {
        Ok(o) => println!("{}", o),
        Err(e) => eprintln!("{:#?}", e),
    }

    println!("{:?}", divide_by_zero_1(1, 5).unwrap());

    let a = [1, 4, 5];
    let x = a.iter().map(|&x| (x as i32).pow(4)).collect::<Vec<i32>>();
    println!("{:?}", x);

    Ok(())
}
