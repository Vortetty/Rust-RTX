use super::Scene;
use rand_chacha::ChaCha20Rng;
use crate::{camera::Camera, hittable::HittableList, mats::{MatManager, lambertian::LambertianMat, dielectric::DielectricMat, metal::MetalMat}, color::Color, hittables::sphere::Sphere, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct DofSpheresGlass {}

impl Scene for DofSpheresGlass {
    fn setup(&self, world: &mut HittableList, cam: &mut Camera, mats: &mut MatManager, aspect_ratio: &mut f32, rng: &mut ChaCha20Rng) {
        let ground_mat: i64 = mats.gen_mat(Box::new(LambertianMat {
            albedo: Color {
                r: 86.0 / 255.0,
                g: 125.0 / 255.0,
                b: 70.0 / 255.0,
            },
        }));
        let sphere_center_mat: i64 = mats.gen_mat(Box::new(LambertianMat {
            albedo: Color {
                r: 0.7,
                g: 0.3,
                b: 0.3,
            },
        }));
        let sphere_left_mat: i64 = mats.gen_mat(Box::new(DielectricMat { refract_index: 1.5 }));
        let sphere_right_mat: i64 = mats.gen_mat(Box::new(MetalMat {
            albedo: Color {
                r: 0.8,
                g: 0.6,
                b: 0.2,
            },
            fuzz: 1.0,
        }));

        world.add(Sphere::new_box(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            &ground_mat,
        ));
        world.add(Sphere::new_box(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            &sphere_left_mat,
        ));
        world.add(Sphere::new_box(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            &sphere_left_mat,
        ));
        world.add(Sphere::new_box(
            Vec3::new(-1.0, 0.0, -1.0),
            0.3,
            &sphere_left_mat,
        ));
        world.add(Sphere::new_box(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.25,
            &sphere_left_mat,
        ));
        world.add(Sphere::new_box(
            Vec3::new(-1.0, 0.0, -1.0),
            0.15,
            &sphere_right_mat,
        ));
        world.add(Sphere::new_box(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            &sphere_center_mat,
        ));
        world.add(Sphere::new_box(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            &sphere_right_mat,
        ));

        *aspect_ratio = Self::get_aspect_ratio(self);
        let look_from = Vec3::newi(-2, 2, 1);
        let look_to = Vec3::newi(0, 0, -1);
        let view_up = Vec3::newi(0, 1, 0);
        let focus_dist = (look_from - look_to).length();
        let aperture = 0.5;
        let vert_fov = 35.0;

        *cam = Camera::new(
            look_from,
            look_to,
            view_up,
            vert_fov,
            *aspect_ratio,
            aperture,
            focus_dist,
            rng
        );
    }
}