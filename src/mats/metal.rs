use std::simd::f32x4;

use crate::{
    color::Color, hittable::HitRecord, material::Material, ray::Ray, utils::{reflect, random_in_unit_sphere},
};

#[derive(Clone, Copy)]
pub struct MetalMat {
    pub albedo: Color,
    pub fuzz: f32
}

impl Material for MetalMat {
    #[allow(unused_variables)]
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &HitRecord,
        attenuation: &mut f32x4,
        scattered: &mut crate::ray::Ray,
        rng: &mut rand_chacha::ChaCha20Rng,
    ) -> bool {
        let reflected = reflect(&ray_in.dir.unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.point, reflected + self.fuzz * random_in_unit_sphere(rng));
        *attenuation = self.albedo.to_simd4();
        return true;
    }
}
