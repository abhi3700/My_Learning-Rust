/*
   generate a rng value with a normal distribution {mean: 2.0, std_dev: 3.0}
   For more, refer Wiki: https://en.wikipedia.org/wiki/Normal_distribution

   Using the rng_thread inside the normal distribution sample from the hill curve.
*/

// RNG thread
use rand_distr::{Distribution, Normal, NormalError};

pub fn main() -> Result<(), NormalError> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0)?;

    let n = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", n);

    Ok(())
}
