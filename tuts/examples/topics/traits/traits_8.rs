/*
    Apply single trait (Area) to multiple structs (Circle, Rectangle, Triangle)

*/

// Circle
#[derive(Debug)]
struct Circle {
    radius: f64,
}

// Rectangle
#[derive(Debug)]
struct Rectangle {
    length: f64,
    width: f64,
}

// Triangle
#[derive(Debug)]
struct Triangle {
    s1: f64,
    s2: f64,
    s3: f64,
}

// common trait
trait Area {
    fn area(&self) -> f64;
}

//=== Implementations
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        let s = (self.s1 + self.s2 + self.s3) / 2.0;
        (s * (s - self.s1) * (s - self.s2) * (s - self.s3)).sqrt()
    }
}

// main
pub fn main() {
    let circle = Circle { radius: 5.0 };

    let rectangle = Rectangle {
        length: 10.0,
        width: 5.0,
    };

    let triangle = Triangle {
        s1: 3.0,
        s2: 4.0,
        s3: 5.0,
    };

    println!("Area of circle is: {}", circle.area());
    println!("Area of rectangle is: {}", rectangle.area());
    println!("Area of triangle is: {}", triangle.area());
}
