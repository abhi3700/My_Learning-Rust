/* 
    Implement operator overloading for struct
*/

use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Point {
    x: f32,
    y: f32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y 
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

pub fn run() {
    assert_eq!(
        Point{x: 4.0, y: 5.0}, 
        Point{x: 2.0, y: 2.0} + Point{x: 2.0, y: 3.0}
    );

}