pub mod generate;
pub mod setup;
pub mod solve;

#[cfg(test)]
mod tests {
    use crate::setup::Parameters;

    use num_bigint_dig::{RandBigInt, ToBigUint};
    use rand;

    #[test]
    fn test_solve() {
        let security = 32;
        let time = 100.to_biguint().unwrap();
        let params = Parameters::new(security, time);

        let one = 1.to_biguint().unwrap();
        let mut rng = rand::thread_rng();
        let secret = rng.gen_biguint_range(&one, &(&params.n - &one));
        let puzzle = &params.generate(&secret);

        let res = puzzle.solve();
        assert_eq!(secret, res);
    }

    #[test]
    fn test_solve_homomorphic() {
        let security = 32;
        let time = 100.to_biguint().unwrap();
        let params = Parameters::new(security, time);

        let one = 1.to_biguint().unwrap();
        let mut rng = rand::thread_rng();
        let secret0 = rng.gen_biguint_range(&one, &(&params.n - &one));
        let puzzle0 = &params.generate(&secret0);
        let secret1 = rng.gen_biguint_range(&one, &(&params.n - &one));
        let puzzle1 = &params.generate(&secret1);

        let puzzle = puzzle0.add(puzzle1);
        let secret = &secret0 + &secret1;

        let res = puzzle.solve();
        assert_eq!(secret, res);
    }
}
