mod aabb;
mod aarect;
mod r#box;
mod bvh;
mod constant_medium;
mod hittable_list;
mod moving_sphere;
mod sphere;

pub use aabb::AABB;
pub use aarect::{XYRect, XZRect, YZRect};
pub use bvh::BVHNode;
pub use hittable_list::HittableList;
pub use moving_sphere::MovingSphere;
pub use r#box::Box;
pub use sphere::Sphere;
pub use constant_medium::ConstantMedium;

use std::sync::Arc;

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

pub struct Translate {
    hittable: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(hittable: Arc<dyn Hittable>, translate: Vec3) -> Self {
        Self {
            hittable,
            offset: translate,
        }
    }
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

pub struct RotateY {
    hittable: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<AABB>,
}

impl RotateY {
    /// angle in degrees
    pub fn new(hittable: Arc<dyn Hittable>, angle: f64) -> Self {
        let angle = angle.to_radians();
        let sin_theta = angle.sin();
        let cos_theta = angle.cos();
        let bounding_box = hittable.bounding_box(0.0, 1.0).map(|bb| {
            let mut min = Point3::splat(f64::INFINITY);
            let mut max = Point3::splat(f64::NEG_INFINITY);

            for i in 0..=1 {
                for j in 0..=1 {
                    for k in 0..=1 {
                        let x = i as f64 * bb.max().x + (1 - i) as f64 * bb.min().x;
                        let y = j as f64 * bb.max().y + (1 - j) as f64 * bb.min().y;
                        let z = k as f64 * bb.max().z + (1 - k) as f64 * bb.min().z;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let new_point = Point3::new(new_x, y, new_z);

                        for d in 0..3 {
                            min[d] = min[d].min(new_point[d]);
                            max[d] = max[d].max(new_point[d]);
                        }
                    }
                }
            }
            AABB::new(min, max)
        });
        Self {
            hittable,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = Point3::new(
            self.cos_theta * r.origin().x - self.sin_theta * r.origin().z,
            r.origin().y,
            self.sin_theta * r.origin().x + self.cos_theta * r.origin().z,
        );
        let direction = Vec3::new(
            self.cos_theta * r.direction().x - self.sin_theta * r.direction().z,
            r.direction().y,
            self.sin_theta * r.direction().x + self.cos_theta * r.direction().z,
        );
        let rotated_r = Ray::new_with_time(origin, direction, r.time());
        self.hittable.hit(&rotated_r, t_min, t_max).map(|mut rec| {
            let mut p = rec.p;
            let mut normal = rec.normal;
            p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
            p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;
            normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;
            rec.p = p;
            rec.set_face_normal(&rotated_r, normal);

            rec
        })
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<AABB> {
        self.bounding_box.clone()
    }
}
