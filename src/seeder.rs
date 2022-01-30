use std::hash::Hasher;

use ahash::AHasher;
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::{Rng, RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

pub struct UwUSeeder {
    rng: Xoshiro256Plus,
}

impl UwUSeeder {
    #[inline]
    pub fn new(word: &[u8], random: bool) -> UwUSeeder {
        let rand_u64 = if !random {
            let mut hasher = AHasher::new_with_keys(0, 0);
            hasher.write(word);
            hasher.finish()
        } else {
            rand::rngs::OsRng::default().next_u64()
        };

        UwUSeeder {
            rng: Xoshiro256Plus::seed_from_u64(rand_u64),
        }
    }

    #[inline]
    pub fn random_float(&mut self) -> f64 {
        self.rng.gen_range(0.0..1.0)
    }

    #[inline]
    pub fn random_int<T: SampleUniform, R: SampleRange<T>>(&mut self, range: R) -> T {
        self.rng.gen_range(range)
    }
}