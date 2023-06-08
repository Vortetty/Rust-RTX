use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub fn rand_double(rng: &mut ChaCha20Rng) -> f32 {
    return (*rng).next_u64() as f32/u64::MAX as f32;
}

pub fn rand_double_range(rng: &mut ChaCha20Rng, min: f32, max: f32) -> f32 {
    return min + (max-min)*rand_double(rng);
}