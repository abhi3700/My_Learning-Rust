//! Add polymorphism without traits.

struct Cat;
struct Fish;

fn survive_cat(animal: &Cat) {
    println!("Cat can survive on land");
}

fn survive_fish(animal: &Fish) {
    println!("Fish can survive in water");
}

pub fn main() {
    let c = Cat;
    survive_cat(&c);

    let f = Fish;
    survive_fish(&f);
}
