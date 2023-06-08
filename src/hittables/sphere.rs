use std::{f32::consts::PI, simd::f32x2};

use crate::{vec3::Vec3, hittable::Hittable, hittable::HitRecord, ray::Ray, aabb::AABB};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: i64
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, trace_len_min: f32, trace_len_max: f32, rec: &mut HitRecord) -> bool {
        let bbox = AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius)
        };
        if !bbox.hit(ray, trace_len_min, trace_len_max) {
            return false
        }

        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let b_half = oc.dot_prod(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = b_half * b_half - a * c;
        if discriminant < 0.0 {
            return false;
        };
        let sqrt_disc = f32::sqrt(discriminant);

        let mut root = (-b_half - sqrt_disc) / a;
        if root <  trace_len_min || trace_len_max < root {
            root = (-b_half - sqrt_disc) / a;
            if root <  trace_len_min || trace_len_max < root {
                return false
            };
        };

        rec.trace_len = root;
        rec.point = ray.at(rec.trace_len);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(&ray, &outward_normal);
        rec.material = self.material;

        let theta = f32::acos(-outward_normal.y);
        let phi = f32::atan2(-outward_normal.z, -outward_normal.x);

        let tmp = f32x2::from_array([phi, theta]) / f32x2::from_array([2.0*PI, PI]);

        rec.tex_u = tmp[0]; //phi / (2.0 * PI);
        rec.tex_v = tmp[1]; //theta / PI;

        return true;
    }

    fn bounds(&self, output_box: &mut crate::aabb::AABB) -> bool {
        output_box.max = self.center - Vec3::new(self.radius, self.radius, self.radius);
        output_box.min = self.center + Vec3::new(self.radius, self.radius, self.radius);
        return true;
    }
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(_center: Vec3, _radius: f32, material: &i64) -> Sphere {
        return Sphere{
            center: _center,
            radius: _radius,
            material: material.clone()
        }
    }
    pub fn new_box(_center: Vec3, _radius: f32, material: &i64) -> Box<Sphere> {
        return Box::new(Sphere{
            center: _center,
            radius: _radius,
            material: material.clone()
        })
    }
}
