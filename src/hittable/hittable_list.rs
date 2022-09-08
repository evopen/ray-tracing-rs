use std::sync::Arc;

use super::{Aabb, BVHNode};
use crate::hittable::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn build_bvh(&self, time_0: crate::Float, time_1: crate::Float) -> BVHNode {
        BVHNode::new(&self.objects, time_0, time_1)
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut result = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }

        return result;
    }

    fn bounding_box(&self, time_0: crate::Float, time_1: crate::Float) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        };
        let mut object_iter = self.objects.iter();
        let mut bounding_box = object_iter.next().unwrap().bounding_box(time_0, time_1)?;
        for object in object_iter {
            if let Some(b) = object.bounding_box(time_0, time_1) {
                bounding_box = b.surrounding_box(&bounding_box);
            } else {
                return None;
            }
        }
        Some(bounding_box)
    }
}
