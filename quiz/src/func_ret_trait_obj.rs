//! Key learnings:
//! ================
//! Q. How to make the function return incompatible types?
//! A. Using Boxed trait object. Here, the trait has to be applied for/shared by the incompatible types.
//!====================================================================================
//! Q. Why `Debug` trait is implemented here?
//! A. It's implemented because of printing `foo()` on console using `{:?}` in `println!()`. 2 methods:
//! M-1: So, at 3 places, it has to be implemented as `#[derive(Debug)]` on struct:
//!     - `Details` trait
//!     - `House` struct
//!     - `Vehicle` struct
//! E.g. `House` & `Vehicle` structs have to implement `Debug` trait as in `main()` function, there is a need to implement `Debug` trait
//!
//! M-2 [RECOMMENDED]: It can be implemented via `impl Debug for dyn Details {}`. Here, we can have
//! customized printable string as well.
//!====================================================================================
//! Q. Why `Display` trait is implemented here?
//! A. It's implemented because of printing `foo()` on console using `{}` in `println!()`. 2 methods:
//! M-1: So, at 3 places, it has to be implemented as `#[derive(Display)]` on struct:
//!     - `Details` trait
//!     - `House` struct
//!     - `Vehicle` struct
//! E.g. `House` & `Vehicle` structs have to implement `Debug` trait as in `main()` function, there is a need to implement `Debug` trait
//!
//! M-2 [RECOMMENDED]: It can be implemented via `impl Display for dyn Details {}`. Here, we can have
//! customized printable string as well.
//!====================================================================================
//!
//! To ensure a function is able to return incompatible types, that can be done by using a trait bound that is shared among the 2 struct types.

#[allow(unused_imports)]
use std::fmt::Display;
use std::fmt::{self, Debug, Formatter};

/// Struct representing a house
#[derive(Default)]
struct House {
    area_sqft: u32,
    purchase_date: String,
}

/// Struct representing a vehicle
#[derive(Default)]
struct Vehicle {
    name: String,
    model: String,
    purchase_date: String,
}

trait Details {
    fn summary(&self) -> String;
}

impl Details for House {
    fn summary(&self) -> String {
        format!(
            "The house has an area of {} sqft and was purchased on {}",
            self.area_sqft, self.purchase_date
        )
    }
}
impl Details for Vehicle {
    fn summary(&self) -> String {
        format!(
            "The {} vehicle with model {} was purchased on {}",
            self.name, self.model, self.purchase_date
        )
    }
}

/// function returns incompatible types
// This function takes a boolean flag as an argument and returns a Box containing either a House or Vehicle object.
fn foo(flag: bool) -> Box<dyn Details> {
    // If the flag is true, return a House object
    if flag {
        Box::new(House {
            area_sqft: 5000,
            purchase_date: "21 Nov 2017".to_string(),
        })
    // Otherwise, return a Vehicle object
    } else {
        Box::new(Vehicle {
            name: "BMW".to_string(),
            model: "320d".to_string(),
            purchase_date: "13 Aug 2022".to_string(),
        })
    }
}

// M-1: Using `trait Details: Display`, but also need to implement for structs
// M-2 [RECOMMENDED]:
impl Display for dyn Details {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

// M-1: Using `trait Details: Debug`, but also need to implement for structs
// M-2 [RECOMMENDED]:
impl Debug for dyn Details {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

pub fn main() {
    let x = foo(true);
    println!("{}", x.summary());

    // for this we have to implement `Debug` trait at 3 places - 1 trait, 2 structs
    println!("===\n{:?}", foo(true));

    // For this, we have to implement `Display` trait which is implemented at 1 places - 1 trait, 2 structs
    println!("===\n{}", foo(true));
}

#[cfg(test)]
mod test {

    use super::*;

    fn init() -> (House, Vehicle) {
        let house = House {
            area_sqft: 5000,
            purchase_date: "21 Nov 2017".to_string(),
        };
        let vehicle = Vehicle {
            name: "BMW".to_string(),
            model: "320d".to_string(),
            purchase_date: "13 Aug 2022".to_string(),
        };

        (house, vehicle)
    }

    // initialize function for test functions
    #[test]
    fn check_foo_returns_house_if_true() {
        let (house, _) = init();
        assert_eq!(house.summary(), foo(true).summary());
    }

    #[test]
    fn check_foo_returns_vehicle_if_false() {
        let (_, vehicle) = init();
        assert_eq!(vehicle.summary(), foo(false).summary());
        // assert_eq!(Box::new(vehicle) as Box<dyn Details>, foo(false));
    }

    #[test]
    fn check_purchase_date_for_house() {
        let (house, _) = init();
        assert_eq!(
            format!(
                "The house has an area of {} sqft and was purchased on {}",
                house.area_sqft, house.purchase_date
            ),
            house.summary()
        );
    }

    #[test]
    fn check_purchase_date_for_vehicle() {
        let (_, vehicle) = init();
        assert_eq!(
            format!(
                "The {} vehicle with model {} was purchased on {}",
                vehicle.name, vehicle.model, vehicle.purchase_date
            ),
            vehicle.summary()
        );
    }
}
