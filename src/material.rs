use std::sync::Arc;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::{self, Ray};
use crate::texture::{SolidColor, Texture};
use crate::types::Point3;
use crate::utils::rand_vec3_in_unit_sphere;
use crate::{types, utils};

pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material: Sync + Send {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter>;

    fn emitted(&self, u: crate::Float, v: crate::Float, p: Point3) -> Color {
        return Color::splat(0.0);
    }
}

pub struct Lambertian {
    base_color: Arc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = rec.normal + utils::rand_vec3_unit();
        if types::is_near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        Some(Scatter {
            attenuation: self.base_color.value(rec.u, rec.v, rec.p),
            ray: Ray::new_with_time(rec.p, scatter_direction, r.time()),
        })
    }
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self {
            base_color: texture,
        }
    }

    pub fn new_with_color(color: Color) -> Self {
        Self {
            base_color: Arc::new(SolidColor::new(color)),
        }
    }
}

pub struct Metal {
    base_color: Color,
    fuzz: crate::Float,
}

impl Metal {
    pub fn new(base_color: Color, fuzz: crate::Float) -> Self {
        Self {
            base_color,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = ray::reflect(r.direction().normalize(), rec.normal);
        let scattered = Ray::new_with_time(
            rec.p,
            reflected + self.fuzz * rand_vec3_in_unit_sphere(),
            r.time(),
        );

        if scattered.direction().dot(rec.normal) < 0.0 {
            return None;
        } else {
            return Some(Scatter {
                attenuation: self.base_color,
                ray: scattered,
            });
        }
    }
}

pub struct Dielectric {
    pub ir: crate::Float, // index of refraction
}

impl Dielectric {
    pub fn new(ir: crate::Float) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: crate::Float, ref_idx: crate::Float) -> crate::Float {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r.direction().normalize();
        let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let dir_out = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > utils::gen_float()
        {
            ray::reflect(unit_direction, rec.normal)
        } else {
            ray::refract(r.direction().normalize(), rec.normal, refraction_ratio)
        };

        let scatter = Scatter {
            attenuation: Color::splat(1.0),
            ray: Ray::new_with_time(rec.p, dir_out, r.time()),
        };
        Some(scatter)
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { emit: texture }
    }

    pub fn new_with_color(color: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r: &Ray, _rec: &HitRecord) -> Option<Scatter> {
        return None;
    }

    fn emitted(&self, u: crate::Float, v: crate::Float, p: Point3) -> Color {
        return self.emit.value(u, v, p);
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
    pub fn new_with_color(color: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(color)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let scattered = Ray::new_with_time(rec.p, utils::rand_vec3_in_unit_sphere(), r.time());
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some(Scatter {
            attenuation,
            ray: scattered,
        })
    }
}
