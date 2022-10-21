use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_primes::{Generator};
use rand;

pub struct Parameters {
    pub t: BigUint,
    pub n: BigUint,
    pub g: BigUint,
    pub h: BigUint,
}

impl Parameters {
    pub fn new(security: usize, t: BigUint) -> Self {
        let one = 1.to_biguint().unwrap();
        let two = 2.to_biguint().unwrap();
        let mut rng = rand::thread_rng();

        let p = gen_safe_prime(security);
        let q = gen_safe_prime(security);
        let n = &p * &q;

        let low = 0.to_biguint().unwrap();
        let g_tilda = rng.gen_biguint_range(&low, &n);
        let g = g_tilda.modpow(&two, &n);
        let g = (&n - g).modpow(&one, &n);
        let phi_n = (&p - &one) * (&q - &one);
        let two_t = two.modpow(&t, &(&phi_n / &two));
        let h = g.modpow(&two_t, &n);

        Parameters { t, n, g, h }
    }
}

fn gen_safe_prime(security: usize) -> BigUint {
    let p = Generator::safe_prime(security);
    BigUint::from_bytes_be(&p.to_bytes_be())
}
