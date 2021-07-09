use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::rand_vec3_in_unit_sphere;
use crate::Vec3;
use crate::{utils, vec3};

pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    base_color: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = rec.normal + utils::rand_vec3_unit();
        if vec3::is_near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        Some(Scatter {
            attenuation: self.base_color,
            ray: Ray::new(rec.p, scatter_direction),
        })
    }
}

impl Lambertian {
    pub fn new(base_color: &Color) -> Self {
        Self {
            base_color: base_color.to_owned(),
        }
    }
}

pub struct Metal {
    base_color: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(base_color: &Color, fuzz: f64) -> Self {
        Self {
            base_color: base_color.to_owned(),
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = vec3::reflect(r.direction().normalize(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * rand_vec3_in_unit_sphere());

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
