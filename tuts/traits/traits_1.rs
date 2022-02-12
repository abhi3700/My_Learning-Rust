/* 
    Car example
    - implement a trait `Detail` for different car objects (defined in `struct`)

    NOTE: here, the `String` type creates a problem in return

*/

pub trait Detail {
    fn brand(&self) -> String;
    fn color(&self) -> String;
    fn driving_mode(&self) -> bool;
    fn manu_year(&self) -> u16;
}

#[derive(Debug)]            // used for printing via `println!`
struct Car {
    brand: String,
    color: String,
    mode: bool,         // manual: false, automatic: true
    year: u16
}

impl Detail for Car {
    fn brand(&self) -> String {
        // using `format` instead of directly returning the brand bcoz it throws error:
        // "move occurs because `self.brand` has type `String`, which does not implement the `Copy` trait"
        return format!("{}", self.brand);           
    }
    fn color(&self) -> String {
        return format!("{}", self.color);
    }
    fn driving_mode(&self) -> bool {
        return self.mode;
    }
    fn manu_year(&self) -> u16 {
        return self.year;
    }
    
}

fn main() {
    let c1 = Car {
        brand: "BMW".to_string(),
        color: "Dark Blue".to_string(),
        mode: true,
        year: 2012
    };

    println!("Car c1: \n{:#?}", &c1);
}
