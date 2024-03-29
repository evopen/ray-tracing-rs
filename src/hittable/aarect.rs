use std::sync::Arc;

use super::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::types::Point3;
use crate::Vec3;

const AABB_EPSILON: crate::Float = 0.0001;

pub struct XYRect {
    x0: crate::Float,
    x1: crate::Float,
    y0: crate::Float,
    y1: crate::Float,
    k: crate::Float,
    material: Arc<dyn Material>,
}

pub struct XZRect {
    x0: crate::Float,
    x1: crate::Float,
    z0: crate::Float,
    z1: crate::Float,
    k: crate::Float,
    material: Arc<dyn Material>,
}

pub struct YZRect {
    y0: crate::Float,
    y1: crate::Float,
    z0: crate::Float,
    z1: crate::Float,
    k: crate::Float,
    material: Arc<dyn Material>,
}

impl XYRect {
    pub fn new(
        x0: crate::Float,
        x1: crate::Float,
        y0: crate::Float,
        y1: crate::Float,
        k: crate::Float,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl XZRect {
    pub fn new(
        x0: crate::Float,
        x1: crate::Float,
        z0: crate::Float,
        z1: crate::Float,
        k: crate::Float,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl YZRect {
    pub fn new(
        y0: crate::Float,
        y1: crate::Float,
        z0: crate::Float,
        z1: crate::Float,
        k: crate::Float,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hittable for XYRect {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<crate::hittable::HitRecord> {
        let t = (self.k - r.origin().z) / r.direction().z;
        if t < t_min || t_max < t {
            return None;
        }
        let x = r.origin().x + t * r.direction().x;
        let y = r.origin().y + t * r.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = HitRecord {
            p: r.at(t),
            normal: Vec3::default(),
            material: self.material.clone(),
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            front_face: false,
        };
        rec.set_face_normal(r, Vec3::new(0.0, 0.0, 1.0));
        Some(rec)
    }

    fn bounding_box(&self, _time_0: crate::Float, _time_1: crate::Float) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.y0, self.k - AABB_EPSILON),
            Point3::new(self.x1, self.y1, self.k + AABB_EPSILON),
        ))
    }
}

impl Hittable for XZRect {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<crate::hittable::HitRecord> {
        let t = (self.k - r.origin().y) / r.direction().y;
        if t < t_min || t_max < t {
            return None;
        }
        let x = r.origin().x + t * r.direction().x;
        let z = r.origin().z + t * r.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord {
            p: r.at(t),
            normal: Vec3::default(),
            material: self.material.clone(),
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            front_face: false,
        };
        rec.set_face_normal(r, Vec3::new(0.0, 1.0, 0.0));
        Some(rec)
    }

    fn bounding_box(&self, _time_0: crate::Float, _time_1: crate::Float) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.k - AABB_EPSILON, self.z0),
            Point3::new(self.x1, self.k + AABB_EPSILON, self.z1),
        ))
    }
}

impl Hittable for YZRect {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<crate::hittable::HitRecord> {
        let t = (self.k - r.origin().x) / r.direction().x;
        if t < t_min || t_max < t {
            return None;
        }
        let y = r.origin().y + t * r.direction().y;
        let z = r.origin().z + t * r.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord {
            p: r.at(t),
            normal: Vec3::default(),
            material: self.material.clone(),
            t,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            front_face: false,
        };
        rec.set_face_normal(r, Vec3::new(1.0, 0.0, 0.0));
        Some(rec)
    }

    fn bounding_box(&self, _time_0: crate::Float, _time_1: crate::Float) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.k - AABB_EPSILON, self.y0, self.z0),
            Point3::new(self.k + AABB_EPSILON, self.y1, self.z1),
        ))
    }
}
