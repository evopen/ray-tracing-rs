// based on ray tracing in one weekend 3.2.3

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for i in 0..image_height {
        for j in 0..image_width {
            let r = i as f64 / (image_height - 1) as f64;
            let g = j as f64 / (image_width - 1) as f64;
            let b = 0.25;
            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;
            print!("{} {} {}\t", ir, ig, ib);
        }
        println!();
    }
}
