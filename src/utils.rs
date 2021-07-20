use std::cell::RefCell;

use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

use crate::Vec3;

thread_local! {
    pub static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(4));
}

#[inline]
pub fn gen_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    RNG.with(|f| f.borrow_mut().gen_range(range))
}

#[inline]
pub fn gen_float() -> crate::Float {
    RNG.with(|f| f.borrow_mut().gen_range(0.0..1.0))
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
pub fn rand_vec3_range(min: crate::Float, max: crate::Float) -> Vec3 {
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
        let p = Vec3::new(gen_float(), gen_float(), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}
