use std::sync::Arc;

use super::HittableList;
use super::{XYRect, XZRect, YZRect};
use crate::hittable::Hittable;
use crate::material::Material;
use crate::types::Point3;

use super::aabb::Aabb;

pub struct Box {
    p0: Point3,
    p1: Point3,
    sides: HittableList,
}

impl Box {
    pub fn new(p0: Point3, p1: Point3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            material.clone(),
        )));
        sides.add(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            material.clone(),
        )));
        sides.add(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            material.clone(),
        )));
        sides.add(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            material.clone(),
        )));
        sides.add(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            material.clone(),
        )));
        sides.add(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            material.clone(),
        )));

        Self { p0, p1, sides }
    }
}

impl Hittable for Box {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<crate::hittable::HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, time_0: crate::Float, time_1: crate::Float) -> Option<Aabb> {
        self.sides.bounding_box(time_0, time_1)
    }
}
