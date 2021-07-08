use std::cell::RefCell;

use rand::prelude::SmallRng;
use rand::{Rng, RngCore, SeedableRng};

thread_local! {
    pub static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(12345));
}

#[inline]
pub fn random_f64() -> f64 {
    RNG.with(|f| f.borrow_mut().gen_range(0.0..1.0))
}

#[inline]
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    RNG.with(|f| f.borrow_mut().gen_range(min..max))
}
