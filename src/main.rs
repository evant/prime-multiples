use std::ops::{AddAssign};

use gmp_mpfr_sys::gmp;
use primal::Primes;
use rug::{Assign, Integer};
use rug::ops::{PowAssign};
use std::fs::File;
use std::io::Write;

fn main() {
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
