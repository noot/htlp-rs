use num_bigint_dig::{BigUint, ModInverse, ToBigUint};

use crate::generate::Puzzle;
use crate::setup::Parameters;

impl Puzzle {
    pub fn solve(&self, params: &Parameters) -> BigUint {
        let zero = 0.to_biguint().unwrap();
        debug_assert!(&self.u != &zero);
        debug_assert!(&self.v != &zero);

        let one = 1.to_biguint().unwrap();
        let n2 = &params.n * &params.n;

        let mut w = self.u.clone();
        let mut i = 0.to_biguint().unwrap();

        // w = u^(2*t) mod n
        loop {
            w = &w * &w;
            w = w.modpow(&one, &params.n);
            i = i + &one;
            if i == params.t {
                break;
            }
        }

        // w^n mod n2
        w = w.modpow(&params.n, &n2);
        debug_assert!(&w != &zero);

        // w^(-n)
        let wn = w.mod_inverse(&n2).unwrap().to_biguint().unwrap();
        debug_assert!(&wn != &zero);

        // v*(w^(-n))
        let vw = &self.v * &wn;
        debug_assert!(&vw != &zero);

        let snum = vw.modpow(&one, &n2) - &one; // v*(w^(-n)) (mod n^2) - 1
        let s = &snum / &params.n;
        s
    }
}

impl Parameters {
    pub fn solve(&self, puzzle: &Puzzle) -> BigUint {
        puzzle.solve(self)
    }
}
