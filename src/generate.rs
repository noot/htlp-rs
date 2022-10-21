use num_bigint_dig::{BigUint, RandBigInt, ToBigUint};
use rand;

use crate::setup::Parameters;

pub struct Puzzle {
    pub u: BigUint,
    pub v: BigUint,
}

impl Parameters {
    pub fn generate(&self, s: &BigUint) -> Puzzle {
        let zero = 0.to_biguint().unwrap();
        let one = 1.to_biguint().unwrap();

        let n2 = &self.n * &self.n; // n^2
        let mut rng = rand::thread_rng();
        let r = rng.gen_biguint_range(&one, &(&n2 - &one));

        // u = g^r mod n
        let u = self.g.modpow(&r, &self.n);
        debug_assert!(&u != &zero);

        // v = h^(r*n) * (1 + n)^s mod n^2
        let rn = &r * &self.n; // r*n
        let hrn = self.h.modpow(&rn, &n2); // h^(r*n)
        let ns = (&self.n + &one).modpow(&s, &n2); // (1 + n)^s
        let v = (&hrn * &ns).modpow(&one, &n2);
        debug_assert!(&v != &zero);

        Puzzle { u, v }
    }
}
