use crate::Vec3;

pub type Color = Vec3;

pub fn write_color(img: &mut image::RgbImage, x: u32, y: u32, color: &Color) {
    let ir = (255.999 * color.x) as u8;
    let ig = (255.999 * color.y) as u8;
    let ib = (255.999 * color.z) as u8;
    img.put_pixel(x, y, image::Rgb([ir, ig, ib]));
}
