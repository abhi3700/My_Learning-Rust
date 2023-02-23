/*
   Generate a rng value of different types
*/

use rand::Rng;

pub fn main() {
    let mut rng = rand::thread_rng();
    println!("bool rand value: {}", rng.gen::<bool>());
    println!("u32 rand no.: {}", rng.gen::<u32>());
    println!("u16 rand no.: {}", rng.gen::<u16>());
    println!("i64 rand no.: {}", rng.gen::<i64>());
    println!("f64 rand no.: {}", rng.gen::<f64>());
}
