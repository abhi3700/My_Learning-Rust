//! Use of `thiserror` crate

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MyError {
    #[error("driver not found")]
    DriverNotFound,
    #[error("Price mismatch of `{0}`")]
    PriceMismatch(u32),
    #[error("Ride with `{src:?}`, `{des:?}` Not met")]
    RideNotMet { src: String, des: String },
    #[error("Invalid")]
    Invalid,
}

pub(crate) fn main() -> Result<(), MyError> {
    let x = Some(4);

    let _ = match x {
        Some(1) => MyError::DriverNotFound,
        Some(2) => MyError::PriceMismatch(45),
        Some(3) => MyError::RideNotMet {
            src: "New Town, Kolkata".to_string(),
            des: "Dumdum, Kolkata".to_string(),
        },
        _ => MyError::Invalid,
    };

    Ok(())
}
