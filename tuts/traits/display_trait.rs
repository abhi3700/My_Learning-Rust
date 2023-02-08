use std::fmt::Display;

pub fn main() {
    let array_cities = vec![
        "kolkata".to_string(),
        "delhi".to_string(),
        "raipur".to_string(),
    ];

    // println!("{}", array_cities); // Err: `Vec<String>` cannot be formatted with the default formatter

    // M-1: using `{:?}`
    // println!("{:?}", array_cities);

    // ********************************************* //
    // M-2: impl Display trait
    let wrapper_cities = WrapperCities {
        cities: array_cities,
    };

    println!("{}", wrapper_cities);
}

// a struct containing vector for which the display impl has to be customized.
struct WrapperCities {
    cities: Vec<String>,
}

impl Display for WrapperCities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cities found")?;

        for city in self.cities.iter() {
            writeln!(f, "- {}", city)?;
        }

        write!(f, "done!")?;
        Ok(())
    }
}
