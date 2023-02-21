struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub trait Area {
    fn area(&self) -> f64;
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub fn main() {
    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 2.0,
    };
    println!("Circle area: {}", c.area());

    let r = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 20.0,
    };
    println!("Rectangle area: {}", r.area());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_circle_area() {
        let c = Circle {
            x: 0.0,
            y: 0.0,
            radius: 2.0,
        };
        assert_eq!(c.area(), 12.566370614359172);
    }

    #[test]
    fn test_rectangle_area() {
        let r = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 20.0,
        };
        assert_eq!(r.area(), 200.0);
    }
}
