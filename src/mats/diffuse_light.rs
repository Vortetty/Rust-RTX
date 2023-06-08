use std::simd::f32x4;

use crate::{
    color::Color, hittable::HitRecord, material::Material, ray::Ray, utils::random_in_hemisphere,
};

#[derive(Clone, Copy)]
pub struct DiffuseLight {
    pub emit: Color,
}

impl Material for DiffuseLight {
    #[allow(unused_variables)]
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &HitRecord,
        attenuation: &mut f32x4,
        scattered: &mut crate::ray::Ray,
        rng: &mut rand_chacha::ChaCha20Rng,
    ) -> bool {
        return false;
    }

    #[allow(unused_variables)]
    fn emitted(&self, u: f32, v: f32, p: &crate::vec3::Vec3) -> f32x4 {
        return self.emit.to_simd4();
    }
}
