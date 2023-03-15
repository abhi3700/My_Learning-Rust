/*
   Here, there are 2 points - p1, p2 & they need to mixed up via a method `mixup()`.
   So, define generic function `mixup` for the other point - p2.

   Ultimately, returns a result whose types are mixed <X1, Y2>
*/

#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn main() {
    let p1 = Point { x: 1, y: 2.0 };
    let p2 = Point { x: "Hello", y: 'f' };
    println!("{:?}", p1.mixup(p2));
}
