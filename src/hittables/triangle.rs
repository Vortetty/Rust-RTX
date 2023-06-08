use std::f32::EPSILON;
use crate::{vec3::Vec3, hittable::Hittable, hittable::HitRecord, ray::Ray};

#[derive(Clone, Copy)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub material: i64
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, trace_len_min: f32, trace_len_max: f32, rec: &mut HitRecord) -> bool { // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
        //let edge1 = self.v1 - self.v0;
        //let edge2 = self.v2 - self.v0;
        //let h = r.dir.cross_prod(edge2);
        //let a = edge1.dot_prod(h);
//
        //if a > -0.0000001 && a < 0.0000001 {
        //    return false; // Parallel, or close enough that it's not worth testing for collision
        //};
//
        //let f = 1.0 / a;
        //let s = r.orig - self.v0;
        //let u = f * s.dot_prod(h);
//
        //if u < 0.0 || u > 1.0 {
        //    return false; // No collision
        //};
//
        //let q = s.cross_prod(edge1);
        //let v = f * r.dir.dot_prod(q);
//
        //if u < 0.0 || u + v > 1.0 {
        //    return false; // No collision
        //};
//
        //let t = f * edge2.dot_prod(q);
//
        //// Find collision
//
        //if t > 0.0000001 {
        //    // Intersection
        //    //if t > trace_len_min && t < trace_len_max {
        //        rec.trace_len = t;
        //        rec.point = r.orig + r.dir * t;
        //        rec.material = self.material;
        //        rec.set_face_normal(r, &(edge1 * edge2));
        //        return true;
        //    //} else {
        //    //    return false;
        //    //}
        //} else {
        //    return false;
        //}

        //let v0v1 = self.v1 - self.v0;
        //let v0v2 = self.v2 - self.v0;
        //let rov0 = r.orig - self.v0;
        //let n = v0v1.cross_prod(v0v2);
        //let q = rov0.cross_prod(r.dir);
        //let d = 1.0/r.dir.dot_prod(n);
        //let u = d*((-q).dot_prod(v0v2));
        //let v = d*(q.dot_prod(v0v1));
        //let t = d*((-n).dot_prod(rov0));
        //if (u < 0.0 || v < 0.0 || (u+v) > 1.0) {return false;};

        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let pvec = r.dir.cross_prod(v0v2);
        let det = v0v1.dot_prod(pvec);
        if det < f32::EPSILON {
                return false;
        };
        let idet = 1.0/det;
        let tvec = r.orig - self.v0;
        let u = tvec.dot_prod(pvec) * idet;
        if u < 0.0 || u > 1.0 {
                return false;
        }
        let qvec = tvec.cross_prod(v0v1);
        let v = r.dir.dot_prod(qvec) * idet;
        if v < 0.0 || u+v > 1.0 {
                return false;
        }
        let t = v0v2.dot_prod(qvec) * idet;
        if t < trace_len_min || trace_len_max < t {
                return false;
        }

        rec.tex_u = u;
        rec.tex_v = v;
        rec.trace_len = t;
        rec.set_face_normal(r, &v0v1.cross_prod(v0v2));
        rec.material = self.material;
        rec.point = r.at(t);
        return true;
    }

    fn bounds(&self, output_box: &mut crate::aabb::AABB) -> bool {
        return false;
    }
}