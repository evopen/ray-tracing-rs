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
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(material::Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Lambertian::new(&Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(material::Metal::new(&Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(material::Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        &Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        &Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        &Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        &Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

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
                pixel_color += ray_color(&r, &world, max_depth);
            }
            color::write_color(&mut image_buffer, x, y, &pixel_color, samples_per_pixel);
        }
    }
    println!("Done");
    image_buffer.save("result.png").unwrap();
}
