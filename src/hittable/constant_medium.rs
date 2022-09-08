use std::sync::Arc;

use crate::color::Color;
use crate::material::{Isotropic, Material};
use crate::texture::Texture;
use crate::utils;
use crate::Vec3;

use super::{HitRecord, Hittable};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: crate::Float,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn Hittable>,
        density: crate::Float,
        texture: Arc<dyn Texture>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(texture)),
        }
    }

    pub fn new_with_color(
        boundary: Arc<dyn Hittable>,
        density: crate::Float,
        color: Color,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_with_color(color)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<super::HitRecord> {
        // get smallest t hit
        let mut rec_1 =
            match self
                .boundary
                .hit(r, crate::Float::NEG_INFINITY, crate::Float::INFINITY)
            {
                Some(rec) => rec,
                None => return None,
            };

        // get second hit point, must be convex
        let mut rec_2 = match self
            .boundary
            .hit(r, rec_1.t + 0.0001, crate::Float::INFINITY)
        {
            Some(rec) => rec,
            None => return None,
        };

        // handle cases where ray origin inside volume
        if rec_1.t < t_min {
            rec_1.t = t_min;
        }
        if rec_2.t > t_max {
            rec_2.t = t_max;
        }

        // hit point is outside of [t_min, t_max] range
        if rec_1.t >= rec_2.t {
            return None;
        }

        if rec_1.t < 0.0 {
            panic!("what is this?");
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec_2.t - rec_1.t) * ray_length;

        // random distance from rec_0.t
        let hit_distance = self.neg_inv_density * utils::gen_float().ln();

        // ray miss volume
        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec_1.t + hit_distance / ray_length;
        let p = r.at(t);
        let rec = HitRecord {
            p,
            normal: Vec3::splat(1.0),
            material: self.phase_function.clone(),
            t,
            u: 0.0,
            v: 0.0,
            front_face: true,
        };

        Some(rec)
    }

    fn bounding_box(&self, time_0: crate::Float, time_1: crate::Float) -> Option<super::Aabb> {
        self.boundary.bounding_box(time_0, time_1).clone()
    }
}
