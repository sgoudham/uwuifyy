use rand::{Rng, rngs::ThreadRng, thread_rng};
use rand_pcg::Pcg32;
use rand_seeder::Seeder;

pub struct UwUSeeder {
    seeder: Pcg32,
    rng: ThreadRng,
    random: bool,
}

impl UwUSeeder {
    pub fn new(word: &str, random: bool) -> UwUSeeder {
        UwUSeeder {
            seeder: Seeder::from(word).make_rng(),
            rng: thread_rng(),
            random,
        }
    }

    pub fn random(&mut self) -> f32 {
        if self.random {
            self.rng.gen_range(0.0..1.0)
        } else {
            self.seeder.gen_range(0.0..1.0)
        }
    }

    pub fn random_int(&mut self, min: i32, max: i32) -> usize {
        if self.random {
            self.rng.gen_range(min..max) as usize
        } else {
            self.seeder.gen_range(min..max) as usize
        }
    }
}