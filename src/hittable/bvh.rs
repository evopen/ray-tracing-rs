use std::cmp::Ordering;
use std::sync::Arc;

use super::AABB;
use crate::hittable::Hittable;
use crate::utils;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    root: AABB,
}

#[inline(always)]
fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0).unwrap();
    let box_b = b.bounding_box(0.0, 0.0).unwrap();

    if box_a.min()[axis] < box_b.min()[axis] {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

#[inline(always)]
fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}
#[inline(always)]
fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}
#[inline(always)]
fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

impl BVHNode {
    pub fn new(objects: &[Arc<dyn Hittable>], time_0: crate::Float, time_1: crate::Float) -> Self {
        let mut objects = objects.to_vec();
        let axis = utils::gen_range(0..=2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!(),
        };

        let left;
        let right;

        if objects.len() == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if objects.len() == 2 {
            if comparator(&objects[0], &objects[1]) == Ordering::Less {
                left = objects[0].clone();
                right = objects[1].clone();
            } else {
                left = objects[1].clone();
                right = objects[0].clone();
            }
        } else {
            objects.sort_unstable_by(comparator);
            let mid = objects.len() / 2;
            left = Arc::new(BVHNode::new(&objects[0..mid], time_0, time_1));
            right = Arc::new(BVHNode::new(&objects[mid..], time_0, time_1));
        }
        let box_left = left.bounding_box(time_0, time_1).unwrap();
        let box_right = right.bounding_box(time_0, time_1).unwrap();
        let root = box_left.surrounding_box(&box_right);

        Self { left, right, root }
    }
}

impl Hittable for BVHNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.root.hit(r, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(
            r,
            t_min,
            match &hit_left {
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

    fn bounding_box(&self, _time_0: crate::Float, _time_1: crate::Float) -> Option<AABB> {
        Some(self.root.clone())
    }
}
