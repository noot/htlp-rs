use num_bigint_dig::{BigUint, RandBigInt, ToBigUint};
use num_integer::Integer;
use rand;

use crate::setup::Parameters;

/// Puzzle represents a timelock puzzle.
pub struct Puzzle {
    pub u: BigUint,
    pub v: BigUint,
    pub params: Parameters,
}

impl Parameters {
    /// generate generates a new timelock puzzle with the given secret.
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
        let v = (&hrn * &ns).mod_floor(&n2);
        debug_assert!(&v != &zero);

        Puzzle {
            u,
            v,
            params: self.clone(),
        }
    }
}

impl Puzzle {
    /// add returns the combination of the two puzzles.
    /// when solved, the solution is the sum of the two puzzles's secrets.
    pub fn add(&self, other: &Puzzle) -> Puzzle {
        if &self.params != &other.params {
            panic!("puzzle parameters do not match")
        }

        let n2 = &self.params.n * &self.params.n; // n^2
        let u = (&self.u * &other.u).mod_floor(&self.params.n);
        let v = (&self.v * &other.v).mod_floor(&n2);
        Puzzle {
            u,
            v,
            params: self.params.clone(),
        }
    }
}
