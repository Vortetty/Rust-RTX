use super::Scene;
use rand_chacha::ChaCha20Rng;
use crate::{camera::Camera, hittable::HittableList, mats::{MatManager, lambertian::LambertianMat, dielectric::DielectricMat, metal::MetalMat, debug_front::DebugFrontMat, diffuse_light::DiffuseLight}, color::Color, hittables::{sphere::Sphere, triangle::Triangle}, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct CornellBox {}

impl Scene for CornellBox {
    fn setup(&self, world: &mut HittableList, cam: &mut Camera, mats: &mut MatManager, aspect_ratio: &mut f32, rng: &mut ChaCha20Rng) {
        let front_debug_mat = mats.gen_mat(Box::new(DebugFrontMat{}));
        let black_lambert = mats.gen_mat(Box::new(LambertianMat{albedo:Color::new(0.0, 0.0, 0.0)}));

        //world.add(Box::new(Triangle {
        //    v0: Vec3::new(0.0, 0.0, 0.0),
        //    v1: Vec3::new(1.0, 0.0, 0.0),
        //    v2: Vec3::new(0.0, 0.0, 1.0),
        //    material: front_debug_mat
        //}));
        //world.add(Box::new(Triangle {
        //    v0: Vec3::new(0.0, 0.0, 0.0),
        //    v1: Vec3::new(-1.0, 0.0, 0.0),
        //    v2: Vec3::new(0.0, 0.0, 1.0),
        //    material: front_debug_mat
        //}));
        //world.add(Box::new(Triangle {
        //    v0: Vec3::new(0.0, 0.0, 0.0),
        //    v1: Vec3::new(-1.0, 0.0, 0.0),
        //    v2: Vec3::new(0.0, 0.0, -1.0),
        //    material: front_debug_mat
        //}));
        world.add(Box::new(Triangle {
            v0: Vec3::new(0.0, 0.0, 0.0),
            v1: Vec3::new(1.0, 0.0, 0.0),
            v2: Vec3::new(0.0, 0.0, 1.0),
            material: front_debug_mat
        }));
        //world.add(Box::new(Sphere {
        //    center: Vec3::new(0.0, 0.0, -1.0),
        //    radius: 0.5,
        //    material: front_debug_mat
        //}));

        *aspect_ratio = Self::get_aspect_ratio(self);
        let look_from = Vec3::newi(0, -4, 0);
        let look_to = Vec3::newi(0, 0, 0);
        let view_up = Vec3::newi(1, 0, 0);
        let focus_dist = (look_from - look_to).length();
        let aperture = 0.0;
        let vert_fov = 90.0;

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

    fn get_aspect_ratio(&self) -> f32 {
        return 1.0;
    }
}