enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn is_weekend(&self) -> bool {
        match self {
            Day::Saturday | Day::Sunday => {
                println!("Weekend!");
                true
            }
            _ => {
                println!("Not a weekend");
                false
            }
        }
    }
}

fn main() {
    let today = Day::Monday;
    assert_eq!(today.is_weekend(), false);
}
