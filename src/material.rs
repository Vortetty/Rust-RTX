use rand_chacha::ChaCha20Rng;

use crate::{ray::Ray, color::Color, hittable::HitRecord, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray, rng: &mut ChaCha20Rng) -> bool;
    #[allow(unused_variables)]
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }
}