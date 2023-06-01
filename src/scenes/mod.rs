pub mod dof_spheres_glass;
pub mod random_spheres;
pub mod cornell_box;

use rand_chacha::ChaCha20Rng;
use crate::{camera::Camera, hittable::HittableList, mats::MatManager};

pub trait Scene {
    fn setup(&self, world: &mut HittableList, camera: &mut Camera, mats: &mut MatManager, aspect_ratio: &mut f64, rng: &mut ChaCha20Rng);

    fn get_aspect_ratio(&self) -> f64 {
        return 16.0 / 9.0;
    }
}