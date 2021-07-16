use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    root: AABB,
}

impl BVHNode {
    pub fn new() -> Self {
        todo!()
    }
}

impl Hittable for BVHNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.root.hit(r, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(
            r,
            t_min,
            match hit_left {
                Some(rec) => rec.t,
                None => t_max,
            },
        );

        if hit_right.is_some() {
            return hit_right;
        } else if hit_left.is_some() {
            return hit_left;
        } else {
            return None;
        }
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<crate::aabb::AABB> {
        Some(self.root.clone())
    }
}
