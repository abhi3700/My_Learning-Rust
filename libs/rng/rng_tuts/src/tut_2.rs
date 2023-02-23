/*
   generate a rng value within range [0, 10)
*/

// RNG thread
use rand::Rng;

pub fn main() {
    let mut rng = rand::thread_rng();
    println!("A random no. in [0, 10): {}", rng.gen_range(0..10));
}
