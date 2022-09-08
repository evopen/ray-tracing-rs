// based on ray tracing in one weekend 3.2.3
#![allow(
    dead_code,
    unused,
    clippy::needless_return,
    clippy::redundant_clone,
    clippy::needless_range_loop,
    clippy::wildcard_in_or_patterns
)]

mod camera;
mod cli;
mod color;
mod hittable;
mod material;
mod perlin;
mod ray;
mod scene;
mod texture;
mod types;
mod utils;

use std::io::{stdout, Write};
use std::time::Duration;

use color::Color;
use ray::Ray;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use types::{Float, Point3, Vec3};

use camera::Camera;
use hittable::Hittable;

fn ray_color(r: &Ray, background: Color, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::splat(0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, crate::Float::INFINITY) {
        let emitted = rec.material.emitted(rec.u, rec.v, rec.p);
        if let Some(scatter) = rec.material.scatter(r, &rec) {
            return emitted
                + scatter.attenuation * ray_color(&scatter.ray, background, world, depth - 1);
        } else {
            return emitted;
        }
    } else {
        return background;
    }
}

fn main() {
    // Parameters
    let matches = cli::build_app().get_matches();
    let scene = matches.get_one::<u32>("scene").unwrap();
    let use_bvh = matches.is_present("use bvh");
    let threads = matches
        .get_one::<usize>("job")
        .copied()
        .unwrap_or_else(num_cpus::get);

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    // Image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 400;
    let mut samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let hittable_list;
    let lookfrom;
    let lookat;
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut background = Color::splat(0.0);

    match scene {
        1 => {
            hittable_list = scene::random_scene();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::splat(0.0);
            vfov = 20.0;
        }
        2 => {
            hittable_list = scene::two_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::splat(0.0);
            vfov = 20.0;
        }
        3 => {
            hittable_list = scene::two_perlin_spheres();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::splat(0.0);
            vfov = 20.0;
        }
        4 => {
            hittable_list = scene::earth();
            background = Color::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::splat(0.0);
            vfov = 20.0;
        }
        5 => {
            hittable_list = scene::simple_light();
            samples_per_pixel = 400;
            background = Color::splat(0.0);
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            hittable_list = scene::cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 400;
            background = Color::splat(0.0);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            hittable_list = scene::cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::splat(0.0);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        8 | _ => {
            hittable_list = scene::final_scene();
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 10000;
            background = Color::splat(0.0);
            lookfrom = Point3::new(478.0, 278.0, -600.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }

    if let Some(samples) = matches.get_one::<u32>("samples per pixel") {
        samples_per_pixel = *samples;
    }
    if let Some(width) = matches.get_one::<u32>("width") {
        image_width = *width;
    }
    if let Some((w, h)) = matches.get_one::<(crate::Float, crate::Float)>("aspect ratio") {
        aspect_ratio = w / h;
    }

    let image_height = (image_width as crate::Float / aspect_ratio) as u32;
    let bvh = hittable_list.build_bvh(0.0, 1.0);

    let world: Box<dyn Hittable> = match use_bvh {
        true => Box::new(bvh),
        false => Box::new(hittable_list),
    };

    // Camera
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new_with_time_range(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // Render
    let start_time = std::time::Instant::now();

    let mut image_buffer = image::RgbImage::new(image_width, image_height);

    let (tx, rx) = crossbeam::channel::bounded((image_width * image_height) as usize);

    let worker_thread = std::thread::spawn(move || {
        println!("job started");
        let mut job_indices = Vec::with_capacity((image_width * image_height) as usize);
        for x in 0..image_width {
            for y in 0..image_height {
                job_indices.push((x, y));
            }
        }
        job_indices
            .into_par_iter()
            .for_each_with(tx.clone(), |tx, (x, y)| {
                let mut pixel_color = Color::splat(0.0);
                for _ in 0..samples_per_pixel {
                    let u = (x as crate::Float + utils::gen_float())
                        / (image_width - 1) as crate::Float;
                    let v = (y as crate::Float + utils::gen_float())
                        / (image_height - 1) as crate::Float;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, background, world.as_ref(), max_depth);
                }
                tx.send(((x, y), pixel_color)).unwrap();
            });
    });

    loop {
        std::thread::sleep(Duration::from_millis(50));
        let jobs_done = rx.len();
        print!(
            "\rCompleted {:.1}%     ",
            jobs_done as crate::Float / (image_width * image_height) as crate::Float * 100.0
        );
        stdout().flush().unwrap();

        if rx.is_full() {
            worker_thread.join().unwrap();
            break;
        }
    }

    while let Ok(((x, y), color)) = rx.recv() {
        color::write_color(&mut image_buffer, x, y, color, samples_per_pixel);
    }

    println!(
        "\nDone, took {:.1} seconds",
        start_time.elapsed().as_secs_f32()
    );
    image_buffer.save("result.png").unwrap();
}
