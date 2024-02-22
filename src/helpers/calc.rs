use std::mem::replace;

use num_bigint::BigUint;
use num_traits::{
    Zero,
    One
};

pub fn calculate_fibonacci(num: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();

    for _ in 0..num {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }

    f0
}