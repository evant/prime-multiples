use std::ops::{Add, AddAssign};

use gmp_mpfr_sys::gmp;
use num_bigint::BigUint;
use num_traits::identities::Zero;
use primal::Primes;
use rug::{Assign, Integer};
use rug::ops::{Pow, PowAssign};
use std::fs::File;
use std::io::Write;

fn main() {
    // println!("check: {} -> {}", 1139733677usize, check(1139733677usize));
    let mut results = File::create("results2.txt").unwrap();
    let mut acc = Integer::default();
    let mut i2 = Integer::default();
    for (n, i) in Primes::all()
        .enumerate() {
        let n = (n + 1) as u64;
        i2.assign(i);
        i2.pow_assign(2);
        acc.add_assign(&i2);
        if is_divisible(&acc, n) {
            println!("{}", n);
            writeln!(results, "{}", n);
        }
    }
}

#[inline(always)]
fn is_divisible(a: &Integer, b: u64) -> bool {
    return unsafe {
        gmp::mpz_divisible_ui_p(a.as_raw(), b.into()) != 0
    };
}

fn check(n: usize) -> bool {
    let big_n = BigUint::from(n);
    let two = BigUint::from(2u64);
    (Primes::all().take(n)
        .fold(BigUint::zero(), |acc, i| (acc + BigUint::from(i).modpow(&two, &big_n)) % &big_n)
        % &big_n).is_zero()
}
