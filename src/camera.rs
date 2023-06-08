use rand_chacha::ChaCha20Rng;

use crate::{ray::Ray, utils::{deg_to_rad, random_in_unit_sphere, random_in_unit_disk}, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_to: Vec3,
        view_up: Vec3,
        vert_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        rng: &mut ChaCha20Rng
    ) -> Camera {
        let theta = deg_to_rad(vert_fov);
        let h = (theta / 2.0).tan();
        //let aspect_ratio: f32 = 16.0 / 9.0;
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w = (look_from - look_to).unit_vector();
        let u = view_up.cross_prod(w).unit_vector();
        let v = w.cross_prod(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - focus_dist * w;

        return Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            w,
            u,
            v,
            lens_radius: aperture/2.0
        };
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut ChaCha20Rng) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;

        return Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        };
    }

    pub fn default(rng: &mut ChaCha20Rng) -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let look_from = Vec3::newi(0, 0, 0);
        let look_to = Vec3::newi(0, 0, -1);
        let view_up = Vec3::newi(0, 1, 0);
        let focus_dist = (look_from - look_to).length();
        let aperture = 0.0;
        let vert_fov = 35.0;

        return Self::new(
            look_from,
            look_to,
            view_up,
            vert_fov,
            aspect_ratio,
            aperture,
            focus_dist,
            rng
        );
    }
}
