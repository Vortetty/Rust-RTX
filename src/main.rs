mod camera;
mod color;
mod hittable;
mod hittables;
mod material;
mod mats;
mod rand_double;
mod ray;
mod scenes;
mod utils;
mod vec3;
mod aabb;

use std::{
    f64::INFINITY,
    fmt::Write,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use camera::Camera;
use color::Color;
use hittable::{HitRecord, HittableList};
use image::{Rgb, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use mats::MatManager;
use pad::PadStr;
use rand::prelude::*;
use rand::{thread_rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_double::rand_double;
use ray::Ray;
use rayon::{
    current_thread_index,
    prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSlice,
};
use scenes::cornell_box::{CornellBox, self};
use vec3::Vec3;

#[allow(unused_imports)]
use crate::scenes::{dof_spheres_glass::DofSpheresGlass, random_spheres::RandomSpheres, Scene};

use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn ray_color(
    ray: &Ray,
    world: &HittableList,
    rng: &mut ChaCha20Rng,
    mats: &MatManager,
    depth: u64,
) -> Color {
    let mut rec: HitRecord = HitRecord::default();

    if depth <= 0 {
        return Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }

    if world.hit(ray, 0.000000001, INFINITY, &mut rec) {
        //let target = rec.point + random_in_hemisphere(&rec.normal, rng);
        //let tmp_ray = Ray::new(rec.point, target - rec.point);
        //let next_color = ray_color(&tmp_ray, world, rng, depth - 1);
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        let mat = mats.get_mat(&rec.material);
        let emitted = mat.emitted(0.0, 0.0, &Vec3::new(0.0, 0.0, 0.0));

        if mat.scatter(
            ray,
            &rec,
            &mut attenuation,
            &mut scattered,
            rng,
        ) {
            let next_color = ray_color(&scattered, world, rng, mats, depth - 1);
            return Color {
                r: emitted.r + attenuation.r * next_color.r,
                g: emitted.g + attenuation.g * next_color.g,
                b: emitted.b + attenuation.b * next_color.b,
            };
        } else {
            return emitted;
        }
        //return Color {
        //    r: 0.5 * next_color.r,
        //    g: 0.5 * next_color.g,
        //    b: 0.5 * next_color.b,
        //};
        //return Color {
        //    r: 0.0,
        //    g: 0.0,
        //    b: 0.0,
        //};
    }

    let unit_dir = ray.dir;
    let t = 0.5 * (unit_dir.y + 1.0); // y_pos_for_coloring
                                      //return Color {
                                      //    r: 127.5 + t * 127.5,
                                      //    g: 127.5,
                                      //    b: 255.0 - t * 127.5,
                                      //};
    return Color {
        r: (1.0 - t) + 0.5 * t,
        g: (1.0 - t) + 0.7 * t,
        b: (1.0 - t) + t,
    };
}

#[allow(unused_must_use)]
fn main() {
    // -----
    //  RNG
    // -----
    let mut seed: <ChaCha20Rng as SeedableRng>::Seed = Default::default();
    thread_rng().fill(&mut seed);

    let scene = RandomSpheres {};
    let aspect_ratio: f64 = scene.get_aspect_ratio();
    let target_width: u32 = 1920/8;
    let target_height: u32 = (target_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u16 = 500;
    let max_depth: u16 = 50;
    let mprog = Arc::new(MultiProgress::new());
    let bar = Arc::new(mprog.add(ProgressBar::new(target_height as u64)));
    bar.set_style(
        ProgressStyle::with_template("{msg} | {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({eta_precise} @ {per_sec_nice} lines/s)").unwrap()
        .progress_chars("#>-")
        .with_key("per_sec_nice", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.3}", state.per_sec()).unwrap())
    );
    bar.tick();
    bar.set_message(format!("Total").pad_to_width_with_alignment(
        format!("Thread {}", num_cpus::get()).len(),
        pad::Alignment::Left,
    ));
    bar.enable_steady_tick(Duration::from_millis(50));

    let mut rng = ChaCha20Rng::from_seed(seed);
    let mut mats = MatManager::new();
    let mut world = HittableList { objs: vec![] };
    let mut cam = Camera::default(&mut rng);
    let mut aspect_ratio: f64 = 0.0;
    #[allow(unused_mut)]
    scene.setup(&mut world, &mut cam, &mut mats, &mut aspect_ratio, &mut rng);

    let mut smats = Arc::new(mats);
    let mut sworld = Arc::new(world);
    let mut scam = Arc::new(cam);

    let mut rets = (0..target_height)
        .into_par_iter()
        .map(capture_it::capture!(
            [
                target_width,
                target_height,
                samples_per_pixel,
                max_depth,
                &bar,
                &smats,
                &sworld,
                &scam
            ],
            move |line| raytrace(
                target_width,
                target_height,
                samples_per_pixel,
                max_depth,
                line,
                bar,
                smats,
                sworld,
                scam
            )
        ))
        .collect::<Vec<RtRet>>();

    let mut im_raw: Vec<u8> = vec![];
    rets.sort_by_key(|x| x.i_id);
    for mut retv in rets {
        im_raw.append(&mut retv.im);
    }

    let im = RgbImage::from_raw(target_width, target_height, im_raw).unwrap();
    im.save("output.png");
}

struct RtRet {
    im: Vec<u8>,
    i_id: u32,
}

fn raytrace(
    target_width: u32,
    target_height: u32,
    samples_per_pixel: u16,
    max_depth: u16,
    y: u32,
    bar: &Arc<ProgressBar>,
    mats: &MatManager,
    world: &HittableList,
    cam: &Camera
) -> RtRet {
    //let mut rng = ChaCha20Rng::from_seed(seed);

    // ----------------
    //  World / Camera
    // ----------------
    //let mut mats = MatManager::new();
    //let mut world = HittableList { objs: vec![] };
    //let mut cam = Camera::default(&mut rng);
    //let mut aspect_ratio: f64 = 0.0;
    //#[allow(unused_mut)]
    //scene.setup(&mut world, &mut cam, &mut mats, &mut aspect_ratio, &mut rng);
    //let y_iter = split_vec_into_n_groups_and_get_n((0 as u32..target_height as u32).collect::<Vec<u32>>(), num_cpus::get(), thread_num);
    //let y_offset = y_iter[0].clone();
    let mut seed1: <ChaCha20Rng as SeedableRng>::Seed = Default::default();
    thread_rng().fill(&mut seed1);
    let mut rng = ChaCha20Rng::from_seed(seed1);

    // ---------------
    //  Output Buffer
    // ---------------
    //let mut im = RgbImage::new(target_width, /*target_height*/ 1);
    let mut im = vec![];

    // ------------------
    //  RT Without the X
    // ------------------
    for x in 0..target_width {
        let mut pix_color = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
        let t_pix_color = Arc::new(RwLock::new(&mut pix_color));
        for _sample in 0..samples_per_pixel {
            let u = (x as f64 + rand_double(&mut rng)) / (target_width - 1) as f64;
            let v = 1.0 - ((y as f64 + rand_double(&mut rng)) / (target_height - 1) as f64);
            let r = cam.get_ray(u, v, &mut rng);
            let c = ray_color(&r, &world, &mut rng, &mats, max_depth as u64);

            let tmp = Color {
                r: c.r + t_pix_color.read().unwrap().r,
                g: c.g + t_pix_color.read().unwrap().g,
                b: c.b + t_pix_color.read().unwrap().b,
            };
            **t_pix_color.write().unwrap() = tmp;
        }
        let scale = 1.0 / samples_per_pixel as f64;
        //im.put_pixel(
        //    x,
        //    0,
        //    Rgb([
        //        ((pix_color.r * scale).sqrt() * 255.0) as u8,
        //        ((pix_color.g * scale).sqrt() * 255.0) as u8,
        //        ((pix_color.b * scale).sqrt() * 255.0) as u8,
        //    ]),
        //);
        im.push(((pix_color.r * scale).sqrt() * 255.0) as u8);
        im.push(((pix_color.g * scale).sqrt() * 255.0) as u8);
        im.push(((pix_color.b * scale).sqrt() * 255.0) as u8);
    }
    bar.inc(1);

    //let mut raw_out = im.into_raw();
    //raw_out.shrink_to_fit();
    return RtRet {
        im: im, //.into_raw(),
        i_id: y,
    };
}
