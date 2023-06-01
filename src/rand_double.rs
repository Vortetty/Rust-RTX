use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub fn rand_double(rng: &mut ChaCha20Rng) -> f64 {
    return (*rng).next_u64() as f64/u64::MAX as f64;
}

pub fn rand_double_range(rng: &mut ChaCha20Rng, min: f64, max: f64) -> f64 {
    return min + (max-min)*rand_double(rng);
}