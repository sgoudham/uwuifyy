use std::hash::Hasher;

use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng, RngCore, SeedableRng,
};
use rand_xoshiro::{Xoshiro256Plus, Xoshiro256PlusPlus};

pub struct UwUSeeder {
    floating: Xoshiro256Plus,
    int: Xoshiro256PlusPlus,
}

impl UwUSeeder {
    #[inline]
    pub fn new(word: &[u8], random: bool) -> UwUSeeder {
        let entropy = if !random {
            let mut hasher = ahash::AHasher::new_with_keys(1234, 5678);
            hasher.write(word);
            hasher.finish()
        } else {
            rand::rngs::OsRng.next_u64()
        };

        UwUSeeder {
            floating: Xoshiro256Plus::seed_from_u64(entropy),
            int: Xoshiro256PlusPlus::seed_from_u64(entropy),
        }
    }

    #[inline]
    pub fn random(&mut self) -> f64 {
        self.floating.gen_range(0.0..1.0)
    }

    #[inline]
    pub fn random_int<T: SampleUniform, R: SampleRange<T>>(&mut self, range: R) -> T {
        self.int.gen_range(range)
    }
}
