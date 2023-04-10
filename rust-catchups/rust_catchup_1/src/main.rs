// data types: 2
// 1 common trait
// create a function that returns those 2 defined types

use std::fmt::{self, Debug, Display};

trait Details {
    fn summary(&self) -> String;
}

#[derive(Default)]
struct House {
    area_sqft: u32,
    purchased_date: String,
}

#[derive(Default)]
struct Vehicle {
    name: String,
    model: String,
    purchased_date: String,
}

impl Details for House {
    fn summary(&self) -> String {
        format!(
            "The House that was purchased on {} has an area of {}",
            self.purchased_date, self.area_sqft
        )
    }
}

impl Details for Vehicle {
    fn summary(&self) -> String {
        format!(
            "The vehicle with name {}, model {} is purchased on {}",
            self.name, self.model, self.purchased_date,
        )
    }
}

fn foo(flag: bool) -> Box<dyn Details> {
    if flag {
        Box::new(House::default())
    } else {
        Box::new(Vehicle::default())
    }
}

impl Display for dyn Details {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

impl Debug for dyn Details {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

fn main() {
    println!("{}", foo(true)); // M-1: using Display trait
    println!("{:?}", foo(true)); // M-2: using Debug trait
}
