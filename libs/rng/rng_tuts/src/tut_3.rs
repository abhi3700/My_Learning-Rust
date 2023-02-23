/*
   generate a rng value within range [0, 10)
   in different distribution

   Uniform distribution are generally faster, especially when there are multiple
   times rng value generated in a given range.
*/

// RNG thread
use rand::{distributions::Uniform, prelude::Distribution};

pub fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    let mut i = 0; // iteration

    // a person wins if throw is 6
    loop {
        i += 1;
        let throw = die.sample(&mut rng);

        if throw == 6 {
            println!("won in {} iterations!", i);
            break;
        }
    }
}
