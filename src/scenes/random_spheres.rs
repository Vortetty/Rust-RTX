use super::Scene;
use crate::{
    camera::Camera,
    color::Color,
    hittable::HittableList,
    hittables::{sphere::Sphere, bvh_node::BvhNode},
    mats::{dielectric::DielectricMat, lambertian::LambertianMat, metal::MetalMat, MatManager, diffuse_light::DiffuseLight},
    rand_double::{rand_double, rand_double_range},
    vec3::Vec3,
};
use rand_chacha::ChaCha20Rng;

#[derive(Clone, Copy)]
pub struct RandomSpheres {}

impl Scene for RandomSpheres {
    fn setup(
        &self,
        world: &mut HittableList,
        cam: &mut Camera,
        mats: &mut MatManager,
        aspect_ratio: &mut f32,
        rng: &mut ChaCha20Rng,
    ) {
        //let mut world = HittableList { objs: vec![] };
        let ground_mat: i64 = mats.gen_mat(Box::new(LambertianMat {
            albedo: Color::new_01_range(0.5, 0.5, 0.5),
        }));
        world.add(Sphere::new_box(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            &ground_mat,
        ));

        let mat1: i64 = mats.gen_mat(Box::new(DielectricMat { refract_index: 1.5 }));
        let mat2: i64 = mats.gen_mat(Box::new(LambertianMat {
            albedo: Color::new_01_range(0.4, 0.2, 0.1),
        }));
        let mat3: i64 = mats.gen_mat(Box::new(MetalMat {
            albedo: Color {
                r: 0.7,
                g: 0.6,
                b: 0.5,
            },
            fuzz: 0.0,
        }));
        let mat4: i64 = mats.gen_mat(Box::new(DiffuseLight {
            emit: Color {
                r: 5.0,
                g: 5.0,
                b: 5.0,
            }
        }));

        world.add(Sphere::new_box(Vec3::new(0.0, 1.0, 0.0), 1.0, &mat1));
        world.add(Sphere::new_box(Vec3::new(-4.0, 1.0, 0.0), 1.0, &mat2));
        world.add(Sphere::new_box(Vec3::new(4.0, 1.0, 0.0), 1.0, &mat4));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rand_double(rng);
                let center = Vec3::new(
                    a as f32 + 0.9 * rand_double(rng),
                    0.2,
                    b as f32 + 0.9 * rand_double(rng),
                );

                if (center
                    - Vec3 {
                        x: 4.0,
                        y: 0.2,
                        z: 0.0
                    })
                .length()
                    > 0.9
                {
                    let mut inner_sphere = false;
                    let sphere_mat = mats.gen_mat(if choose_mat < 0.7 {
                        Box::new(LambertianMat {
                            albedo: Color::new_01_range(
                                rand_double(rng) * rand_double(rng),
                                rand_double(rng) * rand_double(rng),
                                rand_double(rng) * rand_double(rng),
                            ),
                        })
                    } else if choose_mat < 0.85 {
                        Box::new(MetalMat {
                            albedo: Color::new_01_range(
                                rand_double_range(rng, 0.0, 0.5),
                                rand_double_range(rng, 0.0, 0.5),
                                rand_double_range(rng, 0.0, 0.5),
                            ),
                            fuzz: rand_double_range(rng, 0.0, 0.5),
                        })
                    } else {
                        inner_sphere = rand_double(rng) >= 0.5;
                        Box::new(DielectricMat { refract_index: rand_double_range(rng, 0.5, 2.5) })
                    });

                    world.add(Sphere::new_box(center, 0.2, &sphere_mat));
                    if inner_sphere {
                        world.add(Sphere::new_box(center, 0.15, &sphere_mat));
                    }
                }
            }
        }

        *aspect_ratio = Self::get_aspect_ratio(self);
        let look_from = Vec3::newi(13, 2, 3);
        let look_to = Vec3::newi(0, 0, 0);
        let view_up = Vec3::newi(0, 1, 0);
        let focus_dist = 10.0;
        let aperture = 0.1;
        let vert_fov = 20.0;

        *cam = Camera::new(
            look_from,
            look_to,
            view_up,
            vert_fov,
            *aspect_ratio,
            aperture,
            focus_dist,
            rng,
        );

        //world_out.add(Box::new(BvhNode::new(&world.objs, 0, world.objs.len(), rng)));
    }

    fn get_aspect_ratio(&self) -> f32 {
        return 3.0 / 2.0;
    }
}
