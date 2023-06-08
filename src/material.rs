use std::simd::f32x4;

use rand_chacha::ChaCha20Rng;

use crate::{ray::Ray, color::Color, hittable::HitRecord, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut f32x4, scattered: &mut Ray, rng: &mut ChaCha20Rng) -> bool;
    #[allow(unused_variables)]
    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> f32x4 {
        return Color::new(0.0, 0.0, 0.0).to_simd4();
    }
}