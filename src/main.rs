// based on ray tracing in one weekend 3.2.3

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    let mut image_buffer = image::RgbImage::new(image_width, image_height);

    for j in (0..image_height).rev() {
        println!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;
            image_buffer.put_pixel(i, j, image::Rgb([ir, ig, ib]));
        }
    }
    println!("Done");
    image_buffer.save("result.png").unwrap();
}
