use crate::{ray::Ray, vec3::Vec3};

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            // let t0 = f64::min(
            //     (self.min.idx(i) - ray.orig.idx(i)) / ray.dir.idx(i),
            //     (self.max.idx(i) - ray.orig.idx(i)) / ray.dir.idx(i),
            // );
            // let t1 = f64::max(
            //     (self.min.idx(i) - ray.orig.idx(i)) / ray.dir.idx(i),
            //     (self.max.idx(i) - ray.orig.idx(i)) / ray.dir.idx(i),
            // );
            // let t_min = f64::max(t0, t_min);
            // let t_max = f64::min(t1, t_max);
            // if t_max <= t_min {
            //     return false;
            // };

            let mut mt_min = t_min.clone();
            let mut mt_max = t_max.clone();

            let invd = 1.0 / ray.dir.idx(i);
            let t0 = (self.min.idx(i) - ray.orig.idx(i))* invd;
            let t1 = (self.max.idx(i) - ray.orig.idx(i))* invd;
            if invd < 0.0 {
                mt_min = if t1 > mt_min { t1 } else { mt_min };
                mt_max = if t0 < mt_max { t0 } else { mt_max };
            } else {
                mt_min = if t0 > mt_min { t0 } else { mt_min };
                mt_max = if t1 < mt_max { t1 } else { mt_max };
            }
            if mt_max <= mt_min {
                return false;
            };
        }
        return true;
    }
}
