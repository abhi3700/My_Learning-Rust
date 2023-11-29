/*
    - implementing setter, getter functions for a struct w/o defining trait
*/

struct BMW {
    model: String,
    year: u16,
    color: String,
}

impl BMW {
    fn show_features(&self) {
        println!("model: {}", self.model);
        println!("year: {}", self.year);
        println!("color: {}", self.color);
    }

    fn set_color(&mut self, color: String) {
        self.color = color;
    }
}

pub fn run() {
    let mut b = BMW {
        model: "320d".to_string(),
        year: 2020,
        color: "navy blue".to_string(),
    };

    // show all
    b.show_features();

    // change color
    b.set_color("red".to_string());

    println!("=====================");
    // show all
    b.show_features();
}
