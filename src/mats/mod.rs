pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod diffuse_light;
pub mod debug_front;

use std::collections::HashMap;
use crate::{material::Material, color::Color};
use self::lambertian::LambertianMat;

pub struct MatManager {
    pub mat_map: HashMap<i64, Box<dyn Material>>,
    pub mat_counter: i64
}

impl MatManager {
    pub fn new() -> MatManager {
        let mut tmp = MatManager { mat_map: HashMap::new(), mat_counter: -1 };
        tmp.gen_mat(Box::new(LambertianMat {
            albedo: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
            },
        }));
        return tmp;
    }

    pub fn gen_mat(&mut self, mat: Box<dyn Material>) -> i64 {
        self.mat_counter += 1;
        self.mat_map.insert(self.mat_counter, mat);
        return self.mat_counter;
    }

    pub fn get_mat(&self, mat: &i64) -> &Box<dyn Material> {
        return &self.mat_map[mat];
    }
}

unsafe impl Send for MatManager {}
unsafe impl Sync for MatManager {}