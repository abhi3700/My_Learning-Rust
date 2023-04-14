//! Add Crocodile for implementing a supertrait which implements both the traits:
//! - Land
//! - Water

struct Crocodile;

trait Land {
    fn land(&self) {
        println!("Default land");
    }
}

trait Water {
    fn water(&self) {
        println!("Default water");
    }
}

trait Amphibian: Land + Water {}
impl Land for Crocodile {
    fn land(&self) {
        println!("Crocodile can survive on land");
    }
}
impl Water for Crocodile {
    fn water(&self) {
        println!("Crocodile can survive in water");
    }
}
impl Amphibian for Crocodile {}

fn survive(animal: &dyn Amphibian) {
    animal.land();
    animal.water();
}

pub fn main() {
    let croc = Crocodile;
    survive(&croc);
}
