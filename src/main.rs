// based on ray tracing in one weekend 3.2.3

#![feature(destructuring_assignment)]

mod aabb;
mod bvh;
mod camera;
mod cli;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod utils;
mod vec3;

use std::io::{stdout, Write};
use std::sync::Arc;
use std::time::Duration;

use color::Color;
use ray::Ray;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use texture::{CheckerTexture, NoiseTexture};
use vec3::{Point3, Vec3};

use aabb::AABB;
use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use moving_sphere::MovingSphere;
use sphere::Sphere;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_with_color(
        Color::new(0.2, 0.3, 0.1),
        Color::splat(0.9),
    ));
    let ground_material = Arc::new(material::Lambertian::new(checker));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::rand_f64();
            let center = Point3::new(
                a as f64 + 0.9 * utils::rand_f64(),
                0.2,
                b as f64 + 0.9 * utils::rand_f64(),
            );

            if choose_mat < 0.7 {
                // diffuse
                let albedo = utils::rand_vec3() * utils::rand_vec3();
                let sphere_material = Arc::new(material::Lambertian::new_with_color(albedo));
                let center_1 = center + Vec3::new(0.0, utils::rand_f64_range(0.0, 0.5), 0.0);
                world.add(Arc::new(MovingSphere::new(
                    center,
                    center_1,
                    0.0,
                    1.0,
                    0.2,
                    sphere_material,
                )));
            } else if choose_mat < 0.85 {
                // metal
                let albedo = utils::rand_vec3_range(0.5, 1.0);
                let fuzz = utils::rand_f64_range(0.0, 0.5);
                let sphere_material = Arc::new(material::Metal::new(albedo, fuzz));
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            } else {
                let sphere_material = Arc::new(material::Dielectric::new(1.5));
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(material::Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(material::Lambertian::new_with_color(Color::new(
        0.4, 0.2, 0.1,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = Arc::new(CheckerTexture::new_with_color(
        Color::new(0.2, 0.3, 0.1),
        Color::splat(0.9),
    ));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(material::Lambertian::new(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(material::Lambertian::new(checker.clone())),
    )));
    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(material::Lambertian::new(perlin_texture.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(material::Lambertian::new(perlin_texture.clone())),
    )));
    objects
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::splat(0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, std::f64::INFINITY) {
        if let Some(scatter) = rec.material.scatter(r, &rec) {
            return scatter.attenuation * ray_color(&scatter.ray, world, depth - 1);
        } else {
            return Color::splat(0.0);
        }
    }

    let unit_direction = r.direction().normalize();
    let t = (unit_direction.y + 1.0) * 0.5;
    Color::splat(1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Parameters
    let matches = cli::build_app().get_matches();
    let scene = matches.value_of("scene").unwrap().parse::<u32>().unwrap();
    let use_bvh = matches.is_present("use bvh");

    // Image
    let aspect_ratio = matches
        .value_of("aspect ratio")
        .map(|s| {
            let (a, b) = s.split_once(':').unwrap();
            a.parse::<f64>().unwrap() / b.parse::<f64>().unwrap()
        })
        .unwrap();
    let image_width = matches.value_of("width").unwrap().parse().unwrap();
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = matches
        .value_of("samples per pixel")
        .unwrap()
        .parse()
        .unwrap();
    let max_depth = 50;

    // World
    let hittable_list;
    let lookfrom;
    let lookat;
    let mut vfov = 40.0;
    let mut aperture = 0.0;

    match scene {
        1 => {
            hittable_list = random_scene();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::splat(0.0);
            vfov = 20.0;
        }
        2 => {
            hittable_list = two_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::splat(0.0);
            vfov = 20.0;
        }
        3 | _ => {
            hittable_list = two_perlin_spheres();
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::splat(0.0);
            vfov = 20.0;
        }
    }
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
                    let u = (x as f64 + utils::rand_f64()) / (image_width - 1) as f64;
                    let v = (y as f64 + utils::rand_f64()) / (image_height - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, world.as_ref(), max_depth);
                }
                tx.send(((x, y), pixel_color)).unwrap();
            });
    });

    loop {
        std::thread::sleep(Duration::from_millis(500));
        let jobs_done = rx.len();
        print!(
            "\rCompleted {:.1}%     ",
            jobs_done as f64 / (image_width * image_height) as f64 * 100.0
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

    println!("\nDone, took {} seconds", start_time.elapsed().as_secs());
    image_buffer.save("result.png").unwrap();
}
