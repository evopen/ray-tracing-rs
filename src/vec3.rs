pub use glam::DVec3 as Vec3;

pub type Point3 = Vec3;

pub fn is_near_zero(v: Vec3) -> bool {
    let epsilon = 1e-8;
    v.cmple(Vec3::splat(epsilon)).all()
}
