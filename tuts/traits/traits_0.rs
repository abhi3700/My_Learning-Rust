/* 
    - implementing functions for a struct w/o defining trait
*/

struct BMW {
    model: String,
    year: u16,
    color: String
}

impl BMW {
    fn show_features(&self) {
        println!("model: {}", self.model);
        println!("year: {}", self.year);
        println!("color: {}", self.color);
    }
}

pub fn run() {
    let b = BMW{
        model: "320d".to_string(),
        year: 2020,
        color: "navy blue".to_string()
    };

    b.show_features();
}