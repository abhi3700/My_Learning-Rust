/*
    Car example
    - implement a trait `Detail` for different car objects (defined in `struct`)

    NOTE: here, the `String` type creates a problem in return

*/

pub trait Detail {
    fn brand(&self) -> String;
    fn model(&self) -> String;
    fn car_name(&self) -> String;
    fn color(&self) -> String;
    fn driving_mode(&self) -> bool;
    fn manu_year(&self) -> u16;
}

// NOTE: Even after using Clone in derive attribute, it throws the same error
#[derive(Debug)] // used for printing via `println!`
struct Car {
    brand: String,
    model: String,
    color: String,
    mode: bool, // manual: false, automatic: true
    year: u16,
}

impl Detail for Car {
    fn brand(&self) -> String {
        // using `format` instead of directly returning the brand bcoz it throws error:
        // "move occurs because `self.brand` has type `String`, which does not implement the `Copy` trait"
        return format!("{}", self.brand);
        // return self.brand;      // throws Error
    }

    fn model(&self) -> String {
        format!("{}", self.model)
    }

    fn car_name(&self) -> String {
        format!("{} {}", self.brand, self.model)
    }

    fn color(&self) -> String {
        return format!("{}", self.color);
        // return self.color;      // throws Error
    }
    fn driving_mode(&self) -> bool {
        self.mode
    }
    fn manu_year(&self) -> u16 {
        self.year
    }
}

pub fn main() {
    let c1 = Car {
        brand: "BMW".to_string(),
        model: "320d".to_string(),
        color: "Dark Blue".to_string(),
        mode: true,
        year: 2012,
    };

    println!("Car c1: \n{:#?}", &c1); // print the whole car c1
    println!("Car c1 brand: {}", c1.brand()); // only print the c1's brand
    println!("Car c1 model: {}", c1.car_name()); // print the car brand + model name
}
