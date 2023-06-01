use rand_chacha::ChaCha20Rng;

use crate::rand_double::rand_double;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            r: r / 255.0,
            g: g / 255.0,
            b: b / 255.0,
        }
    }
    pub fn new_01_range(r: f64, g: f64, b: f64) -> Color {
        Color { r: r, g: g, b: b }
    }
    pub fn rand(rng: &mut ChaCha20Rng) -> Color {
        Color {
            r: rand_double(rng),
            g: rand_double(rng),
            b: rand_double(rng),
        }
    }
}
