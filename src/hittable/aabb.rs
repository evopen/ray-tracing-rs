use crate::ray::Ray;
use crate::types::Point3;

#[derive(Clone)]
pub struct Aabb {
    minimum: Point3,
    maximum: Point3,
}

impl Aabb {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: crate::Float, t_max: crate::Float) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for i in 0..3 {
            let inv_d = 1.0 / r.direction()[i];
            let mut t0 = (self.minimum[i] - r.origin()[i]) * inv_d;
            let mut t1 = (self.maximum[i] - r.origin()[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(&self, other: &Aabb) -> Aabb {
        let small = Point3::new(
            self.min().x.min(other.min().x),
            self.min().y.min(other.min().y),
            self.min().z.min(other.min().z),
        );

        let big = Point3::new(
            self.max().x.max(other.max().x),
            self.max().y.max(other.max().y),
            self.max().z.max(other.max().z),
        );

        Aabb::new(small, big)
    }
}
