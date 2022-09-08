mod aabb;
mod aarect;
mod r#box;
mod bvh;
mod constant_medium;
mod hittable_list;
mod moving_sphere;
mod sphere;

pub use aabb::Aabb;
pub use aarect::{XYRect, XZRect, YZRect};
pub use bvh::BVHNode;
pub use constant_medium::ConstantMedium;
pub use hittable_list::HittableList;
pub use moving_sphere::MovingSphere;
pub use r#box::Box;
pub use sphere::Sphere;

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
    pub t: crate::Float,
    pub u: crate::Float,
    pub v: crate::Float,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: &Point3, normal: &Vec3, t: crate::Float, material: &Arc<dyn Material>) -> Self {
        Self {
            p: *p,
            normal: *normal,
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
    fn hit(&self, r: &Ray, t_min: crate::Float, t_max: crate::Float) -> Option<HitRecord>;
    fn bounding_box(&self, time_0: crate::Float, time_1: crate::Float) -> Option<Aabb>;
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
    fn hit(&self, r: &Ray, t_min: crate::Float, t_max: crate::Float) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction());
        self.hittable.hit(&moved_r, t_min, t_max).map(|mut rec| {
            rec.p += self.offset;
            (rec.front_face, rec.normal) = crate::ray::faceforward(r.direction(), rec.normal);
            rec
        })
    }

    fn bounding_box(&self, time_0: crate::Float, time_1: crate::Float) -> Option<Aabb> {
        self.hittable
            .bounding_box(time_0, time_1)
            .map(|bb| Aabb::new(bb.min() + self.offset, bb.max() + self.offset))
    }
}

pub struct RotateY {
    hittable: Arc<dyn Hittable>,
    sin_theta: crate::Float,
    cos_theta: crate::Float,
    bounding_box: Option<Aabb>,
}

impl RotateY {
    /// angle in degrees
    pub fn new(hittable: Arc<dyn Hittable>, angle: crate::Float) -> Self {
        let angle = angle.to_radians();
        let sin_theta = angle.sin();
        let cos_theta = angle.cos();
        let bounding_box = hittable.bounding_box(0.0, 1.0).map(|bb| {
            let mut min = Point3::splat(crate::Float::INFINITY);
            let mut max = Point3::splat(crate::Float::NEG_INFINITY);

            for i in 0..=1 {
                for j in 0..=1 {
                    for k in 0..=1 {
                        let x =
                            i as crate::Float * bb.max().x + (1 - i) as crate::Float * bb.min().x;
                        let y =
                            j as crate::Float * bb.max().y + (1 - j) as crate::Float * bb.min().y;
                        let z =
                            k as crate::Float * bb.max().z + (1 - k) as crate::Float * bb.min().z;

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
            Aabb::new(min, max)
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
    fn hit(&self, r: &Ray, t_min: crate::Float, t_max: crate::Float) -> Option<HitRecord> {
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

    fn bounding_box(&self, _time_0: crate::Float, _time_1: crate::Float) -> Option<Aabb> {
        self.bounding_box.clone()
    }
}
