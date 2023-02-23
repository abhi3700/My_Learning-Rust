/*
   Generate random Point (struct) via implementing Distribution trait
   on Point for Standard
*/

use rand::Rng;
use rand_distr::{Distribution, Standard};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

pub fn main() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();

    // Error thrown if not implemented Distribution trait for Standard.
    let rand_point: Point = rng.gen::<Point>();

    println!("rand tuple: {:?}", rand_tuple);
    println!("rand Point: {:?}", rand_point);
}
