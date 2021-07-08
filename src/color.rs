use crate::Vec3;

pub type Color = Vec3;

pub fn write_color(
    img: &mut image::RgbImage,
    x: u32,
    y: u32,
    color: &Color,
    samples_per_pixel: u32,
) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;
    let ir = (256.0 * r.clamp(0.0, 0.999)) as u8;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as u8;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as u8;

    img.put_pixel(x, img.height() - 1 - y, image::Rgb([ir, ig, ib]));
}
