use std::path::Path;
use std::sync::Arc;

use crate::color::Color;
use crate::types::Point3;

pub trait Texture: Sync + Send {
    fn value(&self, u: crate::Float, v: crate::Float, p: Point3) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: crate::Float, _v: crate::Float, _p: Point3) -> Color {
        return self.color;
    }
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn new_with_rgb(r: crate::Float, g: crate::Float, b: crate::Float) -> Self {
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
    fn value(&self, u: crate::Float, v: crate::Float, p: Point3) -> Color {
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
    scale: crate::Float,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: crate::Float, _v: crate::Float, p: Point3) -> Color {
        Color::splat(1.0) * 0.5 * (1.0 + (10.0 * self.noise.turb(p, 7) + self.scale * p.z).sin())
    }
}

impl NoiseTexture {
    pub fn new(scale: crate::Float) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }
}

pub struct ImageTexture {
    data: image::RgbImage,
}

impl ImageTexture {
    pub fn new(p: impl AsRef<Path>) -> Self {
        Self {
            data: image::open(p).unwrap().into_rgb8(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: crate::Float, v: crate::Float, _p: Point3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let width = self.data.width();
        let height = self.data.height();

        let i = (u * width as crate::Float) as u32;
        let j = (v * height as crate::Float) as u32;

        assert!(i <= width);
        assert!(j <= height);

        let color = self.data.get_pixel(i, j);
        let color_scale = 1.0 / 255.0;

        Color::new(
            color[0] as crate::Float * color_scale,
            color[1] as crate::Float * color_scale,
            color[2] as crate::Float * color_scale,
        )
    }
}
