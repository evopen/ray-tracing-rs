// based on ray tracing in one weekend 3.2.3

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::rc::Rc;

use color::Color;
use hittable::HitRecord;
use ray::Ray;
use vec3::{Point3, Vec3};

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, std::f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + utils::rand_vec3_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(&rec.p, &(target - rec.p)), world);
    }

    let unit_direction = r.direction().normalize();
    let t = (unit_direction.y + 1.0) * 0.5;
    Color::splat(1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - Vec3::new(0.0, 0.0, focal_length) - horizontal / 2.0 - vertical / 2.0;

    // Camera
    let cam = Camera::default();

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
                pixel_color += ray_color(&r, &world);
            }
            color::write_color(&mut image_buffer, x, y, &pixel_color, samples_per_pixel);
        }
    }
    println!("Done");
    image_buffer.save("result.png").unwrap();
}
