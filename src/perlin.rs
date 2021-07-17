use crate::utils;
use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Default for Perlin {
    fn default() -> Self {
        let mut ranfloat = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranfloat.push(utils::rand_f64());
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Perlin {
    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = Vec::with_capacity(POINT_COUNT);
        for i in 0..POINT_COUNT {
            p.push(i);
        }
        Self::permute(&mut p);

        p
    }

    fn permute(p: &mut Vec<usize>) {
        for i in (1..(POINT_COUNT - 1)).rev() {
            let target = utils::gen_range(0..i);
            (p[i], p[target]) = (p[target], p[i]);
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let max = POINT_COUNT - 1;
        let i = ((4.0 * p.x) as i32 & max as i32) as usize;
        let j = ((4.0 * p.y) as i32 & max as i32) as usize;
        let k = ((4.0 * p.z) as i32 & max as i32) as usize;

        let idx = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];

        return self.ranfloat[idx];
    }
}
