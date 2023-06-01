use std::cmp::Ordering;
use std::sync::Arc;

use rand_chacha::ChaCha20Rng;

use crate::hittable::Hittable;
use crate::aabb::AABB;
use crate::rand_double::rand_double;
use crate::vec3::{self, Vec3};

use super::sphere::Sphere;

pub struct BvhNode {
    left: Arc<Box<dyn Hittable + Sync + Send>>,
    right: Arc<Box<dyn Hittable + Sync + Send>>,
    bound: AABB
}

impl Hittable for BvhNode {
    fn bounds(&self, output_box: &mut AABB) -> bool {
        output_box.max = self.bound.max;
        output_box.min = self.bound.min;
        return true;
    }

    fn hit(&self, r: &crate::ray::Ray, trace_len_min: f64, trace_len_max: f64, rec: &mut crate::hittable::HitRecord) -> bool {
        if self.bound.hit(r, trace_len_min, trace_len_max) {
            return false
        };

        let hit_left = self.left.hit(r, trace_len_min, trace_len_max, rec);
        let hit_right = self.right.hit(r, trace_len_min, if hit_left {rec.trace_len} else {trace_len_max}, rec);

        return hit_left || hit_right;
    }
}

fn box_compare(a: &Arc<Box<dyn Hittable + Sync + Send>>, b: &Arc<Box<dyn Hittable + Sync + Send>>, axis: i8) -> Ordering {
    let mut box_a = AABB { min: Vec3::newi(0,0,0), max: Vec3::newi(0,0,0) };
    let mut box_b = AABB { min: Vec3::newi(0,0,0), max: Vec3::newi(0,0,0) };

    if !a.bounds(&mut box_a) || !a.bounds(&mut box_b) {
        panic!("No bounding box in bvh_node constructor");
    };

    if box_a.min.idx(axis) < box_b.min.idx(axis) {
        return Ordering::Less;
    } else if box_a.min.idx(axis) > box_b.min.idx(axis) {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}

impl BvhNode {
    pub fn new(src_objs: &Vec<Arc<Box<dyn Hittable + Sync + Send>>>, start: usize, end: usize, rng: &mut ChaCha20Rng) -> BvhNode {
        let mut objs = src_objs.clone();

        let axis = (rand_double(rng) as i8) % 3;

        let comp = match axis {
            0 => | a: &Arc<Box<dyn Hittable + Sync + Send>>, b: &Arc<Box<dyn Hittable + Sync + Send>> | { return box_compare(a, b, 0) },
            1 => | a: &Arc<Box<dyn Hittable + Sync + Send>>, b: &Arc<Box<dyn Hittable + Sync + Send>> | { return box_compare(a, b, 1) },
            2 => | a: &Arc<Box<dyn Hittable + Sync + Send>>, b: &Arc<Box<dyn Hittable + Sync + Send>> | { return box_compare(a, b, 2) },
            _ => | a: &Arc<Box<dyn Hittable + Sync + Send>>, b: &Arc<Box<dyn Hittable + Sync + Send>> | { return box_compare(a, b, 2) },
        };

        let obj_span = end-start;

        let mut out = BvhNode {
            left: Arc::new(Box::new(Sphere::new(Vec3::newi(0,0,0), 0.0, &0))),
            right: Arc::new(Box::new(Sphere::new(Vec3::newi(0,0,0), 0.0, &0))),
            bound: AABB { min: Vec3::newi(0,0,0), max: Vec3::newi(0,0,0) },
        };

        if obj_span == 1 {
            out.left = out.right;
            out.right = objs[start].clone();
        } else if obj_span == 2 {
            if comp(&objs[start], &objs[start+1]) == Ordering::Less {
                out.left = objs[start].clone();
                out.right = objs[start+1].clone();
            } else {
                out.left = objs[start+1].clone();
                out.right = objs[start].clone();
            }
        } else {
            objs.sort_unstable_by(comp);

            let mid = start + obj_span/2;
            out.left = Arc::new(Box::new(BvhNode::new(&objs, start, mid, rng)));
            out.right = Arc::new(Box::new(BvhNode::new(&objs, mid, end, rng)));
        }

        return out;
    }
}