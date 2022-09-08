#[cfg(feature = "f64")]
pub use glam::DVec3 as Vec3;
#[cfg(feature = "f64")]
pub type Float = f64;
#[cfg(feature = "f64")]
pub use std::f64::consts::PI;

#[cfg(feature = "f32")]
pub use glam::Vec3;
#[cfg(feature = "f32")]
pub type Float = f32;
#[cfg(feature = "f32")]
pub use std::f32::consts::PI;

#[cfg(feature = "f32-simd")]
pub use glam::Vec3A as Vec3;
#[cfg(feature = "f32-simd")]
pub type Float = f32;
#[cfg(feature = "f32-simd")]
pub use std::f32::consts::PI;

pub type Point3 = Vec3;

#[inline]
pub fn is_near_zero(v: Vec3) -> bool {
    let epsilon = 1e-8;
    v.abs().cmple(Vec3::splat(epsilon)).all()
}
