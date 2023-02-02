/*
    Here, optimized use of collect() is shown.

    Observations:
    t0 took 522.734583ms, whereas
    t1 took 433.291583ms

    Hence, using operation on collected vector takes less time than otherwise.

*/

use std::time::Instant;

pub fn main() {
    let s = "Abhijit is a good boy";
    let v = s.bytes().collect::<Vec<u8>>();
    println!("bytes collected in form of vec: \n{:?} \n", v);

    // ****************************************** //
    let t0 = Instant::now();
    let mut sum0 = 0;
    for _ in 0..1000000 {
        for b in s.bytes() {
            sum0 += b as usize;
        }
    }

    println!("time elapsed: {:?}", t0.elapsed());
    println!("sum0: {}", sum0);

    // ****************************************** //

    let t1 = Instant::now();
    let mut sum1 = 0;
    for _ in 0..1000000 {
        for &b in &v {
            sum1 += b as usize;
        }
    }

    println!("time elapsed: {:?}", t1.elapsed());
    println!("sum1: {}", sum1);
}
