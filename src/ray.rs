use crate::Point3;
use crate::Vec3;

#[derive(Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            orig: origin.to_owned(),
            dir: direction.to_owned(),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig.to_owned()
    }

    pub fn direction(&self) -> Vec3 {
        self.dir.to_owned()
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
