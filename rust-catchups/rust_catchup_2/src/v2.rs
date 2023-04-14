//! implement v1 using trait

struct Cat;
struct Fish;

trait Survival {
    fn surv(&self);
}

impl Survival for Cat {
    fn surv(&self) {
        println!("Cat can survive on land");
    }
}

impl Survival for Fish {
    fn surv(&self) {
        println!("Fish can survive in water");
    }
}

fn survive(animal: &impl Survival) {
    animal.surv();
}

pub fn main() {
    let c = Cat;
    survive(&c);

    let f = Fish;
    survive(&f);
}
