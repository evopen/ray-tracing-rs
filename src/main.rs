// based on ray tracing in one weekend 3.2.3

mod color;
mod vec3;
use color::Color;
use vec3::{Point3, Vec3};

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    let mut image_buffer = image::RgbImage::new(image_width, image_height);

    for j in (0..image_height).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );
            color::write_color(&mut &mut image_buffer, i, j, &color);
        }
    }
    println!("Done");
    image_buffer.save("result.png").unwrap();
}
