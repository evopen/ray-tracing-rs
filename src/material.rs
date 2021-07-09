use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::{self, Ray};
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
    pub fn new(base_color: Color) -> Self {
        Self {
            base_color: base_color,
        }
    }
}

pub struct Metal {
    base_color: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(base_color: Color, fuzz: f64) -> Self {
        Self {
            base_color: base_color,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = ray::reflect(r.direction().normalize(), rec.normal);
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

pub struct Dielectric {
    pub ir: f64, // index of refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let refracted = ray::refract(r.direction().normalize(), rec.normal, refraction_ratio);

        let scatter = Scatter {
            attenuation: Color::splat(1.0),
            ray: Ray::new(rec.p, refracted),
        };
        Some(scatter)
    }
}
