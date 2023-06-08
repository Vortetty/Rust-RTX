use std::simd::f32x4;

use rand_chacha::ChaCha20Rng;

use crate::rand_double::rand_double;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: r / 255.0,
            g: g / 255.0,
            b: b / 255.0,
        }
    }
    pub fn new_01_range(r: f32, g: f32, b: f32) -> Color {
        Color { r: r, g: g, b: b }
    }
    pub fn new_simd4(rgb_: f32x4) -> Color {
        let [r,g,b, _] = rgb_.as_array().to_owned();
        Color { r, g, b }
    }
    pub fn rand(rng: &mut ChaCha20Rng) -> Color {
        Color {
            r: rand_double(rng),
            g: rand_double(rng),
            b: rand_double(rng),
        }
    }
    pub fn to_simd4(&self) -> f32x4 {
        return f32x4::from_array([self.r, self.g, self.b, 0.0])
    }
}
