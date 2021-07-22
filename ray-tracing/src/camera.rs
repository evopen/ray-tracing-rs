use crate::ray::Ray;
use crate::Point3;
use crate::{utils, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub front: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub lens_radius: crate::Float,
    time_0: crate::Float,
    time_1: crate::Float,
}

impl Camera {
    /// Brief.
    ///
    /// Description.
    ///
    /// * `vfov` - Vertical field of view in degree
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: crate::Float,
        aspect_ratio: crate::Float,
        aperture: crate::Float,
        focus_dist: crate::Float,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan(); // assume focal length is 1.0
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let front = (lookat - lookfrom).normalize();
        let right = front.cross(vup).normalize();
        let up = right.cross(front);

        let origin = lookfrom;

        // focus plane distance from lens is no longer 1.0, scale up accordingly
        let horizontal = focus_dist * viewport_width * right;
        let vertical = focus_dist * viewport_height * up;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 + focus_dist * front;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            front,
            right,
            up,
            lens_radius,
            time_0: 0.0,
            time_1: 0.0,
        }
    }

    pub fn new_with_time_range(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: crate::Float,
        aspect_ratio: crate::Float,
        aperture: crate::Float,
        focus_dist: crate::Float,
        time_0: crate::Float,
        time_1: crate::Float,
    ) -> Self {
        let mut this = Self::new(
            lookfrom,
            lookat,
            vup,
            vfov,
            aspect_ratio,
            aperture,
            focus_dist,
        );
        this.time_0 = time_0;
        this.time_1 = time_1;

        this
    }

    pub fn get_ray(&self, u: crate::Float, v: crate::Float) -> Ray {
        let rd = self.lens_radius * utils::rand_vec3_in_unit_disk();
        let offset = self.right * rd.x + self.up * rd.y;
        let time = if (self.time_0 - self.time_1).abs() < crate::Float::EPSILON {
            0.0
        } else {
            utils::gen_range(self.time_0..self.time_1)
        };
        Ray::new_with_time(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            time,
        )
    }
}
