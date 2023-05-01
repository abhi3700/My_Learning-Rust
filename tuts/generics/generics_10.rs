//! Implement a trait to a generic type
//! In the main function we try out with different types

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x: x, y: y }
    }

    // NOTE: This method output is generic type `T` & not `f32` as in generics_9.rs
    // In order to use operations - *, +, etc. on `T`, we need to add Copy, Mul, Add traits
    // Why Copy? Because, we are using `self.x` & `self.y` twice in the method body
    // Why Mul & Add? Because, we are using `*` & `+` operators in the method body
    // NOTE: We can also use `where` clause to specify the traits
    fn square_xy(&self) -> T
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
    {
        self.x * self.x + self.y * self.y
    }
}

pub fn main() {
    let z = Pair::new(32, 42);
    println!("square xy (i32) = {}", z.square_xy());

    let z = Pair::new(32.0, 42.0);
    println!("square xy (f64) = {}", z.square_xy());

    let z = Pair::new(-32, 42);
    println!("square xy (f64) = {}", z.square_xy());
}
