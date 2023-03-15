/*
   Here, we implement methods for generic struct
*/

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Here, the Point is of generic type
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

// Here, the Point is of type f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn main() {
    // able to print point-p1 with the impl methods
    let p1 = Point { x: 1, y: 2 };
    println!("Point p1: ({0}, {1})", p1.get_x(), p1.get_y());

    // able to print point-p2 with the impl methods & `distance_from_origin()`
    let p2 = Point { x: 3.0, y: 4.0 };
    println!("Point p2: ({0}, {1})", p2.get_x(), p2.get_y());
    println!("Distance from origin: {}", p2.distance_from_origin());
}
