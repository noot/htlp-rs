# htlp-rs

This repository contains an implementation of [Homomorphic Timelock Puzzles](https://eprint.iacr.org/2019/635.pdf) (see section 4.1). A timelock puzzle is a cryptographic primitive that encloses a secret value within a puzzle (ie. encrypts it). The puzzle can be quickly generated, but can only be solved (decrypted) over a period of time by repeated squarings, which cannot be parallelized. It's the opposite of a VDF in a sense, which takes a long time to generate but can be quickly verified.

The timelock puzzles are linearly homomorphic in that multiple puzzles can be combined and when solved, their output is the summation of their individual secrets.

Note that the time it takes to solve a puzzle is machine-dependent. If one machine can square faster than another, it'll solve the puzzle quicker. If you want to timelock something for definitely *at least* a certain amount of time, try to benchmark solving on the fastest machine you anticipate could try to solve the puzzle. The time parameter passed into `Parameters::new` isn't tied to any real-life measure of time, apart from that a higher value will take longer to solve.

## Usage

```rust
use htlp::setup::Parameters;

use num_bigint_dig::{RandBigInt, ToBigUint};
use rand;

fn generate_and_solve() {
    let security = 32;
    let time = 100.to_biguint().unwrap();
    let params = Parameters::new(security, time);

    let one = 1.to_biguint().unwrap();
    let mut rng = rand::thread_rng();
    let secret = rng.gen_biguint_range(&one, &(&params.n - &one));
    let puzzle = &params.generate(&secret);

    let res = puzzle.solve();
    debug_assert!(secret == res);
}

fn generate_and_solve_homomorphic() {
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
    debug_assert!(secret == res);
}
```