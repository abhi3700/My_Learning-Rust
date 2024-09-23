//! Write a divide function with 2 crates - `color-eyre`, `eyre`.
//!
//! Compared to trivial `Result` type function, eyre seemed to be more illustrative and then came `color-eyre`
//! which seemed to be even more descriptive.

use eyre::{eyre, Result};

/// considering the Result of eyre
fn divide_by_zero(a: u32, b: u32) -> Result<u32> {
    if b == 0 {
        return Err(eyre!("Attempt to divide by zero"));
    } else {
        Ok(a / b)
    }
}

/// considering eyre
#[cfg(feature = "main1")]
pub(crate) fn main() -> Result<()> {
    match divide_by_zero(3, 0) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("{:?}", e),
    }

    Ok(())
}

/// considering color_eyre
fn divide(a: u32, b: u32) -> Result<u32, Report> {
    if b == 0 {
        return Err(eyre!("Attempt to divide by zero"));
    } else {
        Ok(a / b)
    }
}

// use color_eyre::Report;
use color_eyre::eyre::{self, Report};

#[cfg(feature = "main2")]
pub(crate) fn main() -> std::result::Result<(), Report> {
    color_eyre::install()?;

    match divide(4, 0) {
        Ok(s) => println!("{}", s),
        // NOTE: Add {:#?} to just get the message and {:?} to get the stacktrace.
        Err(e) => eprintln!("{:?}", e),
    }
    Ok(())
}
