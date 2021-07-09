use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::Vec3;
use crate::{utils, vec3};

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray);
}

pub struct Lambertian {
    base_color: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) {
        let mut scatter_direction = rec.normal + utils::rand_vec3_unit();
        if vec3::is_near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.base_color;
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
}

impl Metal {
    pub fn new(base_color: &Color) -> Self {
        Self {
            base_color: base_color.to_owned(),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) {
        let reflected = vec3::reflect(r.direction().normalize(), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.base_color;
        assert!(scattered.direction().dot(rec.normal) > 0.0);
    }
}
