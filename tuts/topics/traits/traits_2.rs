/* 
    Implement a trait (related to maths) for rectangle
*/

pub trait MathFunc {
    fn calc_perimeter(&self) -> f64;
    fn calc_area(&self) -> f64;
}

struct Rectangle {
    l: f64,
    b: f64
}

impl MathFunc for Rectangle {
    fn calc_perimeter(&self) -> f64 {
        2.0 * (self.l + self.b) 
    }
    fn calc_area(&self) -> f64 {
        self.l * self.b 
    }
}

fn main() {
    let r1 = Rectangle {
        l: 32.34,
        b: 45.67
    };

    println!("Perimeter: {}", r1.calc_perimeter());
    println!("Area: {}", r1.calc_area());
}