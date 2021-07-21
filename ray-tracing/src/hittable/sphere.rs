use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::material::Material;
use crate::types::Point3;
use crate::Vec3;

use crate::types::PI;

use super::aabb::AABB;

pub struct Sphere {
    pub center: Point3,
    pub radius: crate::Float,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, r: crate::Float, material: Arc<dyn Material>) -> Self {
        Self {
            center: center.to_owned(),
            radius: r,
            material: material.clone(),
        }
    }

    fn get_uv(p: Point3) -> (crate::Float, crate::Float) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl crate::hittable::Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        let mut rec = HitRecord::new(&p, &outward_normal, t, &self.material);
        rec.set_face_normal(r, outward_normal);
        (rec.u, rec.v) = Self::get_uv(rec.normal);

        return Some(rec);
    }

    fn bounding_box(&self, _time_0: crate::Float, _time_1: crate::Float) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::splat(self.radius),
            self.center + Vec3::splat(self.radius),
        ))
    }
}
