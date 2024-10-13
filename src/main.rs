mod rng;
mod primes;

use rng::{Xorshift128, PRNG};
use std::time::Instant;

fn main() {
    let mut xor = Xorshift128::new(123456789, 362436069, 521288629, 88675123);
    
    let mut times = 0;
    for _ in 0..100 {
        let now = Instant::now();
        let num = xor.generate(4096/8);
        times += now.elapsed().as_nanos();

        println!("{}", num.to_string_radix(2).len());
    }

    println!("TIME: {}", times/100);
}
