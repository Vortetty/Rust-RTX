use crate::{
    color::Color, hittable::HitRecord, material::Material, ray::Ray, utils::random_in_hemisphere,
};

#[derive(Clone, Copy)]
pub struct DebugFrontMat {}

impl Material for DebugFrontMat {
    #[allow(unused_variables)]
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &HitRecord,
        attenuation: &mut crate::color::Color,
        scattered: &mut crate::ray::Ray,
        rng: &mut rand_chacha::ChaCha20Rng,
    ) -> bool {
        let mut scatter_dir = rec.normal + random_in_hemisphere(&rec.normal, rng);

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        *scattered = Ray::new(rec.point, scatter_dir);
        *attenuation = if rec.front_face {
            Color::new_01_range(1.0, 0.0, 0.0)
        } else {
            Color::new_01_range(0.0, 1.0, 0.0)
        };
        return true;
    }
}
