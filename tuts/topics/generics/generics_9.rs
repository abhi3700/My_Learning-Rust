//! implement a generic struct type methods with a specific type (say f32)

struct Pair<T> {
    x: T,
    y: T,
}

impl Pair<f32> {
    fn new(x: f32, y: f32) -> Self {
        Pair { x: x, y: y }
    }

    fn square_xy(&self) -> u32 {
        (self.x as u32).pow(2) + (self.y as u32).pow(2)
    }
}

pub fn main() {
    let z = Pair::new(32.0, 42.0);
    println!("square xy = {}", z.square_xy());
}
