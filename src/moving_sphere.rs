use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::material::Material;
use crate::vec3::Point3;
use crate::Hittable;

pub struct MovingSphere {
    center_0: Point3,
    center_1: Point3,
    time_0: f64,
    time_1: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center(r.time());

        let oc = r.origin() - center;
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
        let outward_normal = (p - center) / self.radius;

        let mut rec = HitRecord::new(&p, &outward_normal, t, &self.material);
        rec.set_face_normal(r, &outward_normal);

        return Some(rec);
    }
}

impl MovingSphere {
    pub fn new(
        center_0: Point3,
        center_1: Point3,
        time_0: f64,
        time_1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center_0,
            center_1,
            time_0,
            time_1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: f64) -> Point3 {
        self.center_0
            + ((time - self.time_0) / (self.time_1 - self.time_0)) * (self.center_1 - self.center_0)
    }
}
