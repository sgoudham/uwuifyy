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
    pub fn new(word: &str, random: bool) -> UwUSeeder {
        let entropy = if !random {
            let mut hasher = ahash::AHasher::default();
            hasher.write(word.as_bytes());
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
        f64::from_ne_bytes(self.floating.next_u64().to_ne_bytes())
    }

    #[inline]
    pub fn random_int<T: SampleUniform, R: SampleRange<T>>(&mut self, range: R) -> T {
        self.int.gen_range(range)
    }
}
