pub use glam::DVec3 as Vec3;

pub type Point3 = Vec3;

#[inline]
pub fn is_near_zero(v: Vec3) -> bool {
    let epsilon = 1e-8;
    v.abs().cmple(Vec3::splat(epsilon)).all()
}
