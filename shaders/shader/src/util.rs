use spirv_std::glam::Vec3;

// [0, 1] float rng
pub fn rng(state: &mut u32) -> f32 {
    // Condensed version of pcg_output_rxs_m_xs_32_32, with simple conversion to floating-point [0,1].
    *state = *state * 747796405 + 1;
    let state = *state;
    let mut word = ((state >> (state >> 28) + 4) ^ state) * 277803737;
    word = (word >> 22) ^ word;
    return word as f32 / 4294967295.0;
}

pub fn facefoward(n: &Vec3, i: &Vec3) -> Vec3 {
    match n.dot(*i) < 0.0 {
        true => *n,
        false => -*n,
    }
}

#[inline]
pub fn rand_vec3_range(state: &mut u32, min: f32, max: f32) -> Vec3 {
    let scale = max - min;
    let n1 = rng(state) * scale + min;
    let n2 = rng(state) * scale + min;
    let n3 = rng(state) * scale + min;
    Vec3::new(n1, n2, n3)
}

pub fn rand_vec3_in_unit_sphere(state: &mut u32) -> Vec3 {
    loop {
        let p = rand_vec3_range(state, -1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

pub fn rand_vec3_unit(state: &mut u32) -> Vec3 {
    rand_vec3_in_unit_sphere(state).normalize()
}

pub fn is_near_zero(v: Vec3) -> bool {
    let epsilon = 1e-8;
    v.abs().cmple(Vec3::splat(epsilon)).all()
}
