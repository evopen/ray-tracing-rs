use crate::ray::Ray;
use crate::vec3::Point3;

struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
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

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for i in 0..3 {
            let t0 = ((self.minimum[i] - r.origin()[i]) / r.direction()[i])
                .min((self.maximum[i] - r.origin()[i]) / r.direction()[i]);
            let t1 = ((self.minimum[i] - r.origin()[i]) / r.direction()[i])
                .max((self.maximum[i] - r.origin()[i]) / r.direction()[i]);
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
