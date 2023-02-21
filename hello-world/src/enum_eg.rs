enum color {
    Red,
    Blue,
    Green(String),
}

impl color {
    fn print(&self) {
        match self {
            color::Red => println!("Red"),
            color::Blue => println!("Blue"),
            color::Green(..) => println!("Green"),
        }
    }
}

fn main() {
    let c = color::Red;
    c.print();

    let c = color::Blue;
    c.print();

    let c = color::Green("Light Green".to_string());
    c.print();
}
