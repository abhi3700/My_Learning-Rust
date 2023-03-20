#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[allow(unused)]
#[path = "./assembly_line.rs"]
mod assembly_line {
    pub fn production_rate_per_hour(speed: u8) -> f64 {
        if speed >= 1 && speed <= 4 {
            f64::from(speed * 221 * 100 / 100)
        } else if speed >= 5 && speed <= 8 {
            f64::from(speed * 221 * 90 / 100)
        } else if speed >= 9 && speed <= 10 {
            f64::from(speed * 221 * 77 / 100)
        } else {
            f64::from(0)
        }
    }
}
fn main() {
    {
        ::std::io::_print(
            format_args!("{0}\n", assembly_line::production_rate_per_hour(1)),
        );
    };
}
