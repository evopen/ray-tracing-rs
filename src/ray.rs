use crate::Point3;
use crate::Vec3;

#[derive(Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    time: crate::Float,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin.to_owned(),
            dir: direction.to_owned(),
            time: 0.0,
        }
    }

    pub fn new_with_time(origin: Point3, direction: Vec3, time: crate::Float) -> Self {
        Self {
            orig: origin.to_owned(),
            dir: direction.to_owned(),
            time,
        }
    }

    pub fn time(&self) -> crate::Float {
        return self.time;
    }

    pub fn origin(&self) -> Point3 {
        self.orig.to_owned()
    }

    pub fn direction(&self) -> Vec3 {
        self.dir.to_owned()
    }

    pub fn at(&self, t: crate::Float) -> Point3 {
        self.orig + t * self.dir
    }
}

pub fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - 2.0 * i.dot(n) * n
}

pub fn refract(i: Vec3, n: Vec3, index: crate::Float) -> Vec3 {
    let cos_theta = -i.dot(n).min(1.0);
    let r_out_perp = index * (i + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    return r_out_perp + r_out_parallel;
}

/// return (is_front_face, forwarding incident ray normal)
pub fn faceforward(i: Vec3, n: Vec3) -> (bool, Vec3) {
    if i.dot(n) < 0.0 {
        return (true, n);
    } else {
        return (false, -n);
    }
}
