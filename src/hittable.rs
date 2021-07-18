use std::sync::Arc;

use crate::aabb::AABB;
use crate::material::Material;
use crate::Point3;
use crate::Ray;
use crate::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: &Point3, normal: &Vec3, t: f64, material: &Arc<dyn Material>) -> Self {
        Self {
            p: p.clone(),
            normal: normal.clone(),
            t,
            front_face: false,
            material: material.clone(),
            u: 0.0,
            v: 0.0,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB>;
}

struct Translate {
    hittable: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction());
        self.hittable.hit(&moved_r, t_min, t_max).map(|mut rec| {
            rec.p += self.offset;
            (rec.front_face, rec.normal) = crate::ray::faceforward(r.direction(), rec.normal);
            rec
        })
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        self.hittable
            .bounding_box(time_0, time_1)
            .map(|bb| AABB::new(bb.min() + self.offset, bb.max() + self.offset))
    }
}
