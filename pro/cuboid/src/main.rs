#[derive(Debug, Clone, Copy)]
struct Cuboid {
    length: u32,
    width: u32,
    height: u32,
}

impl Cuboid {
    fn new(length: u32, width: u32, height: u32) -> Self {
        Cuboid {
            length,
            width,
            height,
        }
    }
    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn add(&self, other: &Self) -> Self {
        Cuboid::new(
            self.length + other.length,
            self.width + other.width,
            self.height + other.height,
        )
    }

    fn total_vol(&self, other: &Self) -> u32 {
        self.volume() + other.volume()
    }
}

fn main() {
    let c = Cuboid::new(2, 3, 4);
    let c2 = Cuboid::new(4, 5, 6);

    println!("Volume: {}", &c.volume());
    println!("New Cuboid: {:?}", &c.add(&c2));
    println!("Total volume: {}", &c.total_vol(&c2));
}
