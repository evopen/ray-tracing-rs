use std::cell::RefCell;

use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

use crate::Vec3;

thread_local! {
    pub static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(10086));
}

#[inline]
pub fn rand_f64() -> f64 {
    RNG.with(|f| f.borrow_mut().gen_range(0.0..1.0))
}

#[inline]
pub fn rand_f64_range(min: f64, max: f64) -> f64 {
    RNG.with(|f| f.borrow_mut().gen_range(min..max))
}

#[inline]
pub fn rand_vec3() -> Vec3 {
    Vec3::new(
        RNG.with(|f| f.borrow_mut().gen_range(0.0..1.0)),
        RNG.with(|f| f.borrow_mut().gen_range(0.0..1.0)),
        RNG.with(|f| f.borrow_mut().gen_range(0.0..1.0)),
    )
}

#[inline]
pub fn rand_vec3_range(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        RNG.with(|f| f.borrow_mut().gen_range(min..max)),
        RNG.with(|f| f.borrow_mut().gen_range(min..max)),
        RNG.with(|f| f.borrow_mut().gen_range(min..max)),
    )
}

#[inline]
pub fn rand_vec3_in_unit_sphere() -> Vec3 {
    loop {
        let p = rand_vec3_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

#[inline]
pub fn rand_vec3_unit() -> Vec3 {
    rand_vec3_in_unit_sphere().normalize()
}

#[inline]
pub fn rand_vec3_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand_f64(), rand_f64(), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}
