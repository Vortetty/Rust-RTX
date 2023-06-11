#![feature(portable_simd)]
mod aabb;
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

use std::{
    f32::INFINITY,
    fmt::Write,
    ops,
    simd::{self, f32x4, StdFloat},
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use atomic_counter::{AtomicCounter, RelaxedCounter};
use camera::Camera;
use color::Color;
use hittable::{HitRecord, HittableList};
use humantime::format_duration;
use image::{Rgb, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use mats::MatManager;
use pad::PadStr;
use rand::prelude::*;
use rand::{thread_rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_double::{rand_double, rand_double_range};
use ray::Ray;
use rayon::{
    current_thread_index,
    prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSlice,
};
use scenes::cornell_box::{self, CornellBox};
use vec3::Vec3;
use thread_priority::*;

#[allow(unused_imports)]
use crate::scenes::{dof_spheres_glass::DofSpheresGlass, random_spheres::RandomSpheres, Scene};

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn ray_color(
    ray: &Ray,
    world: &HittableList,
    rng: &mut ChaCha20Rng,
    mats: &MatManager,
    depth: u64,
) -> f32x4 {
    let mut rec: HitRecord = HitRecord::default();

    if depth <= 0 {
        return Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
        .to_simd4();
    }

    if world.hit(ray, 0.000000001, INFINITY, &mut rec) {
        //let target = rec.point + random_in_hemisphere(&rec.normal, rng);
        //let tmp_ray = Ray::new(rec.point, target - rec.point);
        //let next_color = ray_color(&tmp_ray, world, rng, depth - 1);
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0).to_simd4();
        let mat = mats.get_mat(&rec.material);
        let emitted = mat.emitted(0.0, 0.0, &Vec3::new(0.0, 0.0, 0.0));

        if mat.scatter(ray, &rec, &mut attenuation, &mut scattered, rng) {
            let next_color = ray_color(&scattered, world, rng, mats, depth - 1);
            return emitted + attenuation * next_color;
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
    }
    .to_simd4();
}

fn format_bar(bar: &ProgressBar) {
    bar.set_style(
        ProgressStyle::with_template("{msg} | {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({eta_precise} @ {per_sec_nice} lines/s)").unwrap()
        .progress_chars("#>-")
        .with_key("per_sec_nice", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.3}", state.per_sec()).unwrap())
    );
    bar.tick();
    bar.enable_steady_tick(Duration::from_millis(50));
}

#[allow(unused_must_use)]
fn main() {
    // -----
    //  RNG
    // -----
    let mut seed: <ChaCha20Rng as SeedableRng>::Seed = Default::default();
    thread_rng().fill(&mut seed);

    let scene = RandomSpheres {};
    let aspect_ratio: f32 = scene.get_aspect_ratio();
    let target_width: u32 = 1920;
    let target_height: u32 = (target_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel: u16 = 100;
    let max_depth: u16 = 50;
    let mprog = Arc::new(MultiProgress::new());
    let bar = Arc::new(mprog.add(ProgressBar::new(target_height as u64)));
    format_bar(&bar);
    bar.set_message("Render Lines");

    let mut rng = ChaCha20Rng::from_seed(seed);
    let mut mats = MatManager::new();
    let mut world = HittableList { objs: vec![] };
    let mut cam = Camera::default(&mut rng);
    let mut aspect_ratio: f32 = 0.0;
    #[allow(unused_mut)]
    scene.setup(&mut world, &mut cam, &mut mats, &mut aspect_ratio, &mut rng);

    let mut smats = Arc::new(mats);
    let mut sworld = Arc::new(world);
    let mut scam = Arc::new(cam);

    let counter = RelaxedCounter::new(0);

    let start = Instant::now();
    let mut rets = (0..num_cpus::get()) //target_height)
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
                &scam,
                &counter
            ],
            move |line| raytrace(
                target_width,
                target_height,
                samples_per_pixel,
                max_depth,
                counter,
                bar,
                smats,
                sworld,
                scam,
                line
            )
        ))
        .collect::<Vec<Vec<RtRet>>>()
        .join(&RtRet {
            im: vec![],
            i_id: usize::MAX,
        });
    let run_finished = start.elapsed();

    let bar1 = mprog.add(ProgressBar::new(target_height as u64));
    format_bar(&bar1);
    bar1.set_message("Merge Lines");
    let mut im_raw: Vec<u8> = vec![];
    rets.sort_by_key(|x| x.i_id);
    for mut retv in rets {
        im_raw.append(&mut retv.im);
        bar1.inc(1);
    }
    let line_concat_finish = start.elapsed();

    let im_raw_len = im_raw.len();
    let expected = target_width * target_height * 3;
    let im = RgbImage::from_raw(target_width, target_height, im_raw).unwrap();
    im.save("output.png");
    let save_finish = start.elapsed();

    let runtime = format_duration(run_finished);
    let mergetime = format_duration(line_concat_finish - run_finished);
    let savetime = format_duration(save_finish - line_concat_finish);
    let totaltime = format_duration(save_finish);

    println!("Rendering took   {runtime}");
    println!("Merge lines took {mergetime}");
    println!("Saving took      {savetime}");
    println!("Total time taken {totaltime}");
    println!("File saved as    output.png");
}

#[derive(Clone)]
struct RtRet {
    im: Vec<u8>,
    i_id: usize,
}

fn raytrace(
    target_width: u32,
    target_height: u32,
    samples_per_pixel: u16,
    max_depth: u16,
    y_counter: &RelaxedCounter,
    bar: &Arc<ProgressBar>,
    mats: &MatManager,
    world: &HittableList,
    cam: &Camera,
    threadid: usize
) -> Vec<RtRet> {
    ThreadPriority::Max.set_for_current().ok();
    let priority = std::thread::current().get_priority().unwrap().to_posix(ThreadSchedulePolicy::Normal(NormalThreadSchedulePolicy::Other)).unwrap();
    println!("Thread {threadid} priority {priority}");
    //let mut rng = ChaCha20Rng::from_seed(seed);

    // ----------------
    //  World / Camera
    // ----------------
    //let mut mats = MatManager::new();
    //let mut world = HittableList { objs: vec![] };
    //let mut cam = Camera::default(&mut rng);
    //let mut aspect_ratio: f32 = 0.0;
    //#[allow(unused_mut)]
    //scene.setup(&mut world, &mut cam, &mut mats, &mut aspect_ratio, &mut rng);
    //let y_iter = split_vec_into_n_groups_and_get_n((0 as u32..target_height as u32).collect::<Vec<u32>>(), num_cpus::get(), thread_num);
    //let y_offset = y_iter[0].clone();
    let mut seed1: <ChaCha20Rng as SeedableRng>::Seed = Default::default();
    thread_rng().fill(&mut seed1);
    let mut rng = ChaCha20Rng::from_seed(seed1);

    let mut ims = vec![];

    loop {
        // -------
        //  Y val
        // -------
        let y = y_counter.inc() as u32;
        if y >= target_height {
            break;
        }

        // ---------------
        //  Output Buffer
        // ---------------
        //let mut im = RgbImage::new(target_width, /*target_height*/ 1);
        let mut im = vec![];

        // ------------------
        //  RT Without the X
        // ------------------
        let scale = 1.0 / samples_per_pixel as f32;
        for x in 0..target_width {
            let mut pix_color = Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            }
            .to_simd4();
            for _sample in 0..samples_per_pixel {
                let u = (x as f32 + rand_double(&mut rng)) / (target_width - 1) as f32;
                let v = 1.0 - ((y as f32 + rand_double(&mut rng)) / (target_height - 1) as f32);
                let r = cam.get_ray(u, v, &mut rng);
                let c = ray_color(&r, &world, &mut rng, &mats, max_depth as u64);
                pix_color += c;
            }
            //im.put_pixel(
            //    x,
            //    0,
            //    Rgb([
            //        ((pix_color.r * scale).sqrt() * 255.0) as u8,
            //        ((pix_color.g * scale).sqrt() * 255.0) as u8,
            //        ((pix_color.b * scale).sqrt() * 255.0) as u8,
            //    ]),
            //);

            //im.push(((pix_color.r * scale).sqrt() * 255.0) as u8);
            //im.push(((pix_color.g * scale).sqrt() * 255.0) as u8);
            //im.push(((pix_color.b * scale).sqrt() * 255.0) as u8);
            pix_color *= f32x4::from_array([scale, scale, scale, 0.0]);
            pix_color = pix_color.sqrt() * f32x4::from_array([255.0, 255.0, 255.0, 0.0]);
            im.append(pix_color.to_array()[0..3].to_vec().as_mut());
            //im.push(pix_color[0] as u8);
            //im.push(pix_color[1] as u8);
            //im.push(pix_color[2] as u8);
        }
        bar.inc(1);

        ims.push(RtRet {
            im: im.iter().map(|&e| e as u8).collect(),
            i_id: y as usize,
        });
    }

    //let mut raw_out = im.into_raw();
    //raw_out.shrink_to_fit();
    return ims;
}
