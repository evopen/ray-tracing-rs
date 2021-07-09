use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

trait Material {
    fn scatter(r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray);
}
