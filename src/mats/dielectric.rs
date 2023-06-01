use crate::{
    color::Color,
    hittable::HitRecord,
    material::Material,
    ray::Ray,
    utils::{refract, reflect}, vec3::Vec3, rand_double::rand_double,
};

#[derive(Clone, Copy)]
pub struct DielectricMat {
    pub refract_index: f64,
}

impl Material for DielectricMat {
    #[allow(unused_variables)]
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &HitRecord,
        attenuation: &mut crate::color::Color,
        scattered: &mut crate::ray::Ray,
        rng: &mut rand_chacha::ChaCha20Rng,
    ) -> bool {
        *attenuation = Color::new_01_range(1.0, 1.0, 1.0);
        let refraction_ratio: f64 = if rec.front_face {
            1.0/self.refract_index
        } else {
            self.refract_index
        };

        let unit_dir = ray_in.dir.unit_vector();
        let cos_theta = f64::min((-unit_dir).dot_prod(rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || (DielectricMat::reflectance(self, cos_theta, refraction_ratio) > rand_double(rng)) {
            direction = reflect(&unit_dir, &rec.normal);
        } else {
            direction = refract(&unit_dir, &rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(rec.point, direction);
        return true;
    }
}

impl DielectricMat {
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 *= r0;
        return r0 + (1.0-r0)*(1.0-cosine).powf(5.0);
    }
}
