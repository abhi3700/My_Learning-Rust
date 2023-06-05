//! implement v1 using trait

struct Cat;
struct Fish;

trait Survival {
    fn surv(&self) {}
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

// M-1: static dispatch [RECOMMENDED]
fn survive(animal: &impl Survival) {
    animal.surv();
}

// M-2: dynamic dispatch
// fn survive(animal: &dyn Survival) {
//     animal.surv();
// }

// M-3.1: generic
// fn survive<T: Survival>(animal: &T) {
//     animal.surv();
// }

// M-3.2
// fn survive<T>(animal: &T)
// where
//     T: Survival,
// {
//     animal.surv();
// }

pub fn main() {
    let c = Cat;
    survive(&c);

    let f = Fish;
    survive(&f);
}
