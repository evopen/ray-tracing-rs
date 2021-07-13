// based on ray tracing in one weekend 3.2.3

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::rc::Rc;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(material::Lambertian::new(Color::splat(0.5)));
    world.add(Rc::new(Sphere::new(
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

            if choose_mat < 0.8 {
                // diffuse
                let albedo = utils::rand_vec3() * utils::rand_vec3();
                let sphere_material = Rc::new(material::Lambertian::new(albedo));
                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            } else if choose_mat < 0.95 {
                // metal
                let albedo = utils::rand_vec3_range(0.5, 1.0);
                let fuzz = utils::rand_f64_range(0.0, 0.5);
                let sphere_material = Rc::new(material::Metal::new(albedo, fuzz));
                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            } else {
                let sphere_material = Rc::new(material::Dielectric::new(1.5));
                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(material::Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(material::Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
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
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::splat(0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    let mut image_buffer = image::RgbImage::new(image_width, image_height);

    for y in (0..image_height).rev() {
        println!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let mut pixel_color = Color::splat(0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f64 + utils::rand_f64()) / (image_width - 1) as f64;
                let v = (y as f64 + utils::rand_f64()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            color::write_color(&mut image_buffer, x, y, &pixel_color, samples_per_pixel);
        }
    }
    println!("Done");
    image_buffer.save("result.png").unwrap();
}
