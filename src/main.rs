// based on ray tracing in one weekend 3.2.3

mod color;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
}

fn ray_color(r: &Ray) -> Color {
    if let Some(t) = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    } else {
        let unit_direction = r.direction().normalize();
        let t = (unit_direction.y + 1.0) * 0.5;
        Color::splat(1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - Vec3::new(0.0, 0.0, focal_length) - horizontal / 2.0 - vertical / 2.0;

    // Render
    let mut image_buffer = image::RgbImage::new(image_width, image_height);

    for y in (0..image_height).rev() {
        println!("\rScanlines remaining: {}", y);
        let v = y as f64 / (image_height - 1) as f64;
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical - origin),
            );
            let pixel_color = ray_color(&r);
            color::write_color(&mut &mut image_buffer, x, y, &pixel_color);
        }
    }
    println!("Done");
    image_buffer.save("result.png").unwrap();
}
