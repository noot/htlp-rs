use num_bigint_dig::{BigUint, ModInverse, ToBigUint};
use num_integer::Integer;

use crate::generate::Puzzle;

impl Puzzle {
    /// solve solves the puzzle and returns the secret enclosed in it.
    pub fn solve(&self) -> BigUint {
        let zero = 0.to_biguint().unwrap();
        debug_assert!(&self.u != &zero);
        debug_assert!(&self.v != &zero);

        let one = 1.to_biguint().unwrap();
        let two = 2.to_biguint().unwrap();
        let n2 = &self.params.n * &self.params.n;

        let mut w = self.u.clone();
        let mut i = 0.to_biguint().unwrap();

        // w = u^(2*t) mod n
        loop {
            w = w.modpow(&two, &self.params.n);
            i = i + &one;
            if i == self.params.t {
                break;
            }
        }

        // w^n mod n2
        w = w.modpow(&self.params.n, &n2);
        debug_assert!(&w != &zero);

        // w^(-n)
        let wn = w.mod_inverse(&n2).unwrap().to_biguint().unwrap();
        debug_assert!(&wn != &zero);

        // v*(w^(-n))
        let vw = &self.v * &wn;
        debug_assert!(&vw != &zero);

        let snum = vw.mod_floor(&n2) - &one; // v*(w^(-n)) (mod n^2) - 1
        let s = &snum / &self.params.n; // s = [ v*(w^(-n)) (mod n^2) - 1 ] / n
        s
    }
}
