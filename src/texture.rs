use std::sync::Arc;

use image::GenericImageView;

use crate::color::Color;
use crate::vec3::Point3;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        return self.color;
    }
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn new_with_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Color::new(r, g, b),
        }
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new_with_color(odd: Color, even: Color) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(odd)),
            even: Arc::new(SolidColor::new(even)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

#[derive(Default)]
pub struct NoiseTexture {
    noise: super::perlin::Perlin,
    scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::splat(1.0) * 0.5 * (1.0 + (10.0 * self.noise.turb(p, 7) + self.scale * p.z).sin())
    }
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }
}

pub struct ImageTexture {
    data: image::RgbImage,
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let width = self.data.width();
        let height = self.data.height();

        let i = (u * width as f64) as u32;
        let j = (v * height as f64) as u32;

        assert!(i <= width);
        assert!(j <= height);

        let color = self.data.get_pixel(i, j);
        let color_scale = 1.0 / 255.0;

        Color::new(
            color[0] as f64 * color_scale,
            color[1] as f64 * color_scale,
            color[2] as f64 * color_scale,
        )
    }
}
