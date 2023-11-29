//! Traits for enums

// Define a trait for drawable objects
trait Drawable {
    fn draw(&self);
}

// Define an enum for different shapes
enum Shape {
    Circle(f64),         // radius
    Rectangle(f64, f64), // width, height
}

// Implement the Drawable trait for the Shape enum
impl Drawable for Shape {
    fn draw(&self) {
        match self {
            Shape::Circle(radius) => println!("Drawing a circle with radius: {}", radius),
            Shape::Rectangle(width, height) => println!(
                "Drawing a rectangle with width: {} and height: {}",
                width, height
            ),
        }
    }
}

fn main() {
    // Create instances of each enum variant
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);

    // Call the draw method from the Drawable trait
    circle.draw();
    rectangle.draw();
}
