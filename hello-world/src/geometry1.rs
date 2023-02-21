// define Circle
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// define Rectangle
struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

// define trait
trait HasArea {
    fn area(&self) -> f64;
}

// implement HasArea for Circle
impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

// implement HasArea for Rectangle
impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// define function main
pub fn main() {
    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };
    let r = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    println!("Circle area: {}", c.area());
    println!("Rectangle area: {}", r.area());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_circle_area() {
        let c = Circle {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        };
        assert_eq!(c.area(), std::f64::consts::PI);

        let c = Circle {
            x: 0.0,
            y: 0.0,
            radius: 2.0,
        };
        assert_eq!(c.area(), 4.0 * std::f64::consts::PI);
    }

    #[test]
    fn test_rectangle_area() {
        let r = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };
        assert_eq!(r.area(), 100.0);

        let r = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 5.0,
            height: 2.0,
        };
        assert_eq!(r.area(), 10.0);
    }
}
