//! Generic trait for Generic struct
// TODO: complete this

struct Counter<T> {
    count: T,
}

trait MyTrait<T> {
    fn get(&self) -> T;
}

impl<T> MyTrait<T> for Counter<T>
where
    // it is needed, otherwise it will throw error because of `self.count`
    T: Copy,
{
    fn get(&self) -> T {
        self.count
    }
}

pub fn main() {
    let c = Counter { count: 10 };
    println!("{}", c.get());
}
