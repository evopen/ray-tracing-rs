use std::sync::Arc;

use crate::color::Color;
use crate::vec3::Point3;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
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
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        Color::splat(1.0) * self.noise.noise(p * self.scale)
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
